#![allow(dead_code)]
#![allow(unused_imports)]

mod model;
mod device;

use serde_json::{Value, json};
use model::spine::commondatatypes::{DeviceTypeEnumType};
use model::ship::{self};

use std::{
    any::Any,
    sync::{Arc, Mutex},
    time::{Duration},
    net::{TcpStream},
};
use core::cmp;
use chrono::{Utc, Datelike};
use rcgen::{
    self,
    Certificate,
    CertificateParams,
    DistinguishedName,
};
use native_tls::{self};
use tungstenite::{self, Message, WebSocket, stream::MaybeTlsStream};
use httparse::{self};
use zeroconf::{prelude::*, macos::event_loop::BonjourEventLoop};
use zeroconf::{MdnsBrowser, MdnsService, ServiceDiscovery, ServiceRegistration, ServiceType, TxtRecord};

// process incoming json strings and transform it to match the model structure
fn transform_received_json(data: &str) -> String {
    println!("recv: {}", data);

    let mut result: String = str::replace(data, "[{", "{");
    result = str::replace(&result, "},{", ",");
    result = str::replace(&result, "}]", "}");
    result = str::replace(&result, "[]", "{}");

    result
}

// convert objects in json to be arrays with each field being an array alement as eebus expects it
fn process_eebus_json_hierarchie_level(data: &Value) -> Result<Value, String> {
    if data.is_object() {
        match data.as_object() {
            Some(object) => {
                let mut array: Vec<Value> = Vec::new();
                for (key, value) in object.iter() {
                    let new_value = match process_eebus_json_hierarchie_level(value) {
                        Ok(value) => value,
                        Err(err) => return Err(err),
                    };

                    let new_object = json!({
                        key: new_value
                    });
                    let new_object = match serde_json::to_value(new_object) {
                        Ok(value) => value,
                        Err(err) => return Err(err.to_string()),
                    };
                    array.push(new_object);
                }
                let result = match serde_json::to_value(array) {
                    Ok(value) => value,
                    Err(err) => return Err(err.to_string()),
                };
                return Ok(result);
            },
            None => {
                return Ok(data.to_owned());       
            },
        }
    } else if data.is_array() {
        let mut array: Vec<Value> = Vec::new();
        let data_array = match data.as_array() {
            Some(value) => value,
            None => return Err("data is not an array".to_string()),
        };
        for value in data_array.iter() {
            let new_value = match process_eebus_json_hierarchie_level(value) {
                Ok(value) => value,
                Err(err) => return Err(err),
            };
            array.push(new_value);
        }
        let result = match serde_json::to_value(&array) {
            Ok(value) => value,
            Err(err) => return Err(err.to_string()),
        };
        return Ok(result);
    } else {
        return Ok(data.clone());
    }
}

// serialize the model to an eebus expected json format
fn create_eebus_json_string<T>(model: T) -> Result<String, String>
where 
    T: serde::Serialize
{
    let json: String = match serde_json::to_string(&model) {
        Ok(value) => value,
        Err(err) => return Err(err.to_string()),
    };

    let v: Value = match serde_json::from_str(&json) {
        Ok(value) => value,
        Err(err) => return Err(err.to_string()),
    };

    let model = match process_eebus_json_hierarchie_level(&v) {
        Ok(value) => value,
        Err(err) => return Err(err),
    };
    let result = match serde_json::to_string(&model) {
        Ok(value) => value,
        Err(err) => return Err(err.to_string()),
    };

    // we are lazy: fix the first item being put into an array
    let length = result.len()-1;
    let result = result[1..length].to_string();

    Ok(result)
}

// send a json string via ship
fn send_json<T>(ws: &mut WebSocket<tungstenite::stream::MaybeTlsStream<TcpStream>>, msg_type: ship::model::MessageType, model: T) -> Result<(), String>
where 
    T: serde::Serialize
{
    let result = match create_eebus_json_string(&model) {
        Ok(value) => value,
        Err(err) => return Err(err),
    };

    println!("send: {}", result);

    let mut result = result.as_bytes().to_vec();

    let mut msg = vec![msg_type as u8];
    msg.append(&mut result);

    match ws.write_message(Message::Binary(msg)) {
        Ok(_) => Ok(()),
        Err(err) => Err(err.to_string()),
    }
}

#[derive(Default, Debug)]
struct Context {
    service_name: String,
}

struct MdnsLoops<'a> {
    service: BonjourEventLoop<'a>,
    browse: BonjourEventLoop<'a>,
}
unsafe impl Send for MdnsLoops<'_> {}

// register an EEBUS service on mDNS (unused)
fn setup_mdns_service() -> Result<(), String> {
    let service_type = match ServiceType::new("ship", "tcp") {
        Ok(value) => value,
        Err(err) => return Err(err.to_string()),
    };

    let mut service = MdnsService::new(service_type, 4712);
    let mut txt_record = TxtRecord::new();
    let context: Arc<Mutex<Context>> = Arc::default();

    let cem_type = DeviceTypeEnumType::EnergyManagementSystem.to_string();

    let kv: Vec<(&str, &str)> = vec![
        ("txtvers", "1"),
        ("path", "/ship/"),
        ("id", "0"),
        ("ski", "0"),
        ("brand", "WIP"),
        ("model", "WIP"),
        ("type", cem_type.as_str()),
        ("register", "true"),
    ];
    for (key, value) in kv  {
        match txt_record.insert(key, value) {
            Ok(_) => (),
            Err(err) => return Err(err.to_string()),
        }
    }

    service.set_registered_callback(Box::new(mdns_on_service_registered));
    service.set_context(Box::new(context));
    service.set_txt_record(txt_record);

    let event_service_loop = match service.register() {
        Ok(value) => value,
        Err(err) => return Err(err.to_string()),
    };

    match event_service_loop.poll(Duration::from_secs(0)) {
        Ok(_) => (),
        Err(err) => return Err(err.to_string()),
    }

    Ok(())
}

// callback for mDNS when the EEBUS service is registered
fn mdns_on_service_registered(
    result: zeroconf::Result<ServiceRegistration>,
    context: Option<Arc<dyn Any>>,
) {
    let service = result.unwrap();

    println!("Service registered: {:?}", service);

    let context = context
        .as_ref()
        .unwrap()
        .downcast_ref::<Arc<Mutex<Context>>>()
        .unwrap()
        .clone();

    context.lock().unwrap().service_name = service.name().clone();

    println!("Context: {:?}", context);

    // ...
}

// browse for EEBUS services on mDNS
fn browse_mdns() {
    let service_type = match ServiceType::new("ship", "tcp") {
        Ok(value) => value,
        Err(err) => {
            println!("{}", err);
            return;
        },
    };

    let mut browser = MdnsBrowser::new(service_type);

    browser.set_service_discovered_callback(Box::new(mdns_on_service_discovered));

    let event_browse_loop = match browser.browse_services() {
        Ok(value) => value,
        Err(err) => {
            println!("{}", err);
            return;
        },
    };

    println!("Browsing for services...");
    loop {
        match event_browse_loop.poll(Duration::from_secs(0)) {
            Ok(_) => (),
            Err(err) => {
                panic!("{}", err);
            },
        }
    }
}

// callback for mDNS when an EEBUS service is discovered
fn mdns_on_service_discovered(
    result: zeroconf::Result<ServiceDiscovery>,
    _context: Option<Arc<dyn Any>>,
) {
    let service = match result {
        Ok(service) => service,
        Err(err) => {
            println!("Error discovering service: {:?}", err);
            return;
        },
    };

    println!("\n\nService discovered:\n  {:?}", service);

    println!("\nConnecting to {}:{}", service.address(), service.port());
    connect_to_eebus_service(service.address().to_string(), service.port().to_string());
    println!("\n\n");
}

// connect to an EEBUS service
fn connect_to_eebus_service(address: String, port: String) {
    let identity = match get_identity() {
        Ok(cert) => cert,
        Err(e) => {
            println!("Failed to create identity: {}", e);
            return;
        }
    };

    let mut ws = match setup_websocket(identity, address, port) {
        Err(e) => {
            println!("Failed to setup websocket: {}", e);
            return;
        },
        Ok(ws) => ws,
    };

    match ship_handshake(&mut ws) {
        Err(e) => {
            println!("Failed to handshake: {}", e);
            return;
        },
        Ok(_) => {}
    }
}

// get a certificate to be used as identity
fn get_identity() -> Result<native_tls::Identity, String> {
/*
    // TODO: create a certificate instead of using a generated one from a file
    let mut dn = DistinguishedName::new();
    dn.push(rcgen::DnType::OrganizationName, "Demo");
    dn.push(rcgen::DnType::CountryName, "DE");
    dn.push(rcgen::DnType::CommonName, "Demo");
    
    let mut cert_params = CertificateParams::default();
    cert_params.distinguished_name = dn;
    cert_params.serial_number = Option::Some(1);
    // ECDSA is required
    cert_params.alg = &rcgen::PKCS_ECDSA_P256_SHA256;
    cert_params.is_ca = rcgen::IsCa::Ca(rcgen::BasicConstraints::Unconstrained);
    // The SKI is stored in the subject_key_identifier as Sha1 checksum of the private key
    // cert_params.key_identifier_method = ecgen::KeyIdMethod::Sha1;
    cert_params.key_usages = vec![
            rcgen::KeyUsagePurpose::DigitalSignature,
			rcgen::KeyUsagePurpose::KeyEncipherment,
            rcgen::KeyUsagePurpose::KeyCertSign,
			rcgen::KeyUsagePurpose::ContentCommitment,
		];
    cert_params.extended_key_usages = vec![
            rcgen::ExtendedKeyUsagePurpose::ServerAuth,
    ];
    let now = Utc::now();
    cert_params.not_before = rcgen::date_time_ymd(now.year(), now.month() as u8, now.day() as u8);
    cert_params.not_after = rcgen::date_time_ymd(now.year()+50, now.month() as u8, now.day() as u8);

    let certificate = match Certificate::from_params(cert_params) {
        Ok(certificate) => certificate,
        Err(e) => return Err(e.to_string()),
    };

    let cert = match certificate.serialize_pem() {
        Ok(cert) => cert,
        Err(err) => return Err(format!("Error in serializing the cert pem: {}", err)),
    };
    let key = certificate.serialize_private_key_pem();
    
    //  the private key can not be used right: https://github.com/est31/rcgen/issues/82
    match native_tls::Identity::from_pkcs8(cert.as_bytes(), key.as_bytes()) {
        Ok(identity) => return Ok(identity),
        Err(err) => {
            println!("Error in creating identity: {}", err);
            println!("Cert:\n{}", cert);
            println!("Key:\n{}", key);
        },
    };
*/
    let cert = include_bytes!("../keys/demo.crt");
    let key = include_bytes!("../keys/demo.key");

    let identity = match native_tls::Identity::from_pkcs8(cert, key) {
        Ok(identity) => identity,
        Err(err) => return Err(format!("Error in creating identity: {}", err)),
    };
    
    Ok(identity)
}

// setup a secure websocket connection
fn setup_websocket(identity: native_tls::Identity, address: String, port: String) -> Result<WebSocket<MaybeTlsStream<TcpStream>>, String> {
    let connector = match native_tls::TlsConnector::builder()
        .identity(identity)
        .danger_accept_invalid_certs(true)
        .build() {
            Ok(connector) => connector,
            Err(err) => return Err(format!("Error in creating connector: {}", err)),
        };
    let connector: tungstenite::Connector = tungstenite::Connector::NativeTls(connector);

    let stream = match TcpStream::connect(format!("{}:{}", address, port)) {
        Ok(stream) => stream,
        Err(err) => return Err(format!("Error in connecting to the server: {}", err)),
    };

    let websocket_key = tungstenite::handshake::client::generate_key();
    let mut headers = [
        httparse::Header {
            name: "Host",
            value: "localhost".as_bytes(),
        },
        httparse::Header {
            name: "Connection",
            value: "Upgrade".as_bytes(),
        },
        httparse::Header {
            name: "Upgrade",
            value: "websocket".as_bytes(),
        },
        httparse::Header {
            name: "Sec-WebSocket-Version",
            value: "13".as_bytes(),
        },
        httparse::Header {
            name: "Sec-WebSocket-Key",
            value: websocket_key.as_bytes(),
        },
        httparse::Header {
            name: "Sec-WebSocket-Protocol",
            value: "ship".as_bytes(),
        },
    ];

    let mut request: httparse::Request = httparse::Request::new(&mut headers);
    request.method = Some("GET");
    request.version = Some(11);
    request.path = Some("wss://localhost/ship/");
    
    // TODO: the SKI of the peer certificate would need to be checked if the device is paired
    let result = tungstenite::client_tls_with_config(request, stream, None, Some(connector));
    match result {
        Ok((ws, _)) => {Ok(ws)},
        Err(e) => Err(format!("{:?}", e)),
    }
}

// read a websocket connection
fn read_websocket_message(ws: &mut WebSocket<MaybeTlsStream<TcpStream>>) -> Result<Message, String> {
    loop {
        let msg = match ws.read_message() {
            Ok(msg) => msg,
            Err(error) => return Err(error.to_string()),
        };

        if msg.is_empty() {
            continue;
        }

        if msg.is_binary() {
            return Ok(msg)
        }

        return Err(format!("Read ws: invalid response: {}", msg));
    }
}

// get a serializable json string from a websocket message
fn json_from_message(msg: Message) -> Result<String, String> {
    let message = match msg.into_text() {
        Err(e) => return Err(e.to_string()),
        Ok(message) => message,
    };
    let message = &message.as_str()[1..];
    let json = transform_received_json(&message);
    Ok(json)
}

// ship handshake process
fn ship_handshake(ws: &mut WebSocket<MaybeTlsStream<TcpStream>>) -> Result<(), String> {
    // CMI_STATE_CLIENT_SEND
    let ship_init = vec!(ship::model::MessageType::Init as u8, 0);
    let init_message = Message::Binary(ship_init);
    let init_data = init_message.clone().into_data();

    match ws.write_message(init_message) {
        Err(e) => {
            return Err(e.to_string());
        }
        _ => {}
    }
    
    // CMI_STATE_CLIENT_WAIT
    let result = read_websocket_message(ws);
    let msg = match result {
        Err(e) => return Err(e),
        Ok(result) => result,
    };

    // CMI_STATE_CLIENT_EVALUATE
    let response = msg.into_data();
    if response.cmp(&init_data) == cmp::Ordering::Equal {
        println!("Got init message");
    } else {
        return Err("Invalid init message".to_string());
    }

    // SME_HELLO_STATE_READY_INIT
    let mut hello_message = ship::model::ConnectionHello::default();
    hello_message.connection_hello.phase = Some(ship::model::ConnectionHelloPhaseType::Ready);
    hello_message.connection_hello.waiting = Some(60000);

    match send_json(ws, ship::model::MessageType::Control, &hello_message) {
        Err(e) => return Err(e.to_string()),
        Ok(_) => {}
    }

    // SME_HELLO_STATE_READY_LISTEN
    let msg = match read_websocket_message(ws) {
        Err(e) => return Err(e),
        Ok(message) => message,
    };
    let json = match json_from_message(msg) {
        Err(e) => return Err(e),
        Ok(json) => json,
    };

    match serde_json::from_str::<ship::model::ConnectionHello>(&json) {
        Err(e) => return Err(e.to_string()),
        Ok(message) => {
            match message.connection_hello.phase {
                Some(ship::model::ConnectionHelloPhaseType::Ready) => {
                    println!("Got ready message");
                },
                Some(ship::model::ConnectionHelloPhaseType::Aborted) => {
                    return Err("Got aborted message".to_string());
                },
                _ => {
                    return Err("Hello state: Invalid response".to_string());
                }
            }
        }
    }

    // HELLO_OK

    // Protocol Handshake
    let mut protocol_handshake = ship::model::MessageProtocolHandshake{
        message_protocol_handshake: ship::model::MessageProtocolHandshakeType{
            handshake_type: Some(ship::model::ProtocolHandshakeTypeType::AnnounceMax),
            version: ship::model::Version{major: 1, minor: 0},
            formats: ship::model::MessageProtocolFormatsType{format: vec!(ship::model::MessageProtocolFormatType::JsonUTF8)}
        }
    };

    match send_json(ws, ship::model::MessageType::Control, &protocol_handshake) {
        Err(e) => return Err(e.to_string()),
        Ok(_) => {}
    }

    let msg = match read_websocket_message(ws) {
        Err(e) => return Err(e),
        Ok(message) => message,
    };
    let json = match json_from_message(msg) {
        Err(e) => return Err(e),
        Ok(json) => json,
    };

    match serde_json::from_str::<ship::model::MessageProtocolHandshake>(&json) {
        Err(e) => return Err(e.to_string()),
        Ok(message) => {
            if message.message_protocol_handshake.handshake_type != Some(ship::model::ProtocolHandshakeTypeType::Select) {
                // || !message.message_protocol_handshake.formats.format.contains(&ship::model::MessageProtocolFormatType::JsonUTF8) {
                return Err("Invalid protocol handshake response".to_string());
            }
        }
    }

    protocol_handshake.message_protocol_handshake.handshake_type = Some(ship::model::ProtocolHandshakeTypeType::Select);
    match send_json(ws, ship::model::MessageType::Control, &protocol_handshake) {
        Err(e) => return Err(e.to_string()),
        Ok(_) => {}
    }

    println!("Got protocol handshake");

    // PIN State
    let mut pin_state = ship::model::ConnectionPinState::default();
    pin_state.connection_pin_state.pin_state = Some(ship::model::PinStateType::None);
    match send_json(ws, ship::model::MessageType::Control, &pin_state) {
        Err(e) => return Err(e.to_string()),
        Ok(_) => {}
    }

    let msg = match read_websocket_message(ws) {
        Err(e) => return Err(e),
        Ok(message) => message,
    };
    let json = match json_from_message(msg) {
        Err(e) => return Err(e),
        Ok(json) => json,
    };

    let message = match serde_json::from_str::<ship::model::ConnectionPinState>(&json) {
        Err(e) => return Err(e.to_string()),
        Ok(message) => message,
    };
    match message.connection_pin_state.pin_state {
        Some(ship::model::PinStateType::None) => {
            println!("Got pin state: none");
        },
        Some(ship::model::PinStateType::Required) => {
            return Err("Got pin state: required (unsupported)".to_string());
        },
        Some(ship::model::PinStateType::Optional) => {
            return Err("Got pin state: optional (unsupported)".to_string());
        },
        Some(ship::model::PinStateType::PinOk) => {
            return Err("Got pin state: ok (unsupported)".to_string());
        },
        _ => {
            return Err("Got pin state: Invalid response".to_string());
        }
    }

    // Access Methods
    let access_methods_request = ship::model::AccessMethodsRequest::default();
    match send_json(ws, ship::model::MessageType::Control, &access_methods_request) {
        Err(e) => return Err(e.to_string()),
        Ok(_) => {}
    }

    loop {
        let msg = match read_websocket_message(ws) {
            Err(e) => return Err(e),
            Ok(message) => message,
        };
        let json = match json_from_message(msg) {
            Err(e) => return Err(e),
            Ok(json) => json,
        };

        if json.contains("\"accessMethodsRequest\":") {
            println!("Got access methods request");
            let mut access_methods = ship::model::AccessMethods::default();
            access_methods.access_methods.id = "test".to_string();
            match send_json(ws, ship::model::MessageType::Control, &access_methods) {
                Err(e) => return Err(e.to_string()),
                Ok(_) => {}
            };
        } else if json.contains("\"accessMethods\":") {
            println!("Got access methods");
            break;
        } else {
            return Err("access methods: Invalid response".to_string());
        }
    }

    // Spine
    let msg = match read_websocket_message(ws) {
        Err(e) => return Err(e),
        Ok(message) => message,
    };
    let _json = match json_from_message(msg) {
        Err(e) => return Err(e),
        Ok(json) => json,
    };
    println!("Got SHIP message with SPINE payload");

    match ws.close(None) {
        Err(e) => return Err(e.to_string()),
        Ok(_) => {}
    }

    Ok(())
}

fn main() {
    browse_mdns();
}


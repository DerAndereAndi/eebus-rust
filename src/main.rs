#![allow(dead_code)]

mod model;
mod device;

use serde_json::{Value, json};
use model::spine::{self, commondatatypes::{DeviceTypeEnumType}};
use model::ship::{self};

use std::{
    error::Error,
    any::Any,
    sync::{Arc, Mutex},
    time::{Duration, SystemTime},
    net::{TcpStream},
};
use core::cmp;
use rcgen::{
    self,
    Certificate,
    CertificateParams,
    DistinguishedName, RcgenError,
};
use native_tls::{self};
use tungstenite::{self, Message, WebSocket, stream::MaybeTlsStream};
use httparse::{self};
use zeroconf::prelude::*;
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

fn setup_mdns() {
    let mut browser = MdnsBrowser::new(ServiceType::new("ship", "tcp").unwrap());

    browser.set_service_discovered_callback(Box::new(mdns_on_service_discovered));

    let mut service = MdnsService::new(ServiceType::new("ship", "tcp").unwrap(), 4712);
    let mut txt_record = TxtRecord::new();
    let context: Arc<Mutex<Context>> = Arc::default();

    txt_record.insert("txtvers", "1").unwrap();
    txt_record.insert("path", "/ship/").unwrap();
    txt_record.insert("id", "0").unwrap();
    txt_record.insert("ski", "0").unwrap();
    txt_record.insert("brand", "WIP").unwrap();
    txt_record.insert("model", "WIP").unwrap();
    txt_record.insert("type", &DeviceTypeEnumType::EnergyManagementSystem.to_string()).unwrap();
    txt_record.insert("register", "true").unwrap();

    service.set_registered_callback(Box::new(mdns_on_service_registered));
    service.set_context(Box::new(context));
    service.set_txt_record(txt_record);

    let event_service_loop = service.register().unwrap();
    let event_browse_loop = browser.browse_services().unwrap();

    loop {
        // calling `poll()` will keep this alive
        event_browse_loop.poll(Duration::from_secs(0)).unwrap();
        event_service_loop.poll(Duration::from_secs(0)).unwrap();
    }
}

fn mdns_on_service_discovered(
    result: zeroconf::Result<ServiceDiscovery>,
    _context: Option<Arc<dyn Any>>,
) {
    println!("Service discovered: {:?}", result.unwrap());

    // ...
}

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

fn create_certificate() -> Result<Certificate, RcgenError> {
    let subject_alt_names = vec!["localhost".to_string()];

    let mut dn = DistinguishedName::new();
    dn.push(rcgen::DnType::OrganizationName, "WIP");
    dn.push(rcgen::DnType::CountryName, "DE");
    dn.push(rcgen::DnType::CommonName, rcgen::DnValue::PrintableString("WIP".to_string()));
    
    let mut cert_params = CertificateParams::new(subject_alt_names);
    // cert_params.key_usages = vec![
    //         rcgen::KeyUsagePurpose::DigitalSignature,
	// 		rcgen::KeyUsagePurpose::KeyEncipherment,
    //         rcgen::KeyUsagePurpose::KeyCertSign,
	// 		rcgen::KeyUsagePurpose::ContentCommitment,
	// 	];
    cert_params.distinguished_name = dn;
    cert_params.serial_number = Option::Some(1);
    cert_params.alg = &rcgen::PKCS_ECDSA_P256_SHA256;
    cert_params.is_ca = rcgen::IsCa::Ca(rcgen::BasicConstraints::Constrained(0));

    Certificate::from_params(cert_params)
}

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

        return Err("Invalid response".to_string());
    }
}

static KEY: &[u8] = include_bytes!("../keys/evcc.key");    
static CERT: &[u8] = include_bytes!("../keys/evcc.crt");

fn setup_websocket(_cert: Certificate) -> Result<WebSocket<MaybeTlsStream<TcpStream>>, String> {
    // let pem = match cert.serialize_pem() {
    //     Ok(pem) => pem,
    //     Err(err) => return Err(format!("Error in serializing the cert pem: {}", err)),
    // };
    // let key = cert.serialize_private_key_pem();
    // let identity = match native_tls::Identity::from_pkcs8(&pem.as_bytes(), &key.as_bytes()) {
    //     Ok(identity) => identity,
    //     Err(err) => return Err(format!("Error in creating identity: {}", err)),
    // };

    let identity = match native_tls::Identity::from_pkcs8(CERT, KEY) {
        Ok(identity) => identity,
        Err(err) => return Err(format!("Error in creating identity: {}", err)),
    };
    let connector = match native_tls::TlsConnector::builder()
        .identity(identity)
        .danger_accept_invalid_certs(true)
        .build() {
            Ok(connector) => connector,
            Err(err) => return Err(format!("Error in creating connector: {}", err)),
        };
    let connector: tungstenite::Connector = tungstenite::Connector::NativeTls(connector);

    let stream = match TcpStream::connect("localhost:4712") {
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
    
    let result = tungstenite::client_tls_with_config(request, stream, None, Some(connector));
    match result {
        Ok((ws, _)) => Ok(ws),
        Err(e) => Err(format!("{:?}", e)),
    }
}

fn json_from_message(msg: Message) -> Result<String, String> {
    let message = match msg.into_text() {
        Err(e) => return Err(e.to_string()),
        Ok(message) => message,
    };
    let message = &message.as_str()[1..];
    let json = transform_received_json(&message);
    Ok(json)
}

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
                    return Err("Invalid response".to_string());
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
            println!("Got pin state none");
        },
        Some(ship::model::PinStateType::Required) => {
            panic!("Got pin state required");
        },
        Some(ship::model::PinStateType::Optional) => {
            panic!("Got pin state optional");
        },
        Some(ship::model::PinStateType::PinOk) => {
            panic!("Got pin state pin ok");
        },
        _ => {
            panic!("Invalid response");
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
            panic!("Invalid response");
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
    // setup_mdns();

    let cert = match create_certificate() {
        Ok(cert) => cert,
        Err(e) => {
            println!("Failed to create certificate: {}", e);
            return;
        }
    };

    let mut ws = match setup_websocket(cert) {
        Err(e) => panic!("{}", e),
        Ok(ws) => ws,
    };

    match ship_handshake(&mut ws) {
        Err(e) => panic!("{}", e),
        Ok(_) => {}
    }
}


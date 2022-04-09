#![allow(dead_code)]

mod model;
mod device;

use serde_json::{Value, json};
use model::spine::{self, commondatatypes::{DeviceTypeEnumType}};
use model::ship::{self};

use std::{
    any::Any,
    sync::{Arc, Mutex},
    time::{Duration, SystemTime},
    net::{TcpListener, TcpStream},
    // thread::spawn,
};
use core::cmp;
use rcgen::{
    self,
    Certificate,
    CertificateParams,
    DistinguishedName,
};
use native_tls::{self};
use tungstenite::{self, accept, handshake::HandshakeRole, Error, HandshakeError, Message, Result, WebSocket};
use httparse::{self};
use zeroconf::prelude::*;
use zeroconf::{MdnsBrowser, MdnsService, ServiceDiscovery, ServiceRegistration, ServiceType, TxtRecord};

// process incoming json strings and transform it to match the model structure
fn transform_received_json(data: &str) -> String {
    let mut result: String = str::replace(data, "[{", "{");
    result = str::replace(&result, "},{", ",");
    result = str::replace(&result, "}]", "}");
    result = str::replace(&result, "[]", "{}");

    result
}

// convert objects in json to be arrays with each field being an array alement as eebus expects it
fn process_eebus_json_hierarchie_level(data: &Value) -> Value {
    if data.is_object() {
        match data.as_object() {
            Some(object) => {
                let mut array: Vec<Value> = Vec::new();
                for (key, value) in object.iter() {
                    let new_value = process_eebus_json_hierarchie_level(value);

                    let new_object = json!({
                        key: new_value
                    });
                    let new_object = serde_json::to_value(new_object).unwrap();
                    array.push(new_object);
                }
                let result = serde_json::to_value(array).unwrap();
                return result;
            },
            None => {
                return data.to_owned();       
            },
        }
    } else if data.is_array() {
        let mut array: Vec<Value> = Vec::new();
        for value in data.as_array().unwrap().iter() {
            let new_value = process_eebus_json_hierarchie_level(value);
            array.push(new_value);
        }
        let result = serde_json::to_value(&array).unwrap();
        return result;
    } else {
        return data.clone();
    }
}

// serialize the model to an eebus expected json format
fn create_eebus_json_string(model: spine::datagram::SpineType) -> String {
    let json: String = serde_json::to_string(&model).unwrap();

    let v: Value = serde_json::from_str(&json).unwrap();

    let model = process_eebus_json_hierarchie_level(&v);
    let result = serde_json::to_string(&model).unwrap();

    // we are lazy: fix the first item being put into an array
    let length = result.len()-1;
    let result = result[1..length].to_string();

    result
}


fn send_json<T>(ws: &mut WebSocket<tungstenite::stream::MaybeTlsStream<TcpStream>>, msg_type: ship::model::MessageType, model: T) -> Result<()>
where 
    T: serde::Serialize
{
    let json: String = serde_json::to_string(&model).unwrap();

    let v: Value = serde_json::from_str(&json).unwrap();

    let model = process_eebus_json_hierarchie_level(&v);
    let result = serde_json::to_string(&model).unwrap();

    // we are lazy: fix the first item being put into an array
    let length = result.len()-1;
    let result = result[1..length].to_string();

    println!("send: {}", result);

    let mut result = result.as_bytes().to_vec();

    let mut msg = vec![msg_type as u8];
    msg.append(&mut result);

    ws.write_message(Message::Binary(msg))
}

fn test_de_serializing() {
    // let json = r#"{"datagram":[{"header":{}},{"payload":{}}]}"#;
    // let json = r#"{"datagram":[{"header":[{"specificationVersion":"1.2.0"},{"addressSource":[{"device":"d:_i:3210_EVSE"},{"entity":[1,1]},{"feature":6}]},{"addressDestination":[{"device":"d:_i:3210_HEMS"},{"entity":[1]},{"feature":1}]},{"msgCounter":194},{"msgCounterReference":4890},{"cmdClassifier":"reply"}]},{"payload":[{"cmd":[[{"deviceClassificationManufacturerData":[{"deviceName":""},{"deviceCode":""},{"brandName":""},{"powerSource":"mains3Phase"}]}]]}]}]}"#;
    // let json = r#"{"datagram":[{"header":[{"specificationVersion":"1.1.1"},{"addressSource":[{"device":"d:_i:3210_EVSE"},{"entity":[0]},{"feature":0}]},{"addressDestination":[{"device":"d:_i:3210_HEMS"},{"entity":[0]},{"feature":0}]},{"msgCounter":116},{"cmdClassifier":"notify"}]},{"payload":[{"cmd":[[{"function":"nodeManagementDetailedDiscoveryData"},{"filter":[[{"cmdControl":[{"partial":[]}]}]]},{"nodeManagementDetailedDiscoveryData":[{"deviceInformation":[{"description":[{"deviceAddress":[{"device":"d:_i:3210_EVSE"}]}]}]},{"entityInformation":[[{"description":[{"entityAddress":[{"entity":[1,1]}]},{"entityType":"EV"},{"lastStateChange":"added"},{"description":"Electric Vehicle"}]}]]},{"featureInformation":[[{"description":[{"featureAddress":[{"entity":[1,1]},{"feature":1}]},{"featureType":"LoadControl"},{"role":"server"},{"supportedFunction":[[{"function":"loadControlLimitDescriptionListData"},{"possibleOperations":[{"read":[]}]}],[{"function":"loadControlLimitListData"},{"possibleOperations":[{"read":[]},{"write":[]}]}]]},{"description":"Load Control"}]}],[{"description":[{"featureAddress":[{"entity":[1,1]},{"feature":2}]},{"featureType":"ElectricalConnection"},{"role":"server"},{"supportedFunction":[[{"function":"electricalConnectionParameterDescriptionListData"},{"possibleOperations":[{"read":[]}]}],[{"function":"electricalConnectionDescriptionListData"},{"possibleOperations":[{"read":[]}]}],[{"function":"electricalConnectionPermittedValueSetListData"},{"possibleOperations":[{"read":[]}]}]]},{"description":"Electrical Connection"}]}],[{"description":[{"featureAddress":[{"entity":[1,1]},{"feature":3}]},{"featureType":"Measurement"},{"specificUsage":["Electrical"]},{"role":"server"},{"supportedFunction":[[{"function":"measurementListData"},{"possibleOperations":[{"read":[]}]}],[{"function":"measurementDescriptionListData"},{"possibleOperations":[{"read":[]}]}]]},{"description":"Measurements"}]}],[{"description":[{"featureAddress":[{"entity":[1,1]},{"feature":5}]},{"featureType":"DeviceConfiguration"},{"role":"server"},{"supportedFunction":[[{"function":"deviceConfigurationKeyValueDescriptionListData"},{"possibleOperations":[{"read":[]}]}],[{"function":"deviceConfigurationKeyValueListData"},{"possibleOperations":[{"read":[]}]}]]},{"description":"Device Configuration EV"}]}],[{"description":[{"featureAddress":[{"entity":[1,1]},{"feature":6}]},{"featureType":"DeviceClassification"},{"role":"server"},{"supportedFunction":[[{"function":"deviceClassificationManufacturerData"},{"possibleOperations":[{"read":[]}]}]]},{"description":"Device Classification for EV"}]}],[{"description":[{"featureAddress":[{"entity":[1,1]},{"feature":7}]},{"featureType":"TimeSeries"},{"role":"server"},{"supportedFunction":[[{"function":"timeSeriesConstraintsListData"},{"possibleOperations":[{"read":[]}]}],[{"function":"timeSeriesDescriptionListData"},{"possibleOperations":[{"read":[]}]}],[{"function":"timeSeriesListData"},{"possibleOperations":[{"read":[]},{"write":[]}]}]]},{"description":"Time Series"}]}],[{"description":[{"featureAddress":[{"entity":[1,1]},{"feature":8}]},{"featureType":"IncentiveTable"},{"role":"server"},{"supportedFunction":[[{"function":"incentiveTableConstraintsData"},{"possibleOperations":[{"read":[]}]}],[{"function":"incentiveTableData"},{"possibleOperations":[{"read":[]},{"write":[]}]}],[{"function":"incentiveTableDescriptionData"},{"possibleOperations":[{"read":[]},{"write":[]}]}]]},{"description":"Incentive Table"}]}],[{"description":[{"featureAddress":[{"entity":[1,1]},{"feature":9}]},{"featureType":"DeviceDiagnosis"},{"role":"server"},{"supportedFunction":[[{"function":"deviceDiagnosisStateData"},{"possibleOperations":[{"read":[]}]}]]},{"description":"Device Diagnosis EV"}]}],[{"description":[{"featureAddress":[{"entity":[1,1]},{"feature":10}]},{"featureType":"Identification"},{"role":"server"},{"supportedFunction":[[{"function":"identificationListData"},{"possibleOperations":[{"read":[]}]}]]},{"description":"Identification for EV"}]}]]}]}]]}]}]}"#;
    // let json = r#"{"datagram":[{"header":[{"specificationVersion":"1.2.0"},{"addressSource":[{"device":"d:_i:3210_HEMS"},{"entity":[1]},{"feature":7}]},{"addressDestination":[{"device":"d:_i:3210_EVSE"},{"entity":[1,1]},{"feature":1}]},{"msgCounter":5014},{"cmdClassifier":"write"},{"ackRequest":true}]},{"payload":[{"cmd":[[{"loadControlLimitListData":[{"loadControlLimitData":[[{"limitId":1},{"isLimitActive":true},{"value":[{"number":0},{"scale":0}]}],[{"limitId":2},{"isLimitActive":true},{"value":[{"number":0},{"scale":0}]}]]}]}]]}]}]}"#;
    // let json = r#"{"datagram":[{"header":[{"specificationVersion":"1.2.0"},{"addressSource":[{"device":"d:_i:3210_EVSE"},{"entity":[1,1]},{"feature":2}]},{"addressDestination":[{"device":"d:_i:3210_HEMS"},{"entity":[1]},{"feature":9}]},{"msgCounter":253},{"msgCounterReference":4961},{"cmdClassifier":"reply"}]},{"payload":[{"cmd":[[{"electricalConnectionPermittedValueSetListData":[{"electricalConnectionPermittedValueSetData":[[{"electricalConnectionId":0},{"parameterId":1},{"permittedValueSet":[[{"value":[[{"number":0},{"scale":0}]]},{"range":[[{"min":[{"number":6},{"scale":0}]},{"max":[{"number":10},{"scale":0}]}]]}]]}],[{"electricalConnectionId":0},{"parameterId":8},{"permittedValueSet":[[{"value":[[{"number":0},{"scale":0}]]},{"range":[[{"min":[{"number":1384200},{"scale":-3}]},{"max":[{"number":2307},{"scale":0}]}]]}]]}]]}]}]]}]}]}"#;
    let json = r#"{"datagram":[{"header":[{"specificationVersion":"1.1.1"},{"addressSource":[{"device":"d:_i:3210_EVSE"},{"entity":[1,1]},{"feature":3}]},{"addressDestination":[{"device":"d:_i:3210_HEMS"},{"entity":[1]},{"feature":3}]},{"msgCounter":239},{"cmdClassifier":"notify"}]},{"payload":[{"cmd":[[{"measurementListData":[{"measurementData":[[{"measurementId":1},{"valueType":"value"},{"timestamp":"2021-02-17T20:19:06.188Z"},{"value":[{"number":0},{"scale":0}]},{"valueSource":"measuredValue"}],[{"measurementId":4},{"valueType":"value"},{"timestamp":"2021-02-17T20:19:06.188Z"},{"value":[{"number":0},{"scale":0}]},{"valueSource":"measuredValue"}],[{"measurementId":7},{"valueType":"value"},{"timestamp":"2021-02-17T20:19:06.188Z"},{"value":[{"number":0},{"scale":0}]},{"valueSource":"measuredValue"}]]}]}]]}]}]}"#;
    // let json = r#"{"datagram":[{"header":[{"specificationVersion":"1.1.1"},{"addressSource":[{"device":"d:_i:3210_EVSE"},{"entity":[0]},{"feature":0}]},{"addressDestination":[{"device":"d:_i:3210_HEMS"},{"entity":[0]},{"feature":0}]},{"msgCounter":116},{"cmdClassifier":"notify"}]},{"payload":[{"cmd":[[{"function":"nodeManagementDetailedDiscoveryData"},{"filter":[[{"cmdControl":[{"partial":[]}]}]]},{"nodeManagementDetailedDiscoveryData":[{"deviceInformation":[{"description":[{"deviceAddress":[{"device":"d:_i:3210_EVSE"}]}]}]},{"entityInformation":[[{"description":[{"entityAddress":[{"entity":[1,1]}]},{"entityType":"EV"},{"lastStateChange":"added"},{"description":"Electric Vehicle"}]}]]},{"featureInformation":[[{"description":[{"featureAddress":[{"entity":[1,1]},{"feature":1}]},{"featureType":"LoadControl"},{"role":"server"},{"supportedFunction":[[{"function":"loadControlLimitDescriptionListData"},{"possibleOperations":[{"read":[]}]}],[{"function":"loadControlLimitListData"},{"possibleOperations":[{"read":[]},{"write":[]}]}]]},{"description":"Load Control"}]}],[{"description":[{"featureAddress":[{"entity":[1,1]},{"feature":2}]},{"featureType":"ElectricalConnection"},{"role":"server"},{"supportedFunction":[[{"function":"electricalConnectionParameterDescriptionListData"},{"possibleOperations":[{"read":[]}]}],[{"function":"electricalConnectionDescriptionListData"},{"possibleOperations":[{"read":[]}]}],[{"function":"electricalConnectionPermittedValueSetListData"},{"possibleOperations":[{"read":[]}]}]]},{"description":"Electrical Connection"}]}],[{"description":[{"featureAddress":[{"entity":[1,1]},{"feature":3}]},{"featureType":"Measurement"},{"specificUsage":["Electrical"]},{"role":"server"},{"supportedFunction":[[{"function":"measurementListData"},{"possibleOperations":[{"read":[]}]}],[{"function":"measurementDescriptionListData"},{"possibleOperations":[{"read":[]}]}]]},{"description":"Measurements"}]}],[{"description":[{"featureAddress":[{"entity":[1,1]},{"feature":5}]},{"featureType":"DeviceConfiguration"},{"role":"server"},{"supportedFunction":[[{"function":"deviceConfigurationKeyValueDescriptionListData"},{"possibleOperations":[{"read":[]}]}],[{"function":"deviceConfigurationKeyValueListData"},{"possibleOperations":[{"read":[]}]}]]},{"description":"Device Configuration EV"}]}],[{"description":[{"featureAddress":[{"entity":[1,1]},{"feature":6}]},{"featureType":"DeviceClassification"},{"role":"server"},{"supportedFunction":[[{"function":"deviceClassificationManufacturerData"},{"possibleOperations":[{"read":[]}]}]]},{"description":"Device Classification for EV"}]}],[{"description":[{"featureAddress":[{"entity":[1,1]},{"feature":7}]},{"featureType":"TimeSeries"},{"role":"server"},{"supportedFunction":[[{"function":"timeSeriesConstraintsListData"},{"possibleOperations":[{"read":[]}]}],[{"function":"timeSeriesDescriptionListData"},{"possibleOperations":[{"read":[]}]}],[{"function":"timeSeriesListData"},{"possibleOperations":[{"read":[]},{"write":[]}]}]]},{"description":"Time Series"}]}],[{"description":[{"featureAddress":[{"entity":[1,1]},{"feature":8}]},{"featureType":"IncentiveTable"},{"role":"server"},{"supportedFunction":[[{"function":"incentiveTableConstraintsData"},{"possibleOperations":[{"read":[]}]}],[{"function":"incentiveTableData"},{"possibleOperations":[{"read":[]},{"write":[]}]}],[{"function":"incentiveTableDescriptionData"},{"possibleOperations":[{"read":[]},{"write":[]}]}]]},{"description":"Incentive Table"}]}],[{"description":[{"featureAddress":[{"entity":[1,1]},{"feature":9}]},{"featureType":"DeviceDiagnosis"},{"role":"server"},{"supportedFunction":[[{"function":"deviceDiagnosisStateData"},{"possibleOperations":[{"read":[]}]}]]},{"description":"Device Diagnosis EV"}]}],[{"description":[{"featureAddress":[{"entity":[1,1]},{"feature":10}]},{"featureType":"Identification"},{"role":"server"},{"supportedFunction":[[{"function":"identificationListData"},{"possibleOperations":[{"read":[]}]}]]},{"description":"Identification for EV"}]}]]}]}]]}]}]}"#;

    let read = transform_received_json(json);

    let model: spine::datagram::SpineType = serde_json::from_str(&read).unwrap();
    
    let result: String = create_eebus_json_string(model);

    println!("result identical to original json? {:?}", result.eq(json));
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

fn create_certificate() {
    let subject_alt_names = vec!["hello.world.example".to_string(),"localhost".to_string()];

    let mut dn = DistinguishedName::new();
    dn.push(rcgen::DnType::OrganizationName, "WIP");
    dn.push(rcgen::DnType::CountryName, "DE");
    dn.push(rcgen::DnType::CommonName, rcgen::DnValue::PrintableString("WIP".to_string()));
    
    let cert_params = CertificateParams::new(subject_alt_names);
    // cert_params.key_usages = vec![
    //         rcgen::KeyUsagePurpose::DigitalSignature,
	// 		rcgen::KeyUsagePurpose::KeyEncipherment,
    //         rcgen::KeyUsagePurpose::KeyCertSign,
	// 		rcgen::KeyUsagePurpose::ContentCommitment,
	// 	];
    // cert_params.distinguished_name = dn;
    // cert_params.serial_number = Option::Some(1);
    // cert_params.alg = &rcgen::PKCS_ECDSA_P256_SHA256;
    // cert_params.is_ca = rcgen::IsCa::Ca(rcgen::BasicConstraints::Constrained(0));

    let cert = Certificate::from_params(cert_params).unwrap();
    
    // let cert = generate_simple_self_signed(subject_alt_names).unwrap();
    // The certificate is now valid for localhost and the domain "hello.world.example"
    println!("{}", cert.serialize_pem().unwrap());
    println!("{}", cert.serialize_private_key_pem());
}

fn must_not_block<Role: HandshakeRole>(err: HandshakeError<Role>) -> Error {
    match err {
        HandshakeError::Interrupted(_) => panic!("Bug: blocking socket would block"),
        HandshakeError::Failure(f) => f,
    }
}

fn handle_client(stream: TcpStream) -> Result<()> {
    let mut socket = accept(stream).map_err(must_not_block)?;
    println!("Running test");
    loop {
        match socket.read_message()? {
            msg @ Message::Text(_) | msg @ Message::Binary(_) => {
                socket.write_message(msg)?;
            }
            Message::Ping(_) | Message::Pong(_) | Message::Close(_) | Message::Frame(_) => {}
        }
    }
}

static KEY: &[u8] = include_bytes!("../keys/prod.key");    
static CERT: &[u8] = include_bytes!("../keys/prod.crt");

fn setup_websocket() {
    // print!("{}", String::from_utf8_lossy(KEY));

    let identity = native_tls::Identity::from_pkcs8(CERT, KEY).unwrap();
    let connector = native_tls::TlsConnector::builder()
        .identity(identity)
        .danger_accept_invalid_certs(true)
        .build()
        .unwrap();
    let connector: tungstenite::Connector = tungstenite::Connector::NativeTls(connector);

    let stream = TcpStream::connect("localhost:4712").unwrap();

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
    
    let (mut ws, response) = tungstenite::client_tls_with_config(request, stream, None, Some(connector)).unwrap();

    println!("{:?}", response);

    // CMI_STATE_CLIENT_SEND
    let ship_init = vec!(ship::model::MessageType::Init as u8, 0);
    let init_message = Message::Binary(ship_init);
    let init_data = init_message.clone().into_data();

    ws.write_message(init_message).unwrap();
    
    // CMI_STATE_CLIENT_WAIT
    loop {
        let msg = ws.read_message().unwrap();

        if !msg.is_empty() {
            println!("{:?}", msg);
            // CMI_STATE_CLIENT_EVALUATE
            if msg.is_binary() {
                let response = msg.into_data();
                if response.cmp(&init_data) == cmp::Ordering::Equal {
                    println!("Got init message");
                    break;
                }
            } else {
                println!("Invalid response");
                break;   
            }
        }
    }

    // SME_HELLO_STATE_READY_INIT
    let mut hello_message = ship::model::ConnectionHello::default();
    hello_message.connection_hello.phase = Some(ship::model::ConnectionHelloPhaseType::Ready);
    hello_message.connection_hello.waiting = Some(60000);

    send_json(&mut ws, ship::model::MessageType::Control, &hello_message).unwrap();

    // SME_HELLO_STATE_READY_LISTEN
    loop {
        let msg = ws.read_message().unwrap();

        if !msg.is_empty() {
            if msg.is_binary() {
                let message = msg.into_text().unwrap();
                let message = &message.as_str()[1..];
                let json = transform_received_json(&message);
                println!("recv: {}", json);

                let message = serde_json::from_str::<ship::model::ConnectionHello>(&json).unwrap();
                match message.connection_hello.phase {
                    Some(ship::model::ConnectionHelloPhaseType::Ready) => {
                        println!("Got ready message");
                        break;
                    },
                    Some(ship::model::ConnectionHelloPhaseType::Aborted) => {
                        println!("Got aborted message");
                        break;
                    },
                    _ => {
                        println!("Invalid response");
                        break;
                    }
                }
            } else {
                println!("Invalid response");
                break;   
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

    send_json(&mut ws, ship::model::MessageType::Control, &protocol_handshake).unwrap();
    loop {
        let msg = ws.read_message().unwrap();

        if !msg.is_empty() {
            if msg.is_binary() {
                let message = msg.into_text().unwrap();
                let message = &message.as_str()[1..];
                let json = transform_received_json(&message);
                println!("recv: {}", json);

                let message = serde_json::from_str::<ship::model::MessageProtocolHandshake>(&json).unwrap();
                if message.message_protocol_handshake.handshake_type != Some(ship::model::ProtocolHandshakeTypeType::Select) {
                    // || !message.message_protocol_handshake.formats.format.contains(&ship::model::MessageProtocolFormatType::JsonUTF8) {
                    println!("Invalid protocol handshake response");
                    break;
                }

                protocol_handshake.message_protocol_handshake.handshake_type = Some(ship::model::ProtocolHandshakeTypeType::Select);
                send_json(&mut ws, ship::model::MessageType::Control, &protocol_handshake).unwrap();

                println!("Got protocol handshake");
                break;
            } else {
                println!("Invalid response");
                break;   
            }
        }
    }

    // PIN State
    let mut pin_state = ship::model::ConnectionPinState::default();
    pin_state.connection_pin_state.pin_state = Some(ship::model::PinStateType::None);
    send_json(&mut ws, ship::model::MessageType::Control, &pin_state).unwrap();
    loop {
        let msg = ws.read_message().unwrap();

        if !msg.is_empty() {
            if msg.is_binary() {
                let message = msg.into_text().unwrap();
                let message = &message.as_str()[1..];
                let json = transform_received_json(&message);
                println!("recv: {}", json);

                let message = serde_json::from_str::<ship::model::ConnectionPinState>(&json).unwrap();
                match message.connection_pin_state.pin_state {
                    Some(ship::model::PinStateType::None) => {
                        println!("Got pin state none");
                        break;
                    },
                    Some(ship::model::PinStateType::Required) => {
                        println!("Got pin state required");
                        break;
                    },
                    Some(ship::model::PinStateType::Optional) => {
                        println!("Got pin state optional");
                        break;
                    },
                    Some(ship::model::PinStateType::PinOk) => {
                        println!("Got pin state pin ok");
                        break;
                    },
                    _ => {
                        println!("Invalid response");
                        break;
                    }
                }
            } else {
                println!("Invalid response");
                break;   
            }
        }
    }

    // Access Methods
    let access_methods_request = ship::model::AccessMethodsRequest::default();
    send_json(&mut ws, ship::model::MessageType::Control, &access_methods_request).unwrap();
    loop {
        let msg = ws.read_message().unwrap();

        if !msg.is_empty() {
            if msg.is_binary() {
                let message = msg.into_text().unwrap();
                let message = &message.as_str()[1..];
                let json = transform_received_json(&message);
                println!("recv: {}", json);

                if json.contains("\"accessMethods\":") {
                    break;
                } else if json.contains("\"accessMethodsRequest\":") {
                    let mut access_methods = ship::model::AccessMethods::default();
                    access_methods.access_methods.id = "test".to_string();
                    send_json(&mut ws, ship::model::MessageType::Control, &access_methods).unwrap();
                }

            } else {
                println!("Invalid response");
                break;   
            }
        }
    }

    loop {
        let msg = ws.read_message().unwrap();

        if !msg.is_empty() {
            if msg.is_binary() {
                let message = msg.into_text().unwrap();
                let message = &message.as_str()[1..];
                let json = transform_received_json(&message);
                println!("recv: {}", json);
            } else {
                println!("Invalid response");
                break;   
            }
        }
    }

    ws.close(None).unwrap();

    // loop {
    //     println!("test");
    // }

    // let connector: Arc<rustls::ClientConfig> = Arc::new(config);

    // let identity = native_tls::Identity::from_pkcs8(CERT, KEY).unwrap();

    // let connector = TlsConnector::builder()
    //     .danger_accept_invalid_certs(true)
    //     .identity(identity)
    //     .build()
    //     .unwrap();

    // let (mut socket, response) = tungstenite::connect(
    //     Url::parse("wss://192.168.1.59:4712/").unwrap()
    // ).expect("Can't connect");

    // loop {
    //     let msg = socket.read_message().expect("Error reading message");
    //     println!("Received: {}", msg);
    // }

    // let mut tls_config: rustls::ServerConfig = rustls::ServerConfig::new(rustls::NoClientAuth::new());    
    // tls_config.set_single_cert(    
    //     rustls::internal::pemfile::certs(&mut &*CERT).unwrap(),    
    //     rustls::internal::pemfile::pkcs8_private_keys(&mut &*KEY).unwrap()    
    //         .pop().unwrap()    
    // ).unwrap();    
    // let tls_config = Arc::new(tls_config);

    // let server = TcpListener::bind("192.168.1.59:4712").unwrap();

    // for stream in server.incoming() {
    //     spawn(move || match stream {
    //         Ok(stream) => {
    //             if let Err(err) = handle_client(stream) {
    //                 match err {
    //                     Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8 => (),
    //                     e => println!("test: {}", e),
    //                 }
    //             }
    //         }
    //         Err(e) => println!("Error accepting stream: {}", e),
    //     });
    // }
}

fn main() {
    // test_de_serializing();

    // setup_mdns();

    // create_certificate();

    setup_websocket();
}


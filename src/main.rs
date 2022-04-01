mod model;
mod device;

use serde_json::{Value, json};
use model::spine;

use std::any::Any;
use std::sync::{Arc, Mutex};
use std::time::Duration;
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
pub struct Context {
    service_name: String,
}

fn main() {
    // test_de_serializing();

    let mut browser = MdnsBrowser::new(ServiceType::new("ship", "tcp").unwrap());

    browser.set_service_discovered_callback(Box::new(on_service_discovered));

    let mut service = MdnsService::new(ServiceType::new("ship", "tcp").unwrap(), 4712);
    let mut txt_record = TxtRecord::new();
    let context: Arc<Mutex<Context>> = Arc::default();

    txt_record.insert("txtvers", "1").unwrap();
    txt_record.insert("path", "/ship/").unwrap();
    txt_record.insert("id", "0").unwrap();
    txt_record.insert("ski", "0").unwrap();
    txt_record.insert("brand", "WIP").unwrap();
    txt_record.insert("model", "WIP").unwrap();
    txt_record.insert("type", &spine::commondatatypes::DeviceTypeEnumType::EnergyManagementSystem.to_string()).unwrap();

    service.set_registered_callback(Box::new(on_service_registered));
    service.set_context(Box::new(context));
    service.set_txt_record(txt_record);

    let event_service_loop = service.register().unwrap();
    let event_browse_loop = browser.browse_services().unwrap();

    loop {
        // calling `poll()` will keep this browser alive
        event_browse_loop.poll(Duration::from_secs(0)).unwrap();
        event_service_loop.poll(Duration::from_secs(0)).unwrap();
    }
}

fn on_service_discovered(
    result: zeroconf::Result<ServiceDiscovery>,
    _context: Option<Arc<dyn Any>>,
) {
    println!("Service discovered: {:?}", result.unwrap());

    // ...
}

fn on_service_registered(
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

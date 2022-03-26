use serde::{Serialize, Deserialize};
use super::commondatatypes;
use super::commandframe;

#[derive(Serialize, Deserialize, Debug)]
pub struct SpineType {
    pub datagram: Option<DatagramType>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DatagramType {
    pub header: Option<HeaderType>,
    pub payload: Option<PayloadType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HeaderType {
    pub specification_version: Option<commondatatypes::SpecificationVersionType>,
    pub address_source: Option<commondatatypes::FeatureAddressType>,
    pub address_destination: Option<commondatatypes::FeatureAddressType>,
    pub msg_counter: Option<commandframe::MsgCounterType>,
    pub msg_counter_reference: Option<commandframe::MsgCounterType>,
    pub cmd_classifier: Option<commandframe::CmdClassifierType>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PayloadType {
    pub cmd: Vec<commandframe::CmdType>,
}

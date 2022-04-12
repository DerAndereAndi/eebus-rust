use serde::{Serialize, Deserialize};
use super::{commondatatypes, commandframe};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct SpineType {
	#[serde(skip_serializing_if = "Option::is_none")]
    pub datagram: Option<DatagramType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct DatagramType {
	#[serde(skip_serializing_if = "Option::is_none")]
    pub header: Option<HeaderType>,
	#[serde(skip_serializing_if = "Option::is_none")]
    pub payload: Option<PayloadType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HeaderType {
	#[serde(skip_serializing_if = "Option::is_none")]
    pub specification_version: Option<commondatatypes::SpecificationVersionType>,
	#[serde(skip_serializing_if = "Option::is_none")]
    pub address_source: Option<commondatatypes::FeatureAddressType>,
	#[serde(skip_serializing_if = "Option::is_none")]
    pub address_destination: Option<commondatatypes::FeatureAddressType>,
	#[serde(skip_serializing_if = "Option::is_none")]
    pub msg_counter: Option<commandframe::MsgCounterType>,
	#[serde(skip_serializing_if = "Option::is_none")]
    pub msg_counter_reference: Option<commandframe::MsgCounterType>,
	#[serde(skip_serializing_if = "Option::is_none")]
    pub cmd_classifier: Option<commandframe::CmdClassifierType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct PayloadType {
	#[serde(skip_serializing_if = "Option::is_none")]
    pub cmd: Option<Vec<commandframe::CmdType>>,
}

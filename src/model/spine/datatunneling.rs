use serde::{Serialize, Deserialize};
use super::{commondatatypes};

pub type PurposeIdType = String;

pub type ChannelIdType = u32;

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DataTunnelingHeaderType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub purpose_id: Option<PurposeIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub channel_id: Option<ChannelIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub sequence_id: Option<u32>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DataTunnelingHeaderElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub purpose_id: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub channel_id: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub sequence_id: Option<commondatatypes::ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DataTunnelingCallType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub header: Option<DataTunnelingHeaderType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub payload: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DataTunnelingCallElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub header: Option<DataTunnelingHeaderElementsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub payload: Option<commondatatypes::ElementTagType>,
}

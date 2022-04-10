use serde::{Serialize, Deserialize};
use super::commondatatypes;

pub type ErrorNumberType = u32;

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ResultDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub error_number: Option<ErrorNumberType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<commondatatypes::DescriptionType>,
}

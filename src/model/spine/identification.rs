use std::fmt;

use super::super::utils;

use serde::{Serialize, Deserialize};

pub type IdentificationIdType = u8;

pub type IdentificationTypeType = IdentificationTypeEnumType;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum IdentificationTypeEnumType {
	#[serde(rename = "eui48")]
	Eui48,
	#[serde(rename = "eui64")]
	Eui64,
	#[serde(rename = "userRfidTag")]
	UserRfidTag,
}

impl fmt::Display for IdentificationTypeEnumType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		utils::provide_enum_display(self, f)
	}
}

pub type IdentificationValueType = String;

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IdentificationDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub identification_id: Option<IdentificationIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub identification_type: Option<IdentificationTypeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub identification_value: Option<IdentificationValueType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub authorized: Option<bool>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IdentificationListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub identification_data: Option<Vec<IdentificationDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IdentificationListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub identification_id: Option<IdentificationIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub identification_type: Option<IdentificationTypeType>,
}

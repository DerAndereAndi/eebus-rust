use serde::{Serialize, Deserialize};

pub type IdentificationIdType = u8;

pub type IdentificationTypeType = IdentificationTypeEnumType;

#[derive(Serialize, Deserialize, Debug)]
pub enum IdentificationTypeEnumType {
	#[serde(rename = "eui48")]
	Eui48,
	#[serde(rename = "eui64")]
	Eui64,
	#[serde(rename = "userRfidTag")]
	UserRfidTag,
}

pub type IdentificationValueType = String;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IdentificationDataType {
  pub identification_id: Option<IdentificationIdType>,
  pub identification_type: Option<IdentificationTypeType>,
  pub identification_value: Option<IdentificationValueType>,
  pub authorized: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IdentificationListDataType {
  pub identification_data: Option<Vec<IdentificationDataType>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IdentificationListDataSelectorsType {
  pub identification_id: Option<IdentificationIdType>,
  pub identification_type: Option<IdentificationTypeType>,
}

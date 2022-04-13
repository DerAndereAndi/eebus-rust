use std::fmt;

use super::super::utils;

use serde::{Serialize, Deserialize};
use super::{commondatatypes};

pub type MessagingNumberType = u32;

pub type MessagingTypeType = MessagingTypeEnumType;

pub type MessagingDataTextType = String;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum MessagingTypeEnumType {
  Logging,
  Information,
  Warning,
  Alarm,
  Emergency,
  Obsolete,
}

impl fmt::Display for MessagingTypeEnumType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		utils::provide_enum_display(self, f)
	}
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MessagingDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub timestamp: Option<commondatatypes::AbsoluteOrRelativeTimeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub messaging_number: Option<MessagingNumberType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  #[serde(rename = "type")]
  pub messaging_type: Option<MessagingTypeType>, // xsd defines "type", but that is a reserved keyword
	#[serde(skip_serializing_if = "Option::is_none")]
  pub text: Option<MessagingDataTextType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MessagingDataElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub timestamp: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub messaging_number: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub messageing_type: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub text: Option<commondatatypes::ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MessagingListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub messaging_data: Option<Vec<MessagingDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MessagingListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub timestamp_interval: Option<commondatatypes::TimestampIntervalType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub messaging_number: Option<MessagingNumberType>,
}

use std::fmt;

use super::super::utils;

use serde::{Serialize, Deserialize};
use super::commondatatypes;

pub type DeviceConfigurationKeyIdType = u32;

pub type DeviceConfigurationKeyValueStringType = String;

pub type DeviceConfigurationKeyNameType = DeviceConfigurationKeyNameEnumType;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum DeviceConfigurationKeyNameEnumType {
	#[serde(rename = "peakPowerOfPvSystem")]
	PeakPowerOfPvSystem,
	#[serde(rename = "pvCurtailmentLimitFactor")]
	PvCurtailmentLimitFactor,
	#[serde(rename = "asymmetricChargingSupported")]
	AsymmetricChargingSupported,
	#[serde(rename = "communicationsStandard")]
	CommunicationsStandard,
}

impl fmt::Display for DeviceConfigurationKeyNameEnumType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		utils::provide_enum_display(self, f)
	}
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum DeviceConfigurationKeyValueTypeType {
	#[serde(rename = "boolean")]
	Boolean,
	#[serde(rename = "date")]
	Date,
	#[serde(rename = "dateTime")]
	DateTime,
	#[serde(rename = "duration")]
	Duration,
	#[serde(rename = "string")]
	String,
	#[serde(rename = "time")]
	Time,
	#[serde(rename = "scaledNumber")]
	ScaledNumber,
}

impl fmt::Display for DeviceConfigurationKeyValueTypeType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		utils::provide_enum_display(self, f)
	}
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeviceConfigurationKeyValueValueType {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub boolean: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub date: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub date_time: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub duration: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub string: Option<DeviceConfigurationKeyValueStringType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub time: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub scaled_number: Option<commondatatypes::ScaledNumberType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeviceConfigurationKeyValueDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub key_id: Option<DeviceConfigurationKeyIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub value: Option<DeviceConfigurationKeyValueValueType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub is_value_changeable: Option<bool>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeviceConfigurationKeyValueListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub device_configuration_key_value_data: Option<Vec<DeviceConfigurationKeyValueDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeviceConfigurationKeyValueListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub key_id: Option<DeviceConfigurationKeyIdType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeviceConfigurationKeyValueDescriptionDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub key_id: Option<DeviceConfigurationKeyIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub key_name: Option<DeviceConfigurationKeyNameType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub value_type: Option<DeviceConfigurationKeyValueTypeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub unit: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub label: Option<commondatatypes::LabelType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<commondatatypes::DescriptionType>
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeviceConfigurationKeyValueDescriptionListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub device_configuration_key_value_description_data: Option<Vec<DeviceConfigurationKeyValueDescriptionDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeviceConfigurationKeyValueDescriptionListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub key_id: Option<DeviceConfigurationKeyIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub key_name: Option<DeviceConfigurationKeyNameType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeviceConfigurationKeyValueConstraintsDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub key_id: Option<DeviceConfigurationKeyIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub value_range_min: Option<DeviceConfigurationKeyValueValueType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub value_range_max: Option<DeviceConfigurationKeyValueValueType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub value_step_size: Option<DeviceConfigurationKeyValueValueType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeviceConfigurationKeyValueConstraintsListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub device_configuration_key_value_constraints_data: Option<Vec<DeviceConfigurationKeyValueConstraintsDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeviceConfigurationKeyValueConstraintsListDataSelectorsType {
  pub key_id: Option<DeviceConfigurationKeyIdType>,
}

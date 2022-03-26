use serde::{Serialize, Deserialize};
use super::commondatatypes;

pub type DeviceConfigurationKeyIdType = u8;

pub type DeviceConfigurationKeyValueStringType = String;

pub type DeviceConfigurationKeyNameType = DeviceConfigurationKeyNameEnumType;

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeviceConfigurationKeyValueValueType {
	pub boolean: Option<bool>,
  pub date: Option<String>,
  pub date_time: Option<String>,
  pub duration: Option<String>,
  pub string: Option<DeviceConfigurationKeyValueStringType>,
  pub time: Option<String>,
  pub scaled_number: Option<commondatatypes::ScaledNumberType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeviceConfigurationKeyValueDataType {
  pub key_id: Option<DeviceConfigurationKeyIdType>,
  pub value: Option<DeviceConfigurationKeyValueValueType>,
  pub is_value_changeable: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeviceConfigurationKeyValueListDataType {
  pub device_configuration_key_value_data: Option<Vec<DeviceConfigurationKeyValueDataType>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeviceConfigurationKeyValueListDataSelectorsType {
  pub key_id: Option<DeviceConfigurationKeyIdType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeviceConfigurationKeyValueDescriptionDataType {
  pub key_id: Option<DeviceConfigurationKeyIdType>,
  pub key_name: Option<String>,
  pub value_type: Option<DeviceConfigurationKeyValueTypeType>,
  pub unit: Option<String>,
  pub label: Option<commondatatypes::LabelType>,
  pub description: Option<commondatatypes::DescriptionType>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeviceConfigurationKeyValueDescriptionListDataType {
  pub device_configuration_key_value_description_data: Option<Vec<DeviceConfigurationKeyValueDescriptionDataType>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeviceConfigurationKeyValueDescriptionListDataSelectorsType {
  pub key_id: Option<DeviceConfigurationKeyIdType>,
  pub key_name: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeviceConfigurationKeyValueConstraintsDataType {
  pub key_id: Option<DeviceConfigurationKeyIdType>,
  pub value_range_min: Option<DeviceConfigurationKeyValueValueType>,
  pub value_range_max: Option<DeviceConfigurationKeyValueValueType>,
  pub value_step_size: Option<DeviceConfigurationKeyValueValueType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeviceConfigurationKeyValueConstraintsListDataType {
  pub device_configuration_key_value_constraints_data: Option<Vec<DeviceConfigurationKeyValueConstraintsDataType>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeviceConfigurationKeyValueConstraintsListDataSelectorsType {
  pub key_id: Option<DeviceConfigurationKeyIdType>,
}

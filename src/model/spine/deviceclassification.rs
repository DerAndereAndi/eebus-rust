use serde::{Serialize, Deserialize};
use super::commondatatypes;

pub type DeviceClassificationStringType = String;

pub type PowerSourceType = PowerSourceEnumType;

#[derive(Serialize, Deserialize, Debug)]
pub enum PowerSourceEnumType {
	#[serde(rename = "unknown")]
	Unknown,
	#[serde(rename = "mainsSinglePhase")]
	MainsSinglePhase,
	#[serde(rename = "mains3Phase")]
	Mains3Phase,
	#[serde(rename = "battery")]
	Battery,
	#[serde(rename = "dc")]
	Dverrun,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeviceClassificationManufacturerDataType {
	pub device_name: Option<DeviceClassificationStringType>,
	pub device_code: Option<DeviceClassificationStringType>,
	pub serial_number: Option<DeviceClassificationStringType>,
	pub software_revision: Option<DeviceClassificationStringType>,
	pub hardware_revision: Option<DeviceClassificationStringType>,
	pub vendor_name: Option<DeviceClassificationStringType>,
	pub vendor_code: Option<DeviceClassificationStringType>,
	pub brand_name: Option<DeviceClassificationStringType>,
  pub power_source: Option<PowerSourceType>,
  pub manufcaturer_node_identification: Option<DeviceClassificationStringType>,
  pub manufacturer_label: Option<commondatatypes::LabelType>,
  pub manufacturer_description: Option<commondatatypes::DescriptionType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeviceClassificationUserDataType {
  pub user_node_identification: Option<DeviceClassificationStringType>,
  pub user_label: Option<commondatatypes::LabelType>,
  pub user_description: Option<commondatatypes::DescriptionType>,
}

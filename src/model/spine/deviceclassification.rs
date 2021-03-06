use std::fmt;

use super::super::utils;

use serde::{Serialize, Deserialize};
use super::commondatatypes;

pub type DeviceClassificationStringType = String;

pub type PowerSourceType = PowerSourceEnumType;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
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

impl fmt::Display for PowerSourceEnumType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		utils::provide_enum_display(self, f)
	}
}

#[derive(Serialize, Deserialize, Default, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DeviceClassificationManufacturerDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub device_name: Option<DeviceClassificationStringType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub device_code: Option<DeviceClassificationStringType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub serial_number: Option<DeviceClassificationStringType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub software_revision: Option<DeviceClassificationStringType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub hardware_revision: Option<DeviceClassificationStringType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub vendor_name: Option<DeviceClassificationStringType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub vendor_code: Option<DeviceClassificationStringType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub brand_name: Option<DeviceClassificationStringType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub power_source: Option<PowerSourceType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub manufcaturer_node_identification: Option<DeviceClassificationStringType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub manufacturer_label: Option<commondatatypes::LabelType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub manufacturer_description: Option<commondatatypes::DescriptionType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeviceClassificationManufacturerDataElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub device_name: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub device_code: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub serial_number: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub software_revision: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub hardware_revision: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub vendor_name: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub vendor_code: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub brand_name: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub power_source: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub manufcaturer_node_identification: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub manufacturer_label: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub manufacturer_description: Option<commondatatypes::ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeviceClassificationUserDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub user_node_identification: Option<DeviceClassificationStringType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub user_label: Option<commondatatypes::LabelType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub user_description: Option<commondatatypes::DescriptionType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeviceClassificationUserDataElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user_node_identification: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user_label: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub user_description: Option<commondatatypes::ElementTagType>,
}
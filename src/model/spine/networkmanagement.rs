use std::fmt;

use super::super::utils;

use serde::{Serialize, Deserialize};
use super::commondatatypes;

pub type NetworkManagementNativeSetupType = String;

pub type NetworkManagementScanSetupType = String;

pub type NetworkManagementSetupType = String;

pub type NetworkManagementCandidateSetupType = String;

pub type NetworkManagementTechnologyAddressType = String;

pub type NetworkManagementCommunicationsTechnologyInformationType = String;

pub type NetworkManagementMinimumTrustLevelType = String;

pub type NetworkManagementProcessTimeoutType = String;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum NetworkManagementFeatureSetType {
	#[serde(rename = "gateway")]
	Gateway,
	#[serde(rename = "router")]
	Router,
	#[serde(rename = "smart")]
	Smart,
	#[serde(rename = "simple")]
	Simple,
}

impl fmt::Display for NetworkManagementFeatureSetType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		utils::provide_enum_display(self, f)
	}
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum NetworkManagementProcessStateStateType {
	#[serde(rename = "succeeded")]
	Succeeded,
	#[serde(rename = "failed")]
	Failed,
	#[serde(rename = "aborted")]
	Aborted,
}

impl fmt::Display for NetworkManagementProcessStateStateType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		utils::provide_enum_display(self, f)
	}
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum NetworkManagementStateChangeType {
	#[serde(rename = "added")]
	Added,
	#[serde(rename = "removed")]
	Removed,
	#[serde(rename = "modified")]
	Modified,
}

impl fmt::Display for NetworkManagementStateChangeType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		utils::provide_enum_display(self, f)
	}
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NetworkManagementAddNodeCallType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub node_address: Option<commondatatypes::FeatureAddressType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub native_setup: Option<NetworkManagementNativeSetupType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub timeout: Option<NetworkManagementProcessTimeoutType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub label: Option<commondatatypes::LabelType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<commondatatypes::DescriptionType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NetworkManagementAddNodeCallElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub node_address: Option<commondatatypes::FeatureAddressElementsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub native_setup: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub timeout: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub label: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<commondatatypes::ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NetworkManagementRemoveNodeCallType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub node_address: Option<commondatatypes::FeatureAddressType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub timeout: Option<NetworkManagementProcessTimeoutType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NetworkManagementRemoveNodeCallElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub node_address: Option<commondatatypes::FeatureAddressElementsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub timeout: Option<commondatatypes::ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NetworkManagementModifyNodeCallType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub node_address: Option<commondatatypes::FeatureAddressType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub native_setup: Option<NetworkManagementNativeSetupType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub timeout: Option<NetworkManagementProcessTimeoutType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub label: Option<commondatatypes::LabelType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<commondatatypes::DescriptionType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NetworkManagementModifyNodeCallElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub node_address: Option<commondatatypes::FeatureAddressElementsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub native_setup: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub timeout: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub label: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<commondatatypes::ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NetworkManagementScanNetworkCallType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub scan_setup: Option<NetworkManagementScanSetupType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub timeout: Option<NetworkManagementProcessTimeoutType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NetworkManagementScanNetworkCallElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub scan_setup: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub timeout: Option<commondatatypes::ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NetworkManagementDiscoverCallType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub discover_address: Option<commondatatypes::FeatureAddressType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NetworkManagementDiscoverCallElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub discover_address: Option<commondatatypes::FeatureAddressElementsType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct NetworkManagementAbortCallType {}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct NetworkManagementAbortCallElementsType {}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NetworkManagementProcessStateDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  state: Option<NetworkManagementProcessStateStateType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  description: Option<commondatatypes::DescriptionType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NetworkManagementProcessStateDataElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub state: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<commondatatypes::ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NetworkManagementJoiningModeDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub setup: Option<NetworkManagementSetupType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NetworkManagementJoiningModeDataElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub setup: Option<commondatatypes::ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NetworkManagementReportCandidateDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub candidate_setup: Option<NetworkManagementCandidateSetupType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub setup_usable_for_add: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub label: Option<commondatatypes::LabelType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<commondatatypes::DescriptionType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NetworkManagementReportCandidateDataElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub candidate_setup: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub setup_usable_for_add: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub label: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<commondatatypes::ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NetworkManagementDeviceDescriptionDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub device_address: Option<commondatatypes::DeviceAddressType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub device_type: Option<commondatatypes::DeviceTypeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub network_management_responsible_address: Option<commondatatypes::FeatureAddressType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub native_setup: Option<NetworkManagementNativeSetupType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub technology_address: Option<NetworkManagementTechnologyAddressType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub communications_technology_information: Option<NetworkManagementCommunicationsTechnologyInformationType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub network_feature_set: Option<NetworkManagementFeatureSetType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub last_state_change: Option<NetworkManagementStateChangeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub minimum_trust_level: Option<NetworkManagementMinimumTrustLevelType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub label: Option<commondatatypes::LabelType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<commondatatypes::DescriptionType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NetworkManagementDeviceDescriptionDataElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub device_address: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub device_type: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub network_management_responsible_address: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub native_setup: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub technology_address: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub communications_technology_information: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub network_feature_set: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub last_state_change: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub minimum_trust_level: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub label: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<commondatatypes::ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NetworkManagementDeviceDescriptionListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub network_management_device_description_data: Option<Vec<NetworkManagementDeviceDescriptionDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NetworkManagementDeviceDescriptionListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub device_address: Option<commondatatypes::DeviceAddressType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub device_type: Option<commondatatypes::DeviceTypeType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NetworkManagementEntityDescriptionDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub entity_address: Option<commondatatypes::EntityAddressType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub entity_type: Option<commondatatypes::EntityTypeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub last_state_change: Option<NetworkManagementStateChangeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub minimum_trust_level: Option<NetworkManagementMinimumTrustLevelType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub label: Option<commondatatypes::LabelType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<commondatatypes::DescriptionType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NetworkManagementEntityDescriptionDataElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub entity_address: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub entity_type: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub last_state_change: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub minimum_trust_level: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub label: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<commondatatypes::ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NetworkManagementEntityDescriptionListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub network_management_entity_description_data: Option<Vec<NetworkManagementEntityDescriptionDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NetworkManagementEntityDescriptionListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub entity_address: Option<commondatatypes::EntityAddressType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub entity_type: Option<commondatatypes::EntityTypeType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NetworkManagementFeatureDescriptionDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub feature_address: Option<commondatatypes::FeatureAddressType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub feature_type: Option<commondatatypes::FeatureTypeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub specific_usage: Option<Vec<commondatatypes::FeatureSpecificUsageType>>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub feature_group: Option<commondatatypes::FeatureGroupType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub role: Option<commondatatypes::RoleType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub supported_function: Option<Vec<commondatatypes::FunctionPropertyType>>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub last_state_change: Option<NetworkManagementStateChangeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub minimum_trust_level: Option<NetworkManagementMinimumTrustLevelType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub label: Option<commondatatypes::LabelType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<commondatatypes::DescriptionType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub max_response_delay: Option<commondatatypes::MaxResponseDelayType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NetworkManagementFeatureDescriptionDataElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub feature_address: Option<commondatatypes::FeatureAddressElementsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub feature_type: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub specific_usage: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub feature_group: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub role: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub supported_function: Option<commondatatypes::FunctionPropertyElementsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub last_state_change: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub minimum_trust_level: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub label: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_response_delay: Option<commondatatypes::ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NetworkManagementFeatureDescriptionListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub network_management_feature_description_data: Option<Vec<NetworkManagementFeatureDescriptionDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NetworkManagementFeatureDescriptionListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub feature_address: Option<commondatatypes::FeatureAddressType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub feature_type: Option<commondatatypes::FeatureTypeType>,
}

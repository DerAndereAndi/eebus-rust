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

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
pub enum NetworkManagementProcessStateStateType {
	#[serde(rename = "succeeded")]
	Succeeded,
	#[serde(rename = "failed")]
	Failed,
	#[serde(rename = "aborted")]
	Aborted,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum NetworkManagementStateChangeType {
	#[serde(rename = "added")]
	Added,
	#[serde(rename = "removed")]
	Removed,
	#[serde(rename = "modified")]
	Modified,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NetworkManagementAddNodeCallType {
  pub node_address: Option<commondatatypes::FeatureAddressType>,
  pub native_setup: Option<NetworkManagementNativeSetupType>,
  pub timeout: Option<NetworkManagementProcessTimeoutType>,
  pub label: Option<commondatatypes::LabelType>,
  pub description: Option<commondatatypes::DescriptionType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NetworkManagementRemoveNodeCallType {
  pub node_address: Option<commondatatypes::FeatureAddressType>,
  pub timeout: Option<NetworkManagementProcessTimeoutType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NetworkManagementModifyNodeCallType {
  pub node_address: Option<commondatatypes::FeatureAddressType>,
  pub native_setup: Option<NetworkManagementNativeSetupType>,
  pub timeout: Option<NetworkManagementProcessTimeoutType>,
  pub label: Option<commondatatypes::LabelType>,
  pub description: Option<commondatatypes::DescriptionType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NetworkManagementScanNetworkCallType {
  pub scan_setup: Option<NetworkManagementScanSetupType>,
  pub timeout: Option<NetworkManagementProcessTimeoutType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NetworkManagementDiscoverCallType {
  pub discover_address: Option<commondatatypes::FeatureAddressType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NetworkManagementAbortCallType {
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NetworkManagementProcessStateDataType {
  state: Option<NetworkManagementProcessStateStateType>,
  description: Option<commondatatypes::DescriptionType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NetworkManagementJoiningModeDataType {
  pub setup: Option<NetworkManagementSetupType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NetworkManagementReportCandidateDataType {
  pub candidate_setup: Option<NetworkManagementCandidateSetupType>,
  pub setup_usable_for_add: Option<bool>,
  pub label: Option<commondatatypes::LabelType>,
  pub description: Option<commondatatypes::DescriptionType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NetworkManagementDeviceDescriptionDataType {
  pub device_address: Option<commondatatypes::DeviceAddressType>,
  pub device_type: Option<commondatatypes::DeviceTypeType>,
  pub network_management_responsible_address: Option<commondatatypes::FeatureAddressType>,
  pub native_setup: Option<NetworkManagementNativeSetupType>,
  pub technology_address: Option<NetworkManagementTechnologyAddressType>,
  pub communications_technology_information: Option<NetworkManagementCommunicationsTechnologyInformationType>,
  pub network_feature_set: Option<NetworkManagementFeatureSetType>,
  pub last_state_change: Option<NetworkManagementStateChangeType>,
  pub minimum_trust_level: Option<NetworkManagementMinimumTrustLevelType>,
  pub label: Option<commondatatypes::LabelType>,
  pub description: Option<commondatatypes::DescriptionType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NetworkManagementDeviceDescriptionListDataType {
  pub network_management_device_description_data: Option<Vec<NetworkManagementDeviceDescriptionDataType>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NetworkManagementDeviceDescriptionListDataSelectorsType {
  pub device_address: Option<commondatatypes::DeviceAddressType>,
  pub device_type: Option<commondatatypes::DeviceTypeType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NetworkManagementEntityDescriptionDataType {
  pub entity_address: Option<commondatatypes::EntityAddressType>,
  pub entity_type: Option<commondatatypes::EntityTypeType>,
  pub last_state_change: Option<NetworkManagementStateChangeType>,
  pub minimum_trust_level: Option<NetworkManagementMinimumTrustLevelType>,
  pub label: Option<commondatatypes::LabelType>,
  pub description: Option<commondatatypes::DescriptionType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NetworkManagementEntityDescriptionListDataType {
  pub network_management_entity_description_data: Option<Vec<NetworkManagementEntityDescriptionDataType>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NetworkManagementEntityDescriptionListDataSelectorsType {
  pub entity_address: Option<commondatatypes::EntityAddressType>,
  pub entity_type: Option<commondatatypes::EntityTypeType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NetworkManagementFeatureDescriptionDataType {
  pub feuture_address: Option<commondatatypes::FeatureAddressType>,
  pub feature_type: Option<commondatatypes::FeatureTypeType>,
  pub specific_usage: Option<Vec<commondatatypes::FeatureSpecificUsageType>>,
  pub feature_group: Option<commondatatypes::FeatureGroupType>,
  pub role: Option<commondatatypes::RoleType>,
  pub supported_function: Option<Vec<commondatatypes::FunctionPropertyType>>,
  pub last_state_change: Option<NetworkManagementStateChangeType>,
  pub minimum_trust_level: Option<NetworkManagementMinimumTrustLevelType>,
  pub label: Option<commondatatypes::LabelType>,
  pub description: Option<commondatatypes::DescriptionType>,
  pub max_response_delay: Option<commondatatypes::MaxResponseDelayType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NetworkManagementFeatureDescriptionListDataType {
  pub network_management_feature_description_data: Option<Vec<NetworkManagementFeatureDescriptionDataType>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NetworkManagementFeatureDescriptionListDataSelectorsType {
  pub feuture_address: Option<commondatatypes::FeatureAddressType>,
  pub feature_type: Option<commondatatypes::FeatureTypeType>,
}

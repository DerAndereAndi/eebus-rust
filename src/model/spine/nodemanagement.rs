use serde::{Serialize, Deserialize};
use super::commondatatypes;
use super::networkmanagement;
use super::usecaseinformation;
use super::bindingmanagement;
use super::subscriptionmanagement;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NodeManagementSpecificationVersionListType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub specification_version: Option<Vec<commondatatypes::SpecificationVersionType>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NodeManagementDetailedDiscoveryDeviceInformationType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<networkmanagement::NetworkManagementDeviceDescriptionDataType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NodeManagementDetailedDiscoveryEntityInformationType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<networkmanagement::NetworkManagementEntityDescriptionDataType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NodeManagementDetailedDiscoveryFeatureInformationType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<networkmanagement::NetworkManagementFeatureDescriptionDataType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NodeManagementDetailedDiscoveryDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub specification_version_list: Option<NodeManagementSpecificationVersionListType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub device_information: Option<NodeManagementDetailedDiscoveryDeviceInformationType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub entity_information: Option<Vec<NodeManagementDetailedDiscoveryEntityInformationType>>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub feature_information: Option<Vec<NodeManagementDetailedDiscoveryFeatureInformationType>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NodeManagementDetailedDiscoveryDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub device_information: Option<networkmanagement::NetworkManagementDeviceDescriptionListDataSelectorsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub entity_information: Option<networkmanagement::NetworkManagementEntityDescriptionListDataSelectorsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub feature_information: Option<networkmanagement::NetworkManagementFeatureDescriptionListDataSelectorsType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NodeManagementBindingDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub binding_entry: Option<Vec<bindingmanagement::BindingManagementEntryDataType>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NodeManagementBindingDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub binding_entry: Option<bindingmanagement::BindingManagementEntryListDataSelectorsType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NodeManagementBindingRequestCallType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub binding_request: Option<bindingmanagement::BindingManagementRequestCallType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NodeManagementSubscriptionDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub subscription_entry: Option<Vec<subscriptionmanagement::SubscriptionManagementEntryDataType>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NodeManagementSubscriptionDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub subscription_entry: Option<subscriptionmanagement::SubscriptionManagementEntryListDataSelectorsType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NodeManagementSubscriptionRequestCallType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub subscription_request: Option<subscriptionmanagement::SubscriptionManagementRequestCallType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NodeManagementSubscriptionDeleteCallType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub subscription_delete: Option<subscriptionmanagement::SubscriptionManagementDeleteCallType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NodeManagementDestinationDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub device_description: Option<networkmanagement::NetworkManagementDeviceDescriptionDataType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NodeManagementDestinationListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub node_management_destination_data: Option<Vec<NodeManagementDestinationDataType>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NodeManagementDestinationListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub device_description: Option<networkmanagement::NetworkManagementDeviceDescriptionListDataSelectorsType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NodeManagementUseCaseDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub use_case_information: Option<Vec<usecaseinformation::UseCaseInformationDataType>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NodeManagementUseCaseDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub use_case_information: Option<usecaseinformation::UseCaseInformationListDataSelectorsType>,
}

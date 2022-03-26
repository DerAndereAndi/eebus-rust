use serde::{Serialize, Deserialize};
use super::commondatatypes;
use super::networkmanagement;
use super::usecaseinformation;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NodeManagementSpecificationVersionListType {
  pub specification_version: Option<Vec<commondatatypes::SpecificationVersionType>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NodeManagementDetailedDiscoveryDeviceInformationType {
  pub description: Option<networkmanagement::NetworkManagementDeviceDescriptionDataType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NodeManagementDetailedDiscoveryEntityInformationType {
  pub description: Option<networkmanagement::NetworkManagementEntityDescriptionDataType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NodeManagementDetailedDiscoveryFeatureInformationType {
  pub description: Option<networkmanagement::NetworkManagementFeatureDescriptionDataType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NodeManagementDetailedDiscoveryDataType {
  pub specification_version_list: Option<NodeManagementSpecificationVersionListType>,
  pub device_information: Option<NodeManagementDetailedDiscoveryDeviceInformationType>,
  pub entity_information: Option<Vec<NodeManagementDetailedDiscoveryEntityInformationType>>,
  pub feature_information: Option<Vec<NodeManagementDetailedDiscoveryFeatureInformationType>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NodeManagementDetailedDiscoveryDataSelectorsType {
  pub device_information: Option<NetworkManagementDeviceDescriptionListDataSelectorsType>,
  pub entity_information: Option<NetworkManagementEntityDescriptionListDataSelectorsType>,
  pub feature_information: Option<NetworkManagementFeatureDescriptionListDataSelectorsType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NodeManagementBindingDataType {
  pub binding_entry: Option<Vec<BindingManagementEntryDataType>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NodeManagementBindingDataSelectorsType {
  pub binding_entry: Option<BindingManagementEntryListDataSelectorsType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NodeManagementBindingRequestCallType {
  pub binding_request: Option<BindingManagementRequestCallType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NodeManagementSubscriptionDataType {
  pub subscription_entry: Option<Vec<SubscriptionManagementEntryDataType>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NodeManagementSubscriptionDataSelectorsType {
  pub subscription_entry: Option<SubscriptionManagementEntryListDataSelectorsType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NodeManagementSubscriptionRequestCallType {
  pub subscription_request: Option<SubscriptionManagementRequestCallType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NodeManagementSubscriptionDeleteCallType {
  pub subscription_delete: Option<SubscriptionManagementDeleteCallType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NodeManagementDestinationDataType {
  pub device_description: Option<networkmanagement::NetworkManagementDeviceDescriptionDataType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NodeManagementDestinationListDataType {
  pub node_management_destination_data: Option<Vec<NodeManagementDestinationDataType>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NodeManagementDestinationListDataSelectorsType {
  pub device_description: Option<networkmanagement::NetworkManagementDeviceDescriptionListDataSelectorsType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NodeManagementUseCaseDataType {
  pub use_case_information: Option<Vec<usecaseinformation::UseCaseInformationDataType>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NodeManagementUseCaseDataSelectorsType {
  pub use_case_information: Option<usecaseinformation::UseCaseInformationListDataSelectorsType>,
}

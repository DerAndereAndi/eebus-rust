use serde::{Serialize, Deserialize};
use super::{commondatatypes, networkmanagement, usecaseinformation, bindingmanagement, subscriptionmanagement, version};

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NodeManagementSpecificationVersionListType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub specification_version: Option<Vec<commondatatypes::SpecificationVersionType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NodeManagementDetailedDiscoveryDeviceInformationType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<networkmanagement::NetworkManagementDeviceDescriptionDataType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NodeManagementDetailedDiscoveryEntityInformationType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<networkmanagement::NetworkManagementEntityDescriptionDataType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NodeManagementDetailedDiscoveryFeatureInformationType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<networkmanagement::NetworkManagementFeatureDescriptionDataType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
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

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NodeManagementSpecificationVersionListElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub specification_version: Option<version::SpecificationVersionDataElementsType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NodeManagementDetailedDiscoveryDeviceInformationElementsType {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<networkmanagement::NetworkManagementDeviceDescriptionDataElementsType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NodeManagementDetailedDiscoveryEntityInformationElementsType {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<networkmanagement::NetworkManagementEntityDescriptionDataElementsType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NodeManagementDetailedDiscoveryFeatureInformationElementsType {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<networkmanagement::NetworkManagementFeatureDescriptionDataElementsType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NodeManagementDetailedDiscoveryDataElementsType {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub specification_version_list: Option<NodeManagementSpecificationVersionListElementsType>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub device_information: Option<NodeManagementDetailedDiscoveryDeviceInformationElementsType>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub entity_information: Option<Vec<NodeManagementDetailedDiscoveryEntityInformationElementsType>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub feature_information: Option<Vec<NodeManagementDetailedDiscoveryFeatureInformationElementsType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NodeManagementDetailedDiscoveryDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub device_information: Option<networkmanagement::NetworkManagementDeviceDescriptionListDataSelectorsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub entity_information: Option<networkmanagement::NetworkManagementEntityDescriptionListDataSelectorsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub feature_information: Option<networkmanagement::NetworkManagementFeatureDescriptionListDataSelectorsType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NodeManagementBindingDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub binding_entry: Option<Vec<bindingmanagement::BindingManagementEntryDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NodeManagementBindingDataElementsType {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub binding_entry: Option<Vec<bindingmanagement::BindingManagementEntryDataElementsType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NodeManagementBindingDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub binding_entry: Option<bindingmanagement::BindingManagementEntryListDataSelectorsType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NodeManagementBindingRequestCallType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub binding_request: Option<bindingmanagement::BindingManagementRequestCallType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NodeManagementBindingRequestCallElementsType {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub binding_request: Option<bindingmanagement::BindingManagementRequestCallElementsType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NodeManagementBindingDeleteCallType {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub binding_delete: Option<bindingmanagement::BindingManagementDeleteCallType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NodeManagementBindingDeleteCallElementsType {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub binding_delete: Option<bindingmanagement::BindingManagementDeleteCallElementsType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NodeManagementSubscriptionDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub subscription_entry: Option<Vec<subscriptionmanagement::SubscriptionManagementEntryDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NodeManagementSubscriptionDataElementsType {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub subscription_entry: Option<Vec<subscriptionmanagement::SubscriptionManagementEntryDataElementsType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NodeManagementSubscriptionDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub subscription_entry: Option<subscriptionmanagement::SubscriptionManagementEntryListDataSelectorsType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NodeManagementSubscriptionRequestCallType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub subscription_request: Option<subscriptionmanagement::SubscriptionManagementRequestCallType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NodeManagementSubscriptionRequestCallElementsType {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub subscription_request: Option<subscriptionmanagement::SubscriptionManagementRequestCallElementsType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NodeManagementSubscriptionDeleteCallType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub subscription_delete: Option<subscriptionmanagement::SubscriptionManagementDeleteCallType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NodeManagementSubscriptionDeleteCallElementsType {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub subscription_delete: Option<subscriptionmanagement::SubscriptionManagementDeleteCallElementsType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NodeManagementDestinationDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub device_description: Option<networkmanagement::NetworkManagementDeviceDescriptionDataType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NodeManagementDestinationDataElementsType {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub device_description: Option<networkmanagement::NetworkManagementDeviceDescriptionDataElementsType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NodeManagementDestinationListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub node_management_destination_data: Option<Vec<NodeManagementDestinationDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NodeManagementDestinationListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub device_description: Option<networkmanagement::NetworkManagementDeviceDescriptionListDataSelectorsType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NodeManagementUseCaseDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub use_case_information: Option<Vec<usecaseinformation::UseCaseInformationDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NodeManagementUseCaseDataElementsType {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub use_case_information: Option<usecaseinformation::UseCaseInformationDataElementsType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct NodeManagementUseCaseDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub use_case_information: Option<usecaseinformation::UseCaseInformationListDataSelectorsType>,
}

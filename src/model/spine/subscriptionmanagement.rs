use serde::{Serialize, Deserialize};
use super::commondatatypes;

pub type SubscriptionIdType = u32;

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionManagementEntryDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub subscription_id: Option<SubscriptionIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub client_address: Option<commondatatypes::FeatureAddressType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub server_address: Option<commondatatypes::FeatureAddressType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub label: Option<commondatatypes::LabelType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<commondatatypes::DescriptionType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionManagementEntryDataElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub subscription_id: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub client_address: Option<commondatatypes::FeatureAddressElementsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub server_address: Option<commondatatypes::FeatureAddressElementsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub label: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<commondatatypes::ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionManagementEntryListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub subscription_management_entry: Option<Vec<SubscriptionManagementEntryDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionManagementEntryListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub subscription_id: Option<SubscriptionIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub client_address: Option<commondatatypes::FeatureAddressType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub server_address: Option<commondatatypes::FeatureAddressType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionManagementRequestCallType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub client_address: Option<commondatatypes::FeatureAddressType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub server_address: Option<commondatatypes::FeatureAddressType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub server_feature_type: Option<commondatatypes::FeatureTypeType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionManagementRequestCallElementsType {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub client_address: Option<commondatatypes::FeatureAddressElementsType>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub server_address: Option<commondatatypes::FeatureAddressElementsType>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub server_feature_type: Option<commondatatypes::ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionManagementDeleteCallType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub subscription_id: Option<SubscriptionIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub client_address: Option<commondatatypes::FeatureAddressType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub server_address: Option<commondatatypes::FeatureAddressType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionManagementDeleteCallElementsType {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub subscription_id: Option<commondatatypes::ElementTagType>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub client_address: Option<commondatatypes::FeatureAddressElementsType>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub server_address: Option<commondatatypes::FeatureAddressElementsType>,
}

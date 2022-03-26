use serde::{Serialize, Deserialize};
use super::commondatatypes;

pub type SubscriptionIdType = u8;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionManagementEntryDataType {
  pub subscription_id: Option<SubscriptionIdType>,
  pub client_address: Option<commondatatypes::FeatureAddressType>,
  pub server_address: Option<commondatatypes::FeatureAddressType>,
  pub label: Option<commondatatypes::LabelType>,
  pub description: Option<commondatatypes::DescriptionType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionManagementEntryListDataType {
  pub subscription_management_entry: Option<Vec<SubscriptionManagementEntryDataType>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionManagementEntryListDataSelectorsType {
  pub subscription_id: Option<SubscriptionIdType>,
  pub client_address: Option<commondatatypes::FeatureAddressType>,
  pub server_address: Option<commondatatypes::FeatureAddressType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionManagementRequestCallType {
  pub subscription_id: Option<SubscriptionIdType>,
  pub server_address: Option<commondatatypes::FeatureAddressType>,
  pub server_feature_type: Option<commondatatypes::FeatureTypeType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionManagementDeleteCallType {
  pub subscription_id: Option<SubscriptionIdType>,
  pub client_address: Option<commondatatypes::FeatureAddressType>,
  pub server_address: Option<commondatatypes::FeatureAddressType>,
}

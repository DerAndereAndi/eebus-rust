use serde::{Serialize, Deserialize};
use super::commondatatypes;

pub type BindingIdType = u8

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BindingManagementEntryDataType {
  pub binding_id: Option<BindingIdType>,
  pub client_address: Option<commondatatypes::FeatureAddressType>,
  pub server_address: Option<commondatatypes::FeatureAddressType>,
  pub label: Option<commondatatypes::LabelType>,
  pub description: Option<commondatatypes::DescriptionType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BindingManagementEntryListDataType {
  pub binding_management_entry_data: Option<Vec<BindingManagementEntryDataType>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BindingManagementEntryListDataSelectorsType {
  pub binding_id: Option<BindingIdType>,
  pub client_address: Option<commondatatypes::FeatureAddressType>,
  pub server_address: Option<commondatatypes::FeatureAddressType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BindingManagementRequestCallType {
  pub client_address: Option<commondatatypes::FeatureAddressType>,
  pub server_address: Option<commondatatypes::FeatureAddressType>,
  pub server_feature_type: Option<commondatatypes::FeatureTypeType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BindingManagementDeleteCallType {
  pub binding_id: Option<BindingIdType>,
  pub client_address: Option<commondatatypes::FeatureAddressType>,
  pub server_address: Option<commondatatypes::FeatureAddressType>,
}

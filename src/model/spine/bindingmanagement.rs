use serde::{Serialize, Deserialize};
use super::commondatatypes;

pub type BindingIdType = u32;

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BindingManagementEntryDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub binding_id: Option<BindingIdType>,
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
pub struct BindingManagementEntryDataElementsType {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub binding_id: Option<commondatatypes::ElementTagType>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub client_address: Option<commondatatypes::ElementTagType>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub server_address: Option<commondatatypes::ElementTagType>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub label: Option<commondatatypes::ElementTagType>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<commondatatypes::ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BindingManagementEntryListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub binding_management_entry_data: Option<Vec<BindingManagementEntryDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BindingManagementEntryListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub binding_id: Option<BindingIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub client_address: Option<commondatatypes::FeatureAddressType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub server_address: Option<commondatatypes::FeatureAddressType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BindingManagementRequestCallType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub client_address: Option<commondatatypes::FeatureAddressType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub server_address: Option<commondatatypes::FeatureAddressType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub server_feature_type: Option<commondatatypes::FeatureTypeType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BindingManagementRequestCallElementsType {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub client_address: Option<commondatatypes::ElementTagType>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub server_address: Option<commondatatypes::ElementTagType>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub server_feature_type: Option<commondatatypes::ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BindingManagementDeleteCallType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub binding_id: Option<BindingIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub client_address: Option<commondatatypes::FeatureAddressType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub server_address: Option<commondatatypes::FeatureAddressType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BindingManagementDeleteCallElementsType {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub binding_id: Option<commondatatypes::ElementTagType>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub client_address: Option<commondatatypes::ElementTagType>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub server_address: Option<commondatatypes::ElementTagType>,
}
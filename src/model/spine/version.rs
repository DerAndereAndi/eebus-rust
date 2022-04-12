use serde::{Serialize, Deserialize};
use super::commondatatypes;

pub type SpecificationVersionDataType = commondatatypes::SpecificationVersionType;

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SpecificationVersionDataElementsType {}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SpecificationVersionListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub specification_version_data: Option<Vec<SpecificationVersionDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SpecificationVersionListDataSelectorsType {
}

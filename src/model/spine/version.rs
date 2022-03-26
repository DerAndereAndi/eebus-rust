use serde::{Serialize, Deserialize};
use super::commondatatypes;

pub type SpecificationVersionDataType = commondatatypes::SpecificationVersionType;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SpecificationVersionListDataType {
  pub specification_version_data: Option<Vec<SpecificationVersionDataType>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SpecificationVersionListDataSelectorsType {
}

use serde::{Serialize, Deserialize};
use super::commondatatypes;

pub type SpecificationVersionDataType = SpecificationVersionType;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
type SpecificationVersionListDataType struct {
  pub specification_version_data: Option<Vec<SpecificationVersionDataType>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
type SpecificationVersionListDataSelectorsType struct {
}

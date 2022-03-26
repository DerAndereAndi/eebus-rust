use serde::{Serialize, Deserialize};
use super::commondatatypes;

pub type ErrorNumberType = u8;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
type ResultDataType struct {
  pub error_number: Option<ErrorNumberType>,
  pub description: Option<commondatatypes::DescriptionType>,
}

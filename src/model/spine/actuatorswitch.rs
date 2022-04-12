use std::fmt;

use super::super::utils;

use serde::{Serialize, Deserialize};
use super::{commondatatypes};

pub type ActuatorSwitchFctType = ActuatorSwitchFctEnumType;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum ActuatorSwitchFctEnumType {
  On,
  Off,
  Toggle,
}

impl fmt::Display for ActuatorSwitchFctEnumType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		utils::provide_enum_display(self, f)
	}
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ActuatorSwitchDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub function: Option<ActuatorSwitchFctType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ActuatorSwitchDataElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub function: Option<commondatatypes::ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ActuatorSwitchDescriptionDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub label: Option<commondatatypes::LabelType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<commondatatypes::DescriptionType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ActuatorSwitchDescriptionDataElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub label: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<commondatatypes::ElementTagType>,
}

use std::fmt;

use super::super::utils;

use serde::{Serialize, Deserialize};
use super::{commondatatypes};

pub type ActuatorLevelFctType = ActuatorLevelFctEnumType;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum ActuatorLevelFctEnumType {
  Start,
  Up,
  Down,
  Stop,
  PercentageAbsolute,
  PercentageRelative,
  Absolut,
  Relative,
}

impl fmt::Display for ActuatorLevelFctEnumType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		utils::provide_enum_display(self, f)
	}
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ActuatorLevelDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub function: Option<ActuatorLevelFctType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub value: Option<commondatatypes::ScaledNumberType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ActuatorLevelDataElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub function: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub value: Option<commondatatypes::ScaledNumberElementsType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ActuatorLevelDescriptionDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub label: Option<commondatatypes::LabelType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<commondatatypes::DescriptionType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub level_default_unit: Option<commondatatypes::UnitOfMeasurementType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ActuatorLevelDescriptionDataElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub label: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub level_default_unit: Option<commondatatypes::ElementTagType>,
}

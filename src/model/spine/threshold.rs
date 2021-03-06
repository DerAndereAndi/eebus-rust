use std::fmt;

use super::super::utils;

use serde::{Serialize, Deserialize};
use super::commondatatypes;

pub type ThresholdIdType = u32;

pub type ThresholdTypeType = ThresholdTypeEnumType;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum ThresholdTypeEnumType {
	#[serde(rename = "goodAbove")]
	GoodAbove,
	#[serde(rename = "badAbove")]
	BadAbove,
	#[serde(rename = "goodBelow")]
	GoodBelow,
	#[serde(rename = "badBelow")]
	BadBelow,
	#[serde(rename = "minValueThreshold")]
	MinValueThreshold,
	#[serde(rename = "maxValueThreshold")]
	MaxValueThreshold,
	#[serde(rename = "minValueThresholdExtreme")]
	MinValueThresholdExtreme,
	#[serde(rename = "maxValueThresholdExtreme")]
	MaxValueThresholdExtreme,
	#[serde(rename = "sagThreshold")]
	SagThreshold,
	#[serde(rename = "swellThreshold")]
	SwellThreshold,
}

impl fmt::Display for ThresholdTypeEnumType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		utils::provide_enum_display(self, f)
	}
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ThresholdDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub threshold_id: Option<ThresholdIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub threshold_value: Option<commondatatypes::ScaledNumberType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ThresholdDataElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub threshold_id: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub threshold_value: Option<commondatatypes::ScaledNumberElementsType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ThresholdListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub threshold_data: Option<Vec<ThresholdDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ThresholdListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub threshold_id: Option<ThresholdIdType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ThresholdConstraintsDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub threshold_id: Option<ThresholdIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub threshold_range_min: Option<commondatatypes::ScaledNumberType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub threshold_range_max: Option<commondatatypes::ScaledNumberType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub threshold_step_size: Option<commondatatypes::ScaledNumberType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ThresholdConstraintsDataElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub threshold_id: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub threshold_range_min: Option<commondatatypes::ScaledNumberElementsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub threshold_range_max: Option<commondatatypes::ScaledNumberElementsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub threshold_step_size: Option<commondatatypes::ScaledNumberElementsType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ThresholdConstraintsListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub threshold_constraints_data: Option<Vec<ThresholdConstraintsDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ThresholdConstraintsListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub threshold_id: Option<ThresholdIdType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ThresholdDescriptionDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub threshold_id: Option<ThresholdIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub threshold_type: Option<ThresholdTypeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub unit: Option<commondatatypes::UnitOfMeasurementType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub scope_type: Option<commondatatypes::ScopeTypeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub label: Option<commondatatypes::LabelType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<commondatatypes::DescriptionType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ThresholdDescriptionDataElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub threshold_id: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub threshold_type: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub unit: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub scope_type: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub label: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<commondatatypes::ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ThresholdDescriptionListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub threshold_description_data: Option<Vec<ThresholdDescriptionDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ThresholdDescriptionListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub threshold_id: Option<ThresholdIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub scope_type: Option<commondatatypes::ScopeTypeType>,
}

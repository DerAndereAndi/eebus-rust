use serde::{Serialize, Deserialize};
use super::commondatatypes;

pub type ThresholdIdType = u8;

pub type ThresholdTypeType = ThresholdTypeEnumType;

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ThresholdDataType {
  pub threshold_id: Option<ThresholdIdType>,
  pub threshold_value: Option<commondatatypes::ScaledNumberType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ThresholdListDataType {
  pub threshold_data: Option<Vec<ThresholdDataType>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ThresholdListDataSelectorsType {
  pub threshold_id: Option<ThresholdIdType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ThresholdConstraintsDataType {
  pub threshold_id: Option<ThresholdIdType>,
  pub threshold_range_min: Option<commondatatypes::ScaledNumberType>,
  pub threshold_range_max: Option<commondatatypes::ScaledNumberType>,
  pub threshold_step_size: Option<commondatatypes::ScaledNumberType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ThresholdConstraintsDataElementsType {
  pub threshold_id: Option<ThresholdIdType>,
  pub threshold_range_min: Option<commondatatypes::ScaledNumberElementsType>,
  pub threshold_range_max: Option<commondatatypes::ScaledNumberElementsType>,
  pub threshold_step_size: Option<commondatatypes::ScaledNumberElementsType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ThresholdConstraintsListDataType {
  pub threshold_constraints_data: Option<Vec<ThresholdConstraintsDataType>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ThresholdConstraintsListDataSelectorsType {
  pub threshold_id: Option<ThresholdIdType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ThresholdDescriptionDataType {
  pub threshold_id: Option<ThresholdIdType>,
  pub threshold_type: Option<ThresholdTypeType>,
  pub unit: Option<commondatatypes::UnitOfMeasurementType>,
  pub scope_type: Option<commondatatypes::ScopeTypeType>,
  pub label: Option<commondatatypes::LabelType>,
  pub description: Option<commondatatypes::DescriptionType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ThresholdDescriptionDataElementsType {
  pub threshold_id: Option<commondatatypes::ElementTagType>,
  pub threshold_type: Option<commondatatypes::ElementTagType>,
  pub unit: Option<commondatatypes::ElementTagType>,
  pub scope_type: Option<commondatatypes::ElementTagType>,
  pub label: Option<commondatatypes::ElementTagType>,
  pub description: Option<commondatatypes::ElementTagType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ThresholdDescriptionListDataType {
  pub threshold_description_data: Option<Vec<ThresholdDescriptionDataType>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ThresholdDescriptionListDataSelectorsType {
  pub threshold_id: Option<ThresholdIdType>,
  pub scope_type: Option<commondatatypes::ScopeTypeType>,
}

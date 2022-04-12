use std::fmt;

use super::super::utils;

use serde::{Serialize, Deserialize};
use super::{commondatatypes, measurement, timetable};

pub type SetpointIdType = u32;

pub type SetpointTypeType = SetpointTypeEnumType;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum SetpointTypeEnumType {
  ValueAbsolute,
  ValueRelative,
}

impl fmt::Display for SetpointTypeEnumType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		utils::provide_enum_display(self, f)
	}
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SetpointDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub setpoint_id: Option<SetpointIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub value: Option<commondatatypes::ScaledNumberType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub value_min: Option<commondatatypes::ScaledNumberType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub value_max: Option<commondatatypes::ScaledNumberType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub value_tolerance_absolute: Option<commondatatypes::ScaledNumberType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub value_tolerance_percentage: Option<commondatatypes::ScaledNumberType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SetpointDataElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub setpoint_id: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub setpoint_type: Option<commondatatypes::ScaledNumberElementsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub value: Option<commondatatypes::ScaledNumberElementsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub value_min: Option<commondatatypes::ScaledNumberElementsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub value_max: Option<commondatatypes::ScaledNumberElementsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub value_tolerance_absolute: Option<commondatatypes::ScaledNumberElementsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub value_tolerance_percentage: Option<commondatatypes::ScaledNumberElementsType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SetpointListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub setpoint_data: Option<Vec<SetpointDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SetpointListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub setpoint_id: Option<SetpointIdType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SetpointConstraintsDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub setpoint_id: Option<SetpointIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub setpoint_range_min: Option<commondatatypes::ScaledNumberType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub setpoint_range_max: Option<commondatatypes::ScaledNumberType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub setpoint_step_size: Option<commondatatypes::ScaledNumberType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SetpointConstraintsDataElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub setpoint_id: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub setpoint_range_min: Option<commondatatypes::ScaledNumberElementsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub setpoint_range_max: Option<commondatatypes::ScaledNumberElementsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub setpoint_step_size: Option<commondatatypes::ScaledNumberElementsType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SetpointConstraintsListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub setpoint_constraints_data: Option<Vec<SetpointConstraintsDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SetpointConstraintsListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub setpoint_id: Option<SetpointIdType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SetpointDescriptionDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub setpoint_id: Option<SetpointIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub measurement_id: Option<measurement::MeasurementIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub time_table_id: Option<timetable::TimeTableIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub setpoint_type: Option<SetpointTypeType>,
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
pub struct SetpointDescriptionDataElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub setpoint_id: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub measurement_id: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub time_table_id: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub setpoint_type: Option<commondatatypes::ElementTagType>,
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
pub struct SetpointDescriptionListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub setpoint_description_data: Option<Vec<SetpointDescriptionDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SetpointDescriptionListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub setpoint_id: Option<SetpointIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub measurement_id: Option<measurement::MeasurementIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub time_table_id: Option<timetable::TimeTableIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub setpoint_type: Option<SetpointTypeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub scope_type: Option<commondatatypes::ScopeTypeType>,
}

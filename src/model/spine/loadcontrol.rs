use serde::{Serialize, Deserialize};
use super::commondatatypes;
use super::measurement;

pub type LoadControlEventIdType = String;

pub type LoadControlEventActionType = LoadControlEventActionEnumType;

#[derive(Serialize, Deserialize, Debug)]
pub enum LoadControlEventActionEnumType {
	#[serde(rename = "pause")]
	Pause,
	#[serde(rename = "resume")]
	Resume,
	#[serde(rename = "reduce")]
	Reduce,
	#[serde(rename = "increase")]
	Increase,
	#[serde(rename = "emergency")]
	Emergency,
	#[serde(rename = "normal")]
	Normal,
}

pub type LoadControlEventStateType = LoadControlEventStateEnumType;

#[derive(Serialize, Deserialize, Debug)]
pub enum LoadControlEventStateEnumType {
	#[serde(rename = "eventAccepted")]
	EventAccepted,
	#[serde(rename = "eventStarted")]
	EventStarted,
	#[serde(rename = "eventStopped")]
	EventStopped,
	#[serde(rename = "eventRejected")]
	EventRejected,
	#[serde(rename = "eventCancelled")]
	EventCancelled,
	#[serde(rename = "eventError")]
	EventError,
}

pub type LoadControlLimitIdType = u8;

pub type LoadControlLimitTypeType = LoadControlLimitTypeEnumType;

#[derive(Serialize, Deserialize, Debug)]
pub enum LoadControlLimitTypeEnumType {
	#[serde(rename = "minValueLimit")]
	MinValueLimit,
	#[serde(rename = "maxValueLimit")]
	MaxValueLimit,
}

pub type LoadControlCategoryType = LoadControlCategoryEnumType;

#[derive(Serialize, Deserialize, Debug)]
pub enum LoadControlCategoryEnumType {
	#[serde(rename = "obligation")]
	Obligation,
	#[serde(rename = "recommendation")]
	Recommendation,
	#[serde(rename = "optimization")]
	Optimization,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoadControlNodeDataType {
  pub is_node_remote_controllable: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoadControlEventDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub timestamp: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub event_id: Option<LoadControlEventIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub event_action_consume: Option<LoadControlEventActionType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub event_action_produce: Option<LoadControlEventActionType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub time_period: Option<commondatatypes::TimePeriodType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoadControlEventListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub load_control_event_data: Option<Vec<LoadControlEventDataType>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoadControlEventListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub timestamp_inverval: Option<commondatatypes::TimestampIntervalType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub event_id: Option<LoadControlEventIdType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoadControlStateDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub timestamp: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub event_id: Option<LoadControlEventIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub event_state_consume: Option<LoadControlEventStateType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub applied_event_action_consume: Option<LoadControlEventActionType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub event_state_produce: Option<LoadControlEventStateType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub applied_event_action_produce: Option<LoadControlEventActionType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoadControlStateListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub load_control_state_data: Option<Vec<LoadControlStateDataType>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoadControlStateListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub timestamp_inverval: Option<commondatatypes::TimestampIntervalType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub event_id: Option<LoadControlEventIdType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoadControlLimitDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub limit_id: Option<LoadControlLimitIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub is_limit_changeable: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub is_limit_active: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub time_period: Option<commondatatypes::TimePeriodType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub value: Option<commondatatypes::ScaledNumberType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoadControlLimitListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub load_control_limit_data: Option<Vec<LoadControlLimitDataType>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoadControlLimitListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub limit_id: Option<LoadControlLimitIdType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoadControlLimitConstraintsDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub limit_id: Option<LoadControlLimitIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub value_range_min: Option<commondatatypes::ScaledNumberType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub value_range_max: Option<commondatatypes::ScaledNumberType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub value_step_size: Option<commondatatypes::ScaledNumberType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoadControlLimitConstraintsListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub load_control_limit_constraints_data: Option<Vec<LoadControlLimitConstraintsDataType>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoadControlLimitConstraintsListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub limit_id: Option<LoadControlLimitIdType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoadControlLimitDescriptionDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub limit_id: Option<LoadControlLimitIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub limit_type: Option<LoadControlLimitTypeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub limit_category: Option<LoadControlCategoryType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub limit_direction: Option<commondatatypes::EnergyDirectionType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub measurement_id: Option<measurement::MeasurementIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub unit: Option<commondatatypes::UnitOfMeasurementType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub scope_type: Option<commondatatypes::ScopeTypeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub label: Option<commondatatypes::LabelType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<commondatatypes::DescriptionType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoadControlLimitDescriptionListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub load_control_limit_description_data: Option<Vec<LoadControlLimitDescriptionDataType>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoadControlLimitDescriptionListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub limit_id: Option<LoadControlLimitIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub limit_type: Option<LoadControlLimitTypeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub limit_direction: Option<commondatatypes::EnergyDirectionType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub measurement_id: Option<measurement::MeasurementIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub scope_type: Option<commondatatypes::ScopeTypeType>,
}

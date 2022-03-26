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
pub enum LoadControlEventStateEnumType {
	#[serde(rename = "minValueLimit")]
	MinValueLimit,
	#[serde(rename = "maxValueLimit")]
	MaxValueLimit,
}

pub type LoadControlCategoryType = LoadControlCategoryEnumType;

#[derive(Serialize, Deserialize, Debug)]
pub enum LoadControlEventStateEnumType {
	#[serde(rename = "obligation")]
	Obligation,
	#[serde(rename = "recommendation")]
	Recommendation,
	#[serde(rename = "optimization")]
	Optimization,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoadControlNodeDataType struct {
  pub is_node_remote_controllable: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoadControlEventDataType struct {
  pub timestamp: Option<String>,
  pub event_id: Option<LoadControlEventIdType>,
  pub event_action_consume: Option<LoadControlEventActionType>,
  pub event_action_produce: Option<LoadControlEventActionType>,
  pub time_period: Option<commondatatypes::TimePeriodType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoadControlEventListDataType struct {
  pub load_control_event_data: Option<Vec<LoadControlEventDataType>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoadControlEventListDataSelectorsType struct {
  pub timestamp_inverval: Option<commondatatypes::TimestampIntervalType>,
  pub event_id: Option<LoadControlEventIdType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoadControlStateDataType struct {
  pub timestamp: Option<String>,
  pub event_id: Option<LoadControlEventIdType>,
  pub event_state_consume: Option<LoadControlEventStateType>,
  pub applied_event_action_consume: Option<LoadControlEventActionType>,
  pub event_state_produce: Option<LoadControlEventStateType>,
  pub applied_event_action_produce: Option<LoadControlEventActionType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoadControlStateListDataType struct {
  pub load_control_state_data: Option<Vec<LoadControlStateDataType>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoadControlStateListDataSelectorsType struct {
  pub timestamp_inverval: Option<commondatatypes::TimestampIntervalType>,
  pub event_id: Option<LoadControlEventIdType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoadControlLimitDataType struct {
  pub limit_id: Option<LoadControlLimitIdType>,
  pub is_limit_changeable: Option<bool>,
  pub is_limit_active: Option<bool>,
  pub time_period: Option<commondatatypes::TimePeriodType>,
  pub value: Option<commondatatypes::ScaledNumberType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoadControlLimitListDataType struct {
  pub load_control_limit_data: Option<Vec<LoadControlLimitDataType>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoadControlLimitListDataSelectorsType struct {
  pub limit_id: Option<LoadControlLimitIdType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoadControlLimitConstraintsDataType struct {
  pub limit_id: Option<LoadControlLimitIdType>,
  pub value_range_min: Option<commondatatypes::ScaledNumberType>,
  pub value_range_max: Option<commondatatypes::ScaledNumberType>,
  pub value_step_size: Option<commondatatypes::ScaledNumberType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoadControlLimitConstraintsListDataType struct {
  pub load_control_limit_constraints_data: Option<Vec<LoadControlLimitConstraintsDataType>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoadControlLimitConstraintsListDataSelectorsType struct {
  pub limit_id: Option<LoadControlLimitIdType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoadControlLimitDescriptionDataType struct {
  pub limit_id: Option<LoadControlLimitIdType>,
  pub limit_type: Option<LoadControlLimitTypeType>,
  pub limit_category: Option<LoadControlCategoryType>,
  pub limit_direction: Option<commondatatypes::EnergyDirectionType>,
  pub measurement_id: Option<measurement::MeasurementIdType>,
  pub unit: Option<commondatatypes::UnitOfMeasurementType>,
  pub scope_type: Option<commondatatypes::ScopeType>,
  pub label: Option<commondatatypes::LabelType>,
  pub description: Option<commondatatypes::DescriptionType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoadControlLimitDescriptionListDataType struct {
  pub load_control_limit_description_data: Option<Vec<LoadControlLimitDescriptionDataType>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct LoadControlLimitDescriptionListDataSelectorsType struct {
  pub limit_id: Option<LoadControlLimitIdType>,
  pub limit_type: Option<LoadControlLimitTypeType>,
  pub limit_direction: Option<commondatatypes::EnergyDirectionType>,
  pub measurement_id: Option<measurement::MeasurementIdType>,
  pub scope_type: Option<commondatatypes::ScopeType>,
}

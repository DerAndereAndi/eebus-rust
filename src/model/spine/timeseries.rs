use std::fmt;

use super::super::utils;

use serde::{Serialize, Deserialize};
use super::{commondatatypes, measurement};

pub type TimeSeriesIdType = u32;

pub type TimeSeriesSlotIdType = u32;

pub type TimeSeriesSlotCountType = TimeSeriesSlotIdType;

pub type TimeSeriesTypeType = TimeSeriesTypeEnumType;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum TimeSeriesTypeEnumType {
	#[serde(rename = "plan")]
	Plan,
	#[serde(rename = "singleDemand")]
	SingleDemand,
	#[serde(rename = "constraints")]
	Constraints,
}

impl fmt::Display for TimeSeriesTypeEnumType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		utils::provide_enum_display(self, f)
	}
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeSeriesSlotType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub time_series_slot_id: Option<TimeSeriesSlotIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub time_period: Option<commondatatypes::TimePeriodType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub duration: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub recurrence_information: Option<commondatatypes::AbsoluteOrRecurringTimeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub value: Option<commondatatypes::ScaledNumberType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub min_value: Option<commondatatypes::ScaledNumberType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub max_value: Option<commondatatypes::ScaledNumberType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeSeriesDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub time_series_id: Option<TimeSeriesIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub time_period: Option<commondatatypes::TimePeriodType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub time_series_slot: Option<Vec<TimeSeriesSlotType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeSeriesListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub time_series_data: Option<Vec<TimeSeriesDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeSeriesListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub time_series_id: Option<TimeSeriesIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub time_series_slot_id: Option<TimeSeriesSlotIdType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeSeriesDescriptionDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub time_series_id: Option<TimeSeriesIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub time_series_type: Option<TimeSeriesTypeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub time_series_writeable: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub update_required: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub measurement_id: Option<measurement::MeasurementIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub currency: Option<commondatatypes::CurrencyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub unit: Option<commondatatypes::UnitOfMeasurementType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub label: Option<commondatatypes::LabelType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<commondatatypes::DescriptionType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub scope_type: Option<commondatatypes::ScopeTypeType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeSeriesDescriptionListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub time_series_description_data: Option<Vec<TimeSeriesDescriptionDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeSeriesDescriptionListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub time_series_id: Option<TimeSeriesIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub time_series_type: Option<TimeSeriesTypeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub measurement_id: Option<measurement::MeasurementIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub scope_type: Option<commondatatypes::ScopeTypeType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeSeriesConstraintsDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub time_series_id: Option<TimeSeriesIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub slot_count_min: Option<TimeSeriesSlotCountType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub slot_count_max: Option<TimeSeriesSlotCountType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub slot_duration_min: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub slot_duration_max: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub slot_duration_step_size: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub earliest_time_series_start_time: Option<commondatatypes::AbsoluteOrRelativeTimeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub latest_time_series_end_time: Option<commondatatypes::AbsoluteOrRelativeTimeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub slot_value_min: Option<commondatatypes::ScaledNumberType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub slot_value_max: Option<commondatatypes::ScaledNumberType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub slot_value_step_size: Option<commondatatypes::ScaledNumberType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeSeriesConstraintsListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub time_series_constraints_data: Option<Vec<TimeSeriesConstraintsDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeSeriesConstraintsListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub time_series_id: Option<TimeSeriesIdType>,
}

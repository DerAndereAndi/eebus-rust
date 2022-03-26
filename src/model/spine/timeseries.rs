use serde::{Serialize, Deserialize};
use super::{commondatatypes, measurement};

pub type TimeSeriesIdType = u8;

pub type TimeSeriesSlotIdType = u8;

pub type TimeSeriesSlotCountType = TimeSeriesSlotIdType;

pub type TimeSeriesTypeType = TimeSeriesTypeEnumType;

#[derive(Serialize, Deserialize, Debug)]
pub enum TimeSeriesTypeEnumType {
	#[serde(rename = "plan")]
	Plan,
	#[serde(rename = "singleDemand")]
	SingleDemand,
	#[serde(rename = "constraints")]
	Constraints,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeSeriesSlotType {
  pub time_series_slot_id: Option<TimeSeriesSlotIdType>,
  pub time_period: Option<commondatatypes::TimePeriodType>,
  pub duration: Option<String>,
  pub recurrence_information: Option<commondatatypes::AbsoluteOrRecurringTimeType>,
  pub value: Option<commondatatypes::ScaledNumberType>,
  pub min_value: Option<commondatatypes::ScaledNumberType>,
  pub max_value: Option<commondatatypes::ScaledNumberType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeSeriesDataType {
  pub time_series_id: Option<TimeSeriesIdType>,
  pub time_period: Option<commondatatypes::TimePeriodType>,
  pub time_series_slot: Option<Vec<TimeSeriesSlotType>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeSeriesListDataType {
  pub time_series_data: Option<Vec<TimeSeriesDataType>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeSeriesListDataSelectorsType {
  pub time_series_id: Option<TimeSeriesIdType>,
  pub time_series_slot_id: Option<TimeSeriesSlotIdType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeSeriesDescriptionDataType {
  pub time_series_id: Option<TimeSeriesIdType>,
  pub time_series_type: Option<TimeSeriesTypeType>,
  pub time_series_writeable: Option<bool>,
  pub update_required: Option<bool>,
  pub measurement_id: Option<measurement::MeasurementIdType>,
  pub currency: Option<commondatatypes::CurrencyType>,
  pub unit: Option<commondatatypes::UnitOfMeasurementType>,
  pub label: Option<commondatatypes::LabelType>,
  pub description: Option<commondatatypes::DescriptionType>,
  pub scope_type: Option<commondatatypes::ScopeTypeType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeSeriesDescriptionListDataType {
  pub time_series_description_data: Option<Vec<TimeSeriesDescriptionDataType>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeSeriesDescriptionListDataSelectorsType {
  pub time_series_id: Option<TimeSeriesIdType>,
  pub time_series_type: Option<TimeSeriesTypeType>,
  pub measurement_id: Option<measurement::MeasurementIdType>,
  pub scope_type: Option<commondatatypes::ScopeTypeType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeSeriesConstraintsDataType {
  pub time_series_id: Option<TimeSeriesIdType>,
  pub slot_count_min: Option<TimeSeriesSlotCountType>,
  pub slot_count_max: Option<TimeSeriesSlotCountType>,
  pub slot_duration_min: Option<String>,
  pub slot_duration_max: Option<String>,
  pub slot_duration_step_size: Option<String>,
  pub earliest_time_series_start_time: Option<commondatatypes::AbsoluteOrRelativeTimeType>,
  pub latest_time_series_end_time: Option<commondatatypes::AbsoluteOrRelativeTimeType>,
  pub slot_value_min: Option<commondatatypes::ScaledNumberType>,
  pub slot_value_max: Option<commondatatypes::ScaledNumberType>,
  pub slot_value_step_size: Option<commondatatypes::ScaledNumberType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeSeriesConstraintsListDataType {
  pub time_series_constraints_data: Option<Vec<TimeSeriesConstraintsDataType>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeSeriesConstraintsListDataSelectorsType {
  pub time_series_id: Option<TimeSeriesIdType>,
}

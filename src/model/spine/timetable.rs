use serde::{Serialize, Deserialize};
use super::commondatatypes;

pub type TimeTableIdType = u8;

pub type TimeSlotIdType = u8;

pub type TimeSlotCountType = TimeSlotIdType;

pub type TimeSlotTimeModeType = TimeSlotTimeModeEnumType;

#[derive(Serialize, Deserialize, Debug)]
pub enum TimeSlotTimeModeEnumType {
	#[serde(rename = "absolute")]
	Absolute,
	#[serde(rename = "recurring")]
	Recurring,
	#[serde(rename = "both")]
	Both,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeTableDataType {
  pub time_table_id: Option<TimeTableIdType>,
  pub time_slot_id: Option<TimeSlotIdType>,
  pub recurrence_information: Option<commondatatypes::RecurrenceInformationType>,
  pub start_time: Option<commondatatypes::AbsoluteOrRecurringTimeType>,
  pub end_time: Option<commondatatypes::AbsoluteOrRecurringTimeType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeTableListDataType {
  pub time_table_data: Option<Vec<TimeTableDataType>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeTableListDataSelectorsType {
  pub time_table_id: Option<TimeTableIdType>,
  pub time_slot_id: Option<TimeSlotIdType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeTableConstraintsDataType {
  pub time_table_id: Option<TimeTableIdType>,
  pub slot_count_min: Option<TimeSlotCountType>,
  pub slot_count_max: Option<TimeSlotCountType>,
  pub slot_duration_min: Option<String>,
  pub slot_duration_max: Option<String>,
  pub slot_duration_step_size: Option<String>,
  pub slot_shift_step_size: Option<String>,
  pub first_slot_begins_at: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeTableConstraintsListDataType {
  pub time_table_constraints_data: Option<Vec<TimeTableConstraintsDataType>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeTableConstraintsListDataSelectorsType {
  pub time_table_id: Option<TimeTableIdType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeTableDescriptionDataType {
  pub time_table_id: Option<TimeTableIdType>,
  pub time_slot_count_changeable: Option<bool>,
  pub time_slot_times_changeable: Option<bool>,
  pub time_slot_time_mode: Option<TimeSlotTimeModeType>,
  pub label: Option<commondatatypes::LabelType>,
  pub description: Option<commondatatypes::DescriptionType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeTableDescriptionListDataType {
  pub time_table_description_data: Option<Vec<TimeTableDescriptionDataType>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeTableDescriptionListDataSelectorsType {
  pub time_table_id: Option<TimeTableIdType>,
}

use serde::{Serialize, Deserialize};
use super::commondatatypes;

pub type TimeTableIdType = u8;

pub type TimeSlotIdType = u8;

pub type TimeSlotCountType = TimeSlotIdType;

pub type TimeSlotTimeModeType = TimeSlotTimeModeEnumType;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum TimeSlotTimeModeEnumType {
	#[serde(rename = "absolute")]
	Absolute,
	#[serde(rename = "recurring")]
	Recurring,
	#[serde(rename = "both")]
	Both,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeTableDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub time_table_id: Option<TimeTableIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub time_slot_id: Option<TimeSlotIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub recurrence_information: Option<commondatatypes::RecurrenceInformationType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub start_time: Option<commondatatypes::AbsoluteOrRecurringTimeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub end_time: Option<commondatatypes::AbsoluteOrRecurringTimeType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeTableListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub time_table_data: Option<Vec<TimeTableDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeTableListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub time_table_id: Option<TimeTableIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub time_slot_id: Option<TimeSlotIdType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeTableConstraintsDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub time_table_id: Option<TimeTableIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub slot_count_min: Option<TimeSlotCountType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub slot_count_max: Option<TimeSlotCountType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub slot_duration_min: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub slot_duration_max: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub slot_duration_step_size: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub slot_shift_step_size: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub first_slot_begins_at: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeTableConstraintsListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub time_table_constraints_data: Option<Vec<TimeTableConstraintsDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeTableConstraintsListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub time_table_id: Option<TimeTableIdType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeTableDescriptionDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub time_table_id: Option<TimeTableIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub time_slot_count_changeable: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub time_slot_times_changeable: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub time_slot_time_mode: Option<TimeSlotTimeModeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub label: Option<commondatatypes::LabelType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<commondatatypes::DescriptionType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeTableDescriptionListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub time_table_description_data: Option<Vec<TimeTableDescriptionDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeTableDescriptionListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub time_table_id: Option<TimeTableIdType>,
}

use std::fmt;

use super::super::utils;

use serde::{Serialize, Deserialize};
use super::{commondatatypes,measurement};

pub type AlternativesIdType = u32;

pub type PowerSequenceIdType = u32;

pub type PowerTimeSlotNumberType = u32;

pub type PowerTimeSlotValueTypeType = PowerTimeSlotValueTypeEnumType;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum PowerTimeSlotValueTypeEnumType {
  Power,
  PowerMin,
  PowerMax,
  PowerExpectedValue,
  PowerStandardDeviation,
  PowerSkewness,
  Energy,
  EnergyMin,
  EnergyMax,
  EnergyExpectedValue,
  EnergyStandardDeviation,
  EnergySkewness,
}

impl fmt::Display for PowerTimeSlotValueTypeEnumType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		utils::provide_enum_display(self, f)
	}
}

pub type PowerSequenceScopeType = PowerSequenceScopeEnumType;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum PowerSequenceScopeEnumType {
  Forecast,
  Measurement,
  Recommendation,
}

impl fmt::Display for PowerSequenceScopeEnumType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		utils::provide_enum_display(self, f)
	}
}

pub type PowerSequenceStateType = PowerSequenceStateEnumType;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum PowerSequenceStateEnumType {
  Running,
  Paused,
  Scheduled,
  ScheduledPaused,
  Pending,
  Inactive,
  Completed,
  Invalid,
}

impl fmt::Display for PowerSequenceStateEnumType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		utils::provide_enum_display(self, f)
	}
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PowerTimeSlotScheduleDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub sequence_id: Option<PowerSequenceIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub slot_number: Option<PowerTimeSlotNumberType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub time_period: Option<commondatatypes::TimePeriodType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub default_duration: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub duration_uncertainty: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub slot_activated: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<commondatatypes::DescriptionType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PowerTimeSlotScheduleDataElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub sequence_id: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub slot_number: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub time_period: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub default_duration: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub duration_uncertainty: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub slot_activated: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<commondatatypes::ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PowerTimeSlotScheduleListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub power_time_slot_schedule_data: Option<Vec<PowerTimeSlotScheduleDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PowerTimeSlotScheduleListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub sequence_id: Option<PowerSequenceIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub slot_number: Option<PowerTimeSlotNumberType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PowerTimeSlotValueDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub sequence_id: Option<PowerSequenceIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub slot_number: Option<PowerTimeSlotNumberType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub value_type: Option<PowerTimeSlotValueTypeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub value: Option<commondatatypes::ScaledNumberType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PowerTimeSlotValueDataElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub sequence_id: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub slot_number: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub value_type: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub value: Option<commondatatypes::ScaledNumberElementsType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PowerTimeSlotValueListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub power_time_slot_value_data: Option<Vec<PowerTimeSlotValueDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PowerTimeSlotValueListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub sequence_id: Option<PowerSequenceIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub slot_number: Option<PowerTimeSlotNumberType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub value_type: Option<PowerTimeSlotValueTypeType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PowerTimeSlotScheduleConstraintsDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub sequence_id: Option<PowerSequenceIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub slot_number: Option<PowerTimeSlotNumberType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub earliest_start_time: Option<commondatatypes::AbsoluteOrRelativeTimeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub latest_end_time: Option<commondatatypes::AbsoluteOrRelativeTimeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub min_duration: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub max_duration: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub optional_slot: Option<bool>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PowerTimeSlotScheduleConstraintsDataElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub sequence_id: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub slot_number: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub earliest_start_time: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub latest_end_time: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub min_duration: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub max_duration: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub optional_slot: Option<commondatatypes::ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PowerTimeSlotScheduleConstraintsListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub power_time_slot_schedule_constraints_data: Option<Vec<PowerTimeSlotScheduleConstraintsDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PowerTimeSlotScheduleConstraintsListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub sequence_id: Option<PowerSequenceIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub slot_number: Option<PowerTimeSlotNumberType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PowerSequenceAlternativesRelationDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub alternative_id: Option<AlternativesIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub sequence_id: Option<Vec<PowerSequenceIdType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PowerSequenceAlternativesRelationDataElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub alternative_id: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub sequence_id: Option<commondatatypes::ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PowerSequenceAlternativesRelationListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub power_sequence_alternatives_relation_data: Option<Vec<PowerSequenceAlternativesRelationDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PowerSequenceAlternativesRelationListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub alternative_id: Option<AlternativesIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub sequence_id: Option<PowerSequenceIdType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PowerSequenceDescriptionDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub sequence_id: Option<PowerSequenceIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<commondatatypes::DescriptionType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub positive_energy_direction: Option<commondatatypes::EnergyDirectionType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub power_unit: Option<commondatatypes::UnitOfMeasurementType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub energy_unit: Option<commondatatypes::UnitOfMeasurementType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub value_source: Option<measurement::MeasurementValueSourceType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub scope: Option<PowerSequenceScopeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub task_identifier: Option<u32>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub repititions_total: Option<u32>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PowerSequenceDescriptionDataElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub sequence_id: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub positive_energy_direction: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub power_unit: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub energy_unit: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub value_source: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub scope: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub task_identifier: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub repititions_total: Option<commondatatypes::ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PowerSequenceDescriptionListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub power_sequence_description_data: Option<Vec<PowerSequenceDescriptionDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PowerSequenceDescriptionListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub sequence_id: Option<PowerSequenceIdType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PowerSequenceStateDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub sequence_id: Option<PowerSequenceIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub state: Option<PowerSequenceStateType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub active_slot_number: Option<PowerTimeSlotNumberType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub elapsed_slot_time: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub remaining_slot_time: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub sequence_remote_controllable: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub active_repitition_number: Option<u32>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub remaining_pause_time: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PowerSequenceStateDataElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub sequence_id: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub state: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub active_slot_number: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub elapsed_slot_time: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub remaining_slot_time: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub sequence_remote_controllable: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub active_repitition_number: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub remaining_pause_time: Option<commondatatypes::ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PowerSequenceStateListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub power_sequence_state_data: Option<Vec<PowerSequenceStateDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PowerSequenceStateListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub sequence_id: Option<PowerSequenceIdType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PowerSequenceScheduleDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub sequence_id: Option<PowerSequenceIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub start_time: Option<commondatatypes::AbsoluteOrRelativeTimeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub end_time: Option<commondatatypes::AbsoluteOrRelativeTimeType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PowerSequenceScheduleDataElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub sequence_id: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub start_time: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub end_time: Option<commondatatypes::ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PowerSequenceScheduleListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub power_sequence_schedule_data: Option<Vec<PowerSequenceScheduleDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PowerSequenceScheduleListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub sequence_id: Option<PowerSequenceIdType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PowerSequenceScheduleConstraintsDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub sequence_id: Option<PowerSequenceIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub earliest_start_time: Option<commondatatypes::AbsoluteOrRelativeTimeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub latest_start_time: Option<commondatatypes::AbsoluteOrRelativeTimeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub earliest_end_time: Option<commondatatypes::AbsoluteOrRelativeTimeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub latest_end_time: Option<commondatatypes::AbsoluteOrRelativeTimeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub optional_sequence: Option<bool>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PowerSequenceScheduleConstraintsDataElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub sequence_id: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub earliest_start_time: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub latest_start_time: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub earliest_end_time: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub latest_end_time: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub optional_sequence: Option<commondatatypes::ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PowerSequenceScheduleConstraintsListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub power_sequence_schedule_constraints_data: Option<Vec<PowerSequenceScheduleConstraintsDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PowerSequenceScheduleConstraintsListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub sequence_id: Option<PowerSequenceIdType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PowerSequencePriceDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub sequence_id: Option<PowerSequenceIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub potential_start_time: Option<commondatatypes::AbsoluteOrRelativeTimeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub price: Option<commondatatypes::ScaledNumberType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub currency: Option<commondatatypes::CurrencyType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PowerSequencePriceDataElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub sequence_id: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub potential_start_time: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub price: Option<commondatatypes::ScaledNumberElementsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub currency: Option<commondatatypes::ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PowerSequencePriceListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub power_sequence_price_data: Option<Vec<PowerSequencePriceDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PowerSequencePriceListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub sequence_id: Option<PowerSequenceIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub potential_start_time_interval: Option<commondatatypes::TimestampIntervalType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PowerSequenceSchedulePreferenceDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub sequence_id: Option<PowerSequenceIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub greenest: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub cheapest: Option<bool>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PowerSequenceSchedulePreferenceDataElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub sequence_id: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub greenest: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub cheapest: Option<commondatatypes::ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PowerSequenceSchedulePreferenceListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub power_sequence_schedule_preference_data: Option<Vec<PowerSequenceSchedulePreferenceDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PowerSequenceSchedulePreferenceListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub sequence_id: Option<PowerSequenceIdType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PowerSequenceNodeScheduleInformationDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub node_remote_controllable: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub supports_single_slot_scheduling_only: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub alternatives_count: Option<u32>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub total_sequences_count_max: Option<u32>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub supports_reselection: Option<bool>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PowerSequenceNodeScheduleInformationDataElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub node_remote_controllable: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub supports_single_slot_scheduling_only: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub alternatives_count: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub total_sequences_count_max: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub supports_reselection: Option<commondatatypes::ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PowerSequenceScheduleConfigurationRequestCallType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub sequence_id: Option<PowerSequenceIdType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PowerSequenceScheduleConfigurationRequestCallElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub sequence_id: Option<commondatatypes::ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PowerSequencePriceCalculationRequestCallType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub sequence_id: Option<PowerSequenceIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub potential_start_time: Option<commondatatypes::AbsoluteOrRelativeTimeType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PowerSequencePriceCalculationRequestCallElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub sequence_id: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub potential_start_time: Option<commondatatypes::ElementTagType>,
}

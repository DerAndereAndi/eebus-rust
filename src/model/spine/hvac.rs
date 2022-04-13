use std::fmt;

use super::super::utils;

use serde::{Serialize, Deserialize};
use super::{commondatatypes, setpoint, powersequences, timetable};

pub type HvacSystemFunctionIdType = u32;

pub type HvacSystemFunctionTypeType = HvacSystemFunctionTypeEnumType;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum HvacSystemFunctionTypeEnumType {
  Heating,
  Cooling,
  Ventilation,
  Dhw,
}

impl fmt::Display for HvacSystemFunctionTypeEnumType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		utils::provide_enum_display(self, f)
	}
}

pub type HvacOperationModeIdType = u32;

pub type HvacOperationModeTypeType = HvacOperationModeTypeEnumType;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum HvacOperationModeTypeEnumType {
  Auto,
  On,
  Off,
  Eco,
}

impl fmt::Display for HvacOperationModeTypeEnumType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		utils::provide_enum_display(self, f)
	}
}

pub type HvacOverrunIdType = u32;

pub type HvacOverrunTypeType = HvacOverrunTypeEnumType;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum HvacOverrunTypeEnumType {
  OneTimeDhw,
  Party,
  SgReadyCondition1,
  SgReadyCondition3,
  SgReadyCondition4,
  OneDayAway,
  OneDayAtHome,
  OneTimeVentilation,
  HvacSystemOff,
  ValveKick,
}

impl fmt::Display for HvacOverrunTypeEnumType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		utils::provide_enum_display(self, f)
	}
}

pub type HvacOverrunStatusType = HvacOverrunStatusEnumType;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum HvacOverrunStatusEnumType {
  Active,
  Running,
  Finished,
  Inactive,
}

impl fmt::Display for HvacOverrunStatusEnumType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		utils::provide_enum_display(self, f)
	}
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HvacSystemFunctionDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub system_function_id: Option<HvacSystemFunctionIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub current_operation_mode_id: Option<HvacOperationModeIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub is_operation_mode_id_changeable: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub current_setpoint_id: Option<setpoint::SetpointIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub is_setpoint_id_changeable: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub is_overrun_active: Option<bool>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HvacSystemFunctionDataElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub system_function_id: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub current_operation_mode_id: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub is_operation_mode_id_changeable: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub current_setpoint_id: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub is_setpoint_id_changeable: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub is_overrun_active: Option<commondatatypes::ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HvacSystemFunctionListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub hvac_system_function_list_data: Option<Vec<HvacSystemFunctionDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HvacSystemFunctionListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub system_function_id: Option<Vec<HvacSystemFunctionIdType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HvacSystemFunctionOperationModeRelationDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub system_function_id: Option<HvacSystemFunctionIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub operation_mode_id: Option<Vec<HvacOperationModeIdType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HvacSystemFunctionOperationModeRelationDataElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub system_function_id: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub operation_mode_id: Option<commondatatypes::ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HvacSystemFunctionOperationModeRelationListDataType {
  pub hvac_system_function_operation_mode_relation_list_data: Option<Vec<HvacSystemFunctionOperationModeRelationDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HvacSystemFunctionOperationModeRelationListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub system_function_id: Option<HvacSystemFunctionIdType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HvacSystemFunctionSetpointRelationDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub system_function_id: Option<HvacSystemFunctionIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub operation_mode_id: Option<HvacOperationModeIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub setpoint_id: Option<Vec<setpoint::SetpointIdType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HvacSystemFunctionSetpointRelationDataElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub system_function_id: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub operation_mode_id: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub setpoint_id: Option<commondatatypes::ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HvacSystemFunctionSetpointRelationListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub hvac_system_function_setpoint_relation_list_data: Option<Vec<HvacSystemFunctionSetpointRelationDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HvacSystemFunctionSetpointRelationListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub system_function_id: Option<HvacSystemFunctionIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub operation_mode_id: Option<HvacOperationModeIdType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HvacSystemFunctionPowerSequenceRelationDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub system_function_id: Option<HvacSystemFunctionIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub sequence_id: Option<Vec<powersequences::PowerSequenceIdType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HvacSystemFunctionPowerSequenceRelationDataElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub system_function_id: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub sequence_id: Option<commondatatypes::ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HvacSystemFunctionPowerSequenceRelationListDataType {
  pub hvac_system_function_power_sequence_relation_list_data: Option<Vec<HvacSystemFunctionPowerSequenceRelationDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HvacSystemFunctionPowerSequenceRelationListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub system_function_id: Option<HvacSystemFunctionIdType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HvacSystemFunctionDescriptionDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub system_function_id: Option<HvacSystemFunctionIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub system_function_type: Option<HvacSystemFunctionTypeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub label: Option<commondatatypes::LabelType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<commondatatypes::DescriptionType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HvacSystemFunctionDescriptionDataElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub system_function_id: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub system_function_type: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub label: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<commondatatypes::ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HvacSystemFunctionDescriptionListDataType {
  pub hvac_system_function_description_list_data: Option<Vec<HvacSystemFunctionDescriptionDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HvacSystemFunctionDescriptionListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub system_function_id: Option<HvacSystemFunctionIdType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HvacOperationModeDescriptionDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub operation_mode_id: Option<HvacOperationModeIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub operation_mode_type: Option<HvacOperationModeTypeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub label: Option<commondatatypes::LabelType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<commondatatypes::DescriptionType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HvacOperationModeDescriptionDataElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub operation_mode_id: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub operation_mode_type: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub label: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<commondatatypes::ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HvacOperationModeDescriptionListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub operation_mode_description_list_data: Option<Vec<HvacOperationModeDescriptionDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HvacOperationModeDescriptionListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub operation_mode_id: Option<HvacOperationModeIdType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HvacOverrunDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub overrun_id: Option<HvacOverrunIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub overrun_status: Option<HvacOverrunStatusType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub time_table_id: Option<timetable::TimeTableIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub is_overrun_status_changeable: Option<bool>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HvacOverrunDataElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub overrun_id: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub overrun_status: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub time_table_id: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub is_overrun_status_changeable: Option<commondatatypes::ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HvacOverrunListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub hvac_overrun_data: Option<Vec<HvacOverrunDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HvacOverrunListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub overrun_id: Option<HvacOverrunIdType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HvacOverrunDescriptionDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub overrun_id: Option<HvacOverrunIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub overrun_type: Option<HvacOverrunTypeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub affected_system_function_id: Option<Vec<HvacSystemFunctionIdType>>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub label: Option<commondatatypes::LabelType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<commondatatypes::DescriptionType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HvacOverrunDescriptionDataElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub overrun_id: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub overrun_type: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub affected_system_function_id: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub label: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<commondatatypes::ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HvacOverrunDescriptionListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub hvac_overrun_description_data: Option<Vec<HvacOverrunDescriptionDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HvacOverrunDescriptionListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub overrun_id: Option<HvacOverrunIdType>,
}

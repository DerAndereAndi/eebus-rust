use std::fmt;

use super::super::utils;

use serde::{Serialize, Deserialize};
use super::{commondatatypes, hvac, loadcontrol, powersequences};

pub type TaskManagementJobIdType = u32;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum TaskManagementJobStateType {
  // DirectControlActivityStateEnumType
  // HvacOverrunStatusEnumType
  // LoadControlEventStateEnumType
  // PowerSequenceStateEnumType
}

impl fmt::Display for TaskManagementJobStateType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		utils::provide_enum_display(self, f)
	}
}

pub type TaskManagementJobSourceType = TaskManagementJobSourceEnumType;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum TaskManagementJobSourceEnumType {
  InternalMechanism,
  UserInteraction,
  ExternalConfiguration,
}

impl fmt::Display for TaskManagementJobSourceEnumType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		utils::provide_enum_display(self, f)
	}
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TaskManagementDirectControlRelatedType {}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TaskManagementDirectControlRelatedElementsType {}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TaskManagementHvacRelatedType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub overrun_id: Option<hvac::HvacOverrunIdType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TaskManagementHvacRelatedElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub overrun_id: Option<commondatatypes::ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TaskManagementLoadControlReleatedType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub event_id: Option<loadcontrol::LoadControlEventIdType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TaskManagementLoadControlReleatedElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub event_id: Option<commondatatypes::ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TaskManagementPowerSequencesRelatedType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub sequence_id: Option<powersequences::PowerSequenceIdType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TaskManagementPowerSequencesRelatedElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub sequence_id: Option<commondatatypes::ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TaskManagementSmartEnergyManagementPsRelatedType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub sequence_id: Option<powersequences::PowerSequenceIdType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TaskManagementSmartEnergyManagementPsRelatedElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub sequence_id: Option<commondatatypes::ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TaskManagementJobDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub job_id: Option<TaskManagementJobIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub timestamp: Option<commondatatypes::AbsoluteOrRelativeTimeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub job_state: Option<TaskManagementJobStateType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub elapsed_time: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub remaining_time: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TaskManagementJobDataElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub job_id: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub timestamp: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub job_state: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub elapsed_time: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub remaining_time: Option<commondatatypes::ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TaskManagementJobListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub job_list_data: Option<Vec<TaskManagementJobDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TaskManagementJobListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub job_id: Option<TaskManagementJobIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub job_state: Option<TaskManagementJobStateType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TaskManagementJobRelationDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub job_id: Option<TaskManagementJobIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub direct_control_related: Option<TaskManagementDirectControlRelatedType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub hvac_related: Option<TaskManagementHvacRelatedType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub load_control_related: Option<TaskManagementLoadControlReleatedType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub power_sequences_related: Option<TaskManagementPowerSequencesRelatedType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub smart_energy_management_ps_related: Option<TaskManagementSmartEnergyManagementPsRelatedType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TaskManagementJobRelationDataElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub job_id: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub direct_control_related: Option<TaskManagementDirectControlRelatedElementsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub hvac_related: Option<TaskManagementHvacRelatedElementsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub load_control_related: Option<TaskManagementLoadControlReleatedElementsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub power_sequences_related: Option<TaskManagementPowerSequencesRelatedElementsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub smart_energy_management_ps_related: Option<TaskManagementSmartEnergyManagementPsRelatedElementsType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TaskManagementJobRelationListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub job_relation_list_data: Option<Vec<TaskManagementJobRelationDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TaskManagementJobRelationListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub job_id: Option<TaskManagementJobIdType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TaskManagementJobDescriptionDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub job_id: Option<TaskManagementJobIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub job_source: Option<TaskManagementJobSourceType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub label: Option<commondatatypes::LabelType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<commondatatypes::DescriptionType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TaskManagementJobDescriptionDataElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub job_id: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub job_source: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub label: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<commondatatypes::ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TaskManagementJobDescriptionListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub job_description_list_data: Option<Vec<TaskManagementJobDescriptionDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TaskManagementJobDescriptionListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub job_id: Option<TaskManagementJobIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub job_source: Option<TaskManagementJobSourceType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TaskManagementOverviewDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub remote_controllable: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub jobs_active: Option<bool>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TaskManagementOverviewDataElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub remote_controllable: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub jobs_active: Option<commondatatypes::ElementTagType>,
}

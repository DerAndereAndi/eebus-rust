use serde::{Serialize, Deserialize};
use super::{powersequences, operatingconstraints};

pub type SmartEnergyManagementPsAlternativesRelationType = powersequences::PowerSequenceAlternativesRelationDataType; // ignoring changes

pub type SmartEnergyManagementPsAlternativesRelationElementsType = powersequences::PowerSequenceAlternativesRelationDataElementsType; // ignoring changes

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SmartEnergyManagementPsAlternativesType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub relation: Option<SmartEnergyManagementPsAlternativesRelationType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub power_sequence: Option<Vec<SmartEnergyManagementPsPowerSequenceType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SmartEnergyManagementPsAlternativesElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub relation: Option<SmartEnergyManagementPsAlternativesRelationElementsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub power_sequence: Option<SmartEnergyManagementPsPowerSequenceElementsType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SmartEnergyManagementPsPowerSequenceType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<powersequences::PowerSequenceDescriptionDataType>, // ignoring changes
	#[serde(skip_serializing_if = "Option::is_none")]
  pub state: Option<powersequences::PowerSequenceStateDataType>, // ignoring changes
	#[serde(skip_serializing_if = "Option::is_none")]
  pub schedule: Option<powersequences::PowerSequenceScheduleDataType>, // ignoring changes
	#[serde(skip_serializing_if = "Option::is_none")]
  pub schedule_constraints: Option<powersequences::PowerSequenceScheduleConstraintsDataType>, // ignoring changes
	#[serde(skip_serializing_if = "Option::is_none")]
  pub schedule_preference: Option<powersequences::PowerSequenceSchedulePreferenceDataType>, // ignoring changes
	#[serde(skip_serializing_if = "Option::is_none")]
  pub operating_constraints_interrupt: Option<operatingconstraints::OperatingConstraintsInterruptDataType>, // ignoring changes
	#[serde(skip_serializing_if = "Option::is_none")]
  pub operating_constraints_duration: Option<operatingconstraints::OperatingConstraintsDurationDataType>, // ignoring changes
	#[serde(skip_serializing_if = "Option::is_none")]
  pub operating_constraints_resume_implication: Option<operatingconstraints::OperatingConstraintsResumeImplicationDataType>, // ignoring changes
	#[serde(skip_serializing_if = "Option::is_none")]
  pub power_time_slot: Option<Vec<SmartEnergyManagementPsPowerTimeSlotType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SmartEnergyManagementPsPowerSequenceElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<powersequences::PowerSequenceDescriptionDataElementsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub state: Option<powersequences::PowerSequenceStateDataElementsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub schedule: Option<powersequences::PowerSequenceScheduleDataElementsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub schedule_constraints: Option<powersequences::PowerSequenceScheduleConstraintsDataElementsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub schedule_preference: Option<powersequences::PowerSequenceSchedulePreferenceDataElementsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub operating_constraints_interrupt: Option<operatingconstraints::OperatingConstraintsInterruptDataElementsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub operating_constraints_duration: Option<operatingconstraints::OperatingConstraintsDurationDataElementsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub operating_constraints_resume_implication: Option<operatingconstraints::OperatingConstraintsResumeImplicationDataElementsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub power_time_slot: Option<SmartEnergyManagementPsPowerTimeSlotElementsType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SmartEnergyManagementPsPowerTimeSlotType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub schedule: Option<powersequences::PowerTimeSlotScheduleDataType>, // ignoring changes
	#[serde(skip_serializing_if = "Option::is_none")]
  pub value_list: Option<SmartEnergyManagementPsPowerTimeSlotValueListType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub schedule_constraints: Option<powersequences::PowerTimeSlotScheduleConstraintsDataType>, // ignoring changes
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SmartEnergyManagementPsPowerTimeSlotElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub schedule: Option<powersequences::PowerTimeSlotScheduleDataElementsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub value_list: Option<SmartEnergyManagementPsPowerTimeSlotValueListElementsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub schedule_constraints: Option<powersequences::PowerTimeSlotScheduleConstraintsDataElementsType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SmartEnergyManagementPsPowerTimeSlotValueListType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub value: Option<Vec<powersequences::PowerTimeSlotValueDataType>>, // ignoring changes
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SmartEnergyManagementPsPowerTimeSlotValueListElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub value: Option<powersequences::PowerTimeSlotValueDataElementsType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SmartEnergyManagementPsDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub node_schedule_information: Option<powersequences::PowerSequenceNodeScheduleInformationDataType>, // ignoring changes
	#[serde(skip_serializing_if = "Option::is_none")]
  pub alternatives: Option<Vec<SmartEnergyManagementPsAlternativesType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SmartEnergyManagementPsDataElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub node_schedule_information: Option<powersequences::PowerSequenceNodeScheduleInformationDataElementsType>, // ignoring changes
	#[serde(skip_serializing_if = "Option::is_none")]
  pub alternatives: Option<SmartEnergyManagementPsAlternativesElementsType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SmartEnergyManagementPsDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub alternatives_relation: Option<powersequences::PowerSequenceAlternativesRelationListDataSelectorsType>, // ignoring changes
	#[serde(skip_serializing_if = "Option::is_none")]
  pub power_sequence_description: Option<powersequences::PowerSequenceDescriptionListDataSelectorsType>, // ignoring changes
	#[serde(skip_serializing_if = "Option::is_none")]
  pub power_time_slot_schedule: Option<powersequences::PowerTimeSlotScheduleListDataSelectorsType>, // ignoring changes
	#[serde(skip_serializing_if = "Option::is_none")]
  pub power_time_slot_value: Option<powersequences::PowerTimeSlotValueListDataSelectorsType>, // ignoring changes
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SmartEnergyManagementPsPriceDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub price: Option<powersequences::PowerSequencePriceDataType>, // ignoring changes
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SmartEnergyManagementPsPriceDataElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub price: Option<powersequences::PowerSequencePriceDataElementsType>, // ignoring changes
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SmartEnergyManagementPsPriceDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub price: Option<powersequences::PowerSequencePriceListDataSelectorsType>, // ignoring changes
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SmartEnergyManagementPsConfigurationRequestCallType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub schedule_configuration_request: Option<powersequences::PowerSequenceScheduleConfigurationRequestCallType>, // ignoring changes
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SmartEnergyManagementPsConfigurationRequestCallElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub schedule_configuration_request: Option<powersequences::PowerSequenceScheduleConfigurationRequestCallElementsType>, // ignoring changes
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SmartEnergyManagementPsPriceCalculationRequestCallType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub price_calculation_request: Option<powersequences::PowerSequencePriceCalculationRequestCallType>, // ignoring changes
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SmartEnergyManagementPsPriceCalculationRequestCallElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub price_calculation_request: Option<powersequences::PowerSequencePriceCalculationRequestCallElementsType>, // ignoring changes
}

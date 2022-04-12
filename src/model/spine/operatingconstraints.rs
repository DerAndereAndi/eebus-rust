use serde::{Serialize, Deserialize};
use super::{commondatatypes,powersequences};

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OperatingConstraintsInterruptDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub sequence_id: Option<powersequences::PowerSequenceIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub is_pausable: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub is_stoppable: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub not_interruptible_at_high_power: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub max_cycles_per_day: Option<u32>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OperatingConstraintsInterruptDataElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub sequence_id: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub is_pausable: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub is_stoppable: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub not_interruptible_at_high_power: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub max_cycles_per_day: Option<commondatatypes::ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OperatingConstraintsInterruptListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub operating_constraints_interrupt_data: Option<Vec<OperatingConstraintsInterruptDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OperatingConstraintsInterruptListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub sequence_id: Option<powersequences::PowerSequenceIdType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OperatingConstraintsDurationDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub sequence_id: Option<powersequences::PowerSequenceIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub active_duration_min: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub active_duration_max: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub pause_duration_min: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub pause_duration_max: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub active_duration_sum_min: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub active_duration_sum_max: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OperatingConstraintsDurationDataElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub sequence_id: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub active_duration_min: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub active_duration_max: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub pause_duration_min: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub pause_duration_max: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub active_duration_sum_min: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub active_duration_sum_max: Option<commondatatypes::ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OperatingConstraintsDurationListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub operating_constraints_duration_data: Option<Vec<OperatingConstraintsDurationDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OperatingConstraintsDurationListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub sequence_id: Option<powersequences::PowerSequenceIdType>,
}


#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OperatingConstraintsPowerDescriptionDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub sequence_id: Option<powersequences::PowerSequenceIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub positive_energy_direction: Option<commondatatypes::EnergyDirectionType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub power_unit: Option<commondatatypes::UnitOfMeasurementType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub energy_unit: Option<commondatatypes::UnitOfMeasurementType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<commondatatypes::DescriptionType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OperatingConstraintsPowerDescriptionDataElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub sequence_id: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub positive_energy_direction: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub power_unit: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub energy_unit: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<commondatatypes::ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OperatingConstraintsPowerDescriptionListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub operating_constraints_power_description_data: Option<Vec<OperatingConstraintsPowerDescriptionDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OperatingConstraintsPowerDescriptionListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub sequence_id: Option<powersequences::PowerSequenceIdType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OperatingConstraintsPowerRangeDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub sequence_id: Option<powersequences::PowerSequenceIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub power_min: Option<commondatatypes::ScaledNumberType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub power_max: Option<commondatatypes::ScaledNumberType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub energy_min: Option<commondatatypes::ScaledNumberType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub energy_max: Option<commondatatypes::ScaledNumberType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OperatingConstraintsPowerRangeDataElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub sequence_id: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub power_min: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub power_max: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub energy_min: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub energy_max: Option<commondatatypes::ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OperatingConstraintsPowerRangeListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub operating_constraints_power_range_data: Option<Vec<OperatingConstraintsPowerRangeDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OperatingConstraintsPowerRangeListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub sequence_id: Option<powersequences::PowerSequenceIdType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OperatingConstraintsPowerLevelDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub sequence_id: Option<powersequences::PowerSequenceIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub power: Option<Vec<commondatatypes::ScaledNumberType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OperatingConstraintsPowerLevelDataElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub sequence_id: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub power: Option<commondatatypes::ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OperatingConstraintsPowerLevelListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub operating_constraints_power_level_data: Option<Vec<OperatingConstraintsPowerLevelDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OperatingConstraintsPowerLevelListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub sequence_id: Option<powersequences::PowerSequenceIdType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OperatingConstraintsResumeImplicationDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub sequence_id: Option<powersequences::PowerSequenceIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub resume_energy_estimated: Option<commondatatypes::ScaledNumberType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub energy_unit: Option<commondatatypes::UnitOfMeasurementType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub resume_cost_estimated: Option<commondatatypes::ScaledNumberType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub currency: Option<commondatatypes::CurrencyType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OperatingConstraintsResumeImplicationDataElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub sequence_id: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub resume_energy_estimated: Option<commondatatypes::ScaledNumberElementsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub energy_unit: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub resume_cost_estimated: Option<commondatatypes::ScaledNumberElementsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub currency: Option<commondatatypes::ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OperatingConstraintsResumeImplicationListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub operating_constraints_resume_implication_data: Option<Vec<OperatingConstraintsResumeImplicationDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct OperatingConstraintsResumeImplicationListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub sequence_id: Option<powersequences::PowerSequenceIdType>,
}

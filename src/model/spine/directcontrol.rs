use std::fmt;

use super::super::utils;

use serde::{Serialize, Deserialize};
use super::{commondatatypes, powersequences};

pub type DirectControlActivityStateType = DirectControlActivityStateEnumType;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum DirectControlActivityStateEnumType {
  Running,
  Paused,
  Inactive,
}

impl fmt::Display for DirectControlActivityStateEnumType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		utils::provide_enum_display(self, f)
	}
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DirectControlActivityDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub timestamp: Option<commondatatypes::AbsoluteOrRelativeTimeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub activity_state: Option<DirectControlActivityStateType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub is_activity_state_changeable: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub enery_mode: Option<commondatatypes::EnergyModeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub is_energy_mode_changeable: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub power: Option<commondatatypes::ScaledNumberType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub is_power_changeable: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub energy: Option<commondatatypes::ScaledNumberType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub is_energy_changeable: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub sequence_id: Option<powersequences::PowerSequenceIdType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DirectControlActivityDataElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub timestamp: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub activity_state: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub is_activity_state_changeable: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub enery_mode: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub is_energy_mode_changeable: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub power: Option<commondatatypes::ScaledNumberElementsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub is_power_changeable: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub energy: Option<commondatatypes::ScaledNumberElementsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub is_energy_changeable: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub sequence_id: Option<commondatatypes::ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DirectControlActivityListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  direct_control_activity_data: Option<Vec<DirectControlActivityDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DirectControlActivityListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  timestamp_interval: Option<commondatatypes::TimestampIntervalType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DirectControlDescriptionDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub positive_energy_direction: Option<commondatatypes::EnergyDirectionType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub power_unit: Option<commondatatypes::UnitOfMeasurementType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub energy_unit: Option<commondatatypes::UnitOfMeasurementType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DirectControlDescriptionDataElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub positive_energy_direction: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub power_unit: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub energy_unit: Option<commondatatypes::ElementTagType>,
}

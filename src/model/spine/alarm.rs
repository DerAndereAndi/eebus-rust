use std::fmt;

use super::super::utils;

use serde::{Serialize, Deserialize};
use super::{commondatatypes, threshold};

pub type AlarmIdType = u32;

pub type AlarmTypeType = AlarmTypeEnumType;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum AlarmTypeEnumType {
  AlarmCancelled,
  UnderThreshold,
  OverThreshold,
}

impl fmt::Display for AlarmTypeEnumType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		utils::provide_enum_display(self, f)
	}
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AlarmDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub alarm_id: Option<AlarmIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub threshold_id: Option<threshold::ThresholdIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub timestamp: Option<commondatatypes::AbsoluteOrRelativeTimeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub alarm_type: Option<AlarmTypeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub measured_value: Option<commondatatypes::ScaledNumberType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub evaluation_period: Option<commondatatypes::TimePeriodType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub scope_type: Option<commondatatypes::ScopeTypeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub label: Option<commondatatypes::LabelType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<commondatatypes::DescriptionType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AlarmDataElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub alarm_id: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub threshold_id: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub timestamp: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub alarm_type: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub measured_value: Option<commondatatypes::ScaledNumberElementsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub evaluation_period: Option<commondatatypes::TimePeriodElementsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub scope_type: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub label: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<commondatatypes::ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AlarmListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub alarm_data: Option<Vec<AlarmListDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AlarmListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub alarm_id: Option<AlarmIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub scope_type: Option<commondatatypes::ScopeTypeType>,
}

use std::fmt;

use super::super::utils;

use serde::{Serialize, Deserialize};
use super::{commondatatypes, threshold};

pub type ConditionIdType = u32;

pub type SupplyConditionEventTypeType = SupplyConditionEventTypeEnumType;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum SupplyConditionEventTypeEnumType {
  ThesholdExceeded,
  FallenBelowThreshold,
  SupplyInterrupt,
  ReleaseOfLimitations,
  OtherProblem,
  GridConditionUpdate,
}

impl fmt::Display for SupplyConditionEventTypeEnumType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		utils::provide_enum_display(self, f)
	}
}

pub type SupplyConditionOriginatorType = SupplyConditionOriginatorEnumType;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum SupplyConditionOriginatorEnumType {
  ExternDSO,
  ExternSupplier,
  InternalLimit,
  InternalService,
  InternalUser,
}

impl fmt::Display for SupplyConditionOriginatorEnumType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		utils::provide_enum_display(self, f)
	}
}

pub type GridConditionType = GridConditionEnumType;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum GridConditionEnumType {
  ConsumptionRed,
  ConsumptionYellow,
  Good,
  ProductionRed,
  ProductionYellow,
}

impl fmt::Display for GridConditionEnumType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		utils::provide_enum_display(self, f)
	}
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SupplyConditionDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub condition_id: Option<ConditionIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub timestamp: Option<commondatatypes::AbsoluteOrRelativeTimeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub event_type: Option<SupplyConditionEventTypeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub originator: Option<SupplyConditionOriginatorType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub threshold_id: Option<threshold::ThresholdIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub threashold_percentage: Option<commondatatypes::ScaledNumberType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub relevant_period: Option<commondatatypes::TimePeriodType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<commondatatypes::DescriptionType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub grid_condition: Option<GridConditionType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SupplyConditionDataElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub condition_id: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub timestamp: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub event_type: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub originator: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub threshold_id: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub threashold_percentage: Option<commondatatypes::ScaledNumberElementsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub relevant_period: Option<commondatatypes::TimePeriodElementsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub grid_condition: Option<commondatatypes::ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SupplyConditionListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub supply_condition_data: Option<Vec<SupplyConditionDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SupplyConditionListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub condition_id: Option<ConditionIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub timestamp_interval: Option<commondatatypes::TimestampIntervalType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub event_type: Option<SupplyConditionEventTypeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub originator: Option<SupplyConditionOriginatorType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SupplyConditionDescriptionDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub condition_id: Option<ConditionIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub commodity_type: Option<commondatatypes::CommodityTypeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub positive_energy_direction: Option<commondatatypes::EnergyDirectionType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub label: Option<commondatatypes::LabelType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<commondatatypes::DescriptionType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SupplyConditionDescriptionDataElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub condition_id: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub commodity_type: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub positive_energy_direction: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub label: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<commondatatypes::ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SupplyConditionDescriptionListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub supply_condition_description_data: Option<Vec<SupplyConditionDescriptionDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SupplyConditionDescriptionListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub condition_id: Option<ConditionIdType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SupplyConditionThresholdRelationDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub condition_id: Option<ConditionIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub threshold_id: Option<Vec<threshold::ThresholdIdType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SupplyConditionThresholdRelationDataElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub condition_id: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub threshold_id: Option<commondatatypes::ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SupplyConditionThresholdRelationListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub supply_condition_threshold_relation_data: Option<Vec<SupplyConditionThresholdRelationDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SupplyConditionThresholdRelationListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub condition_id: Option<ConditionIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub threshold_id: Option<threshold::ThresholdIdType>,
}

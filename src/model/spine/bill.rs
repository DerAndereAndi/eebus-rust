use std::fmt;

use super::super::utils;

use serde::{Serialize, Deserialize};
use super::{commondatatypes};

pub type BillIdType = u32;

pub type BillTypeType = BillTypeEnumType;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum BillTypeEnumType {
  ChargingSummary,
}

pub type BillPositionIdType = u32;

pub type BillPositionCountType = BillPositionIdType;

pub type BillPositionTypeType = BillPositionTypeEnumType;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum BillPositionTypeEnumType {
  GridElectricEnergy,
  SelfProducedElectricEnergy,
}

pub type BillValueIdType = u32;

pub type BillCostIdType = u32;

pub type BillCostTypeType = BillCostTypeEnumType;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum BillCostTypeEnumType {
  AbsolutePrice,
  RelativePrice,
  Co2Emission,
  RenewableEnergy,
  RadioactiveWaste,
}

impl fmt::Display for BillCostTypeEnumType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		utils::provide_enum_display(self, f)
	}
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BillValueType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub value_id: Option<BillValueIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub unit: Option<commondatatypes::UnitOfMeasurementType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub value: Option<commondatatypes::ScaledNumberType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub value_percentage: Option<commondatatypes::ScaledNumberType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BillValueElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub value_id: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub unit: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub value: Option<commondatatypes::ScaledNumberElementsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub value_percentage: Option<commondatatypes::ScaledNumberElementsType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BillCostType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub cost_id: Option<BillCostIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub cost_type: Option<BillCostTypeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub value_id: Option<BillValueIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub unit: Option<commondatatypes::UnitOfMeasurementType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub currency: Option<commondatatypes::CurrencyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub cost: Option<commondatatypes::ScaledNumberType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub cost_percentage: Option<commondatatypes::ScaledNumberType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BillCostElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub cost_id: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub cost_type: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub value_id: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub unit: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub currency: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub cost: Option<commondatatypes::ScaledNumberElementsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub cost_percentage: Option<commondatatypes::ScaledNumberElementsType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BillPositionType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub position_id: Option<BillPositionIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub position_type: Option<BillPositionTypeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub time_period: Option<commondatatypes::TimePeriodType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub value: Option<BillValueType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub cost: Option<BillCostType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub label: Option<commondatatypes::LabelType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<commondatatypes::DescriptionType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BillPositionElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub position_id: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub position_type: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub time_period: Option<commondatatypes::TimePeriodElementsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub value: Option<BillValueElementsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub cost: Option<BillCostElementsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub label: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<commondatatypes::ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BillDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub bill_id: Option<BillIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub bill_type: Option<BillTypeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub scope_type: Option<commondatatypes::ScopeTypeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub total: Option<BillPositionType>, // XSD defines this as a variant without a few fields, but we do not care as everything is optional
	#[serde(skip_serializing_if = "Option::is_none")]
  pub position: Option<BillPositionType>, // XSD defines this as a variant without a few fields, but we do not care as everything is optional
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BillDataElementsType {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub bill_id: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub bill_type: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub scope_type: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub total: Option<BillPositionElementsType>, // XSD defines this as a variant without a few fields, but we do not care as everything is optional
	#[serde(skip_serializing_if = "Option::is_none")]
  pub position: Option<BillPositionElementsType>, // XSD defines this as a variant without a few fields, but we do not care as everything is optional
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BillListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub bill_data: Option<Vec<BillDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BillListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub bill_id: Option<BillIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub scope_type: Option<commondatatypes::ScopeTypeType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BillConstraintsDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub bill_id: Option<BillIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub position_count_min: Option<BillPositionCountType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub position_count_max: Option<BillPositionCountType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BillConstraintsDataElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub bill_id: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub position_count_min: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub position_count_max: Option<commondatatypes::ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BillConstraintsListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub bill_constraints_data: Option<Vec<BillConstraintsDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BillConstraintsListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub bill_id: Option<BillIdType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BillDescriptionDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub bill_id: Option<BillIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub bill_writeable: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub update_requred: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub supported_bill_type: Option<Vec<BillTypeType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BillDescriptionDataElementsType {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub bill_id: Option<commondatatypes::ElementTagType>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub bill_writeable: Option<commondatatypes::ElementTagType>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub update_requred: Option<commondatatypes::ElementTagType>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub supported_bill_type: Option<commondatatypes::ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BillDescriptionListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub bill_description_data: Option<Vec<BillDescriptionDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct BillDescriptionListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub bill_id: Option<BillIdType>,
}

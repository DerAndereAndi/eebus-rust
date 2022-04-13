use std::fmt;

use super::super::utils;

use serde::{Serialize, Deserialize};
use super::{commondatatypes, measurement, timetable};

pub type TariffIdType = u32;

pub type TariffCountType = TariffIdType;

pub type TierBoundaryIdType = u32;

pub type TierBoundaryCountType = TierBoundaryIdType;

pub type TierBoundaryTypeType = TierBoundaryTypeEnumType;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum TierBoundaryTypeEnumType {
	#[serde(rename = "powerBoundary")]
	PowerBoundary,
	#[serde(rename = "energyBoundary")]
	EnergyBoundary,
	#[serde(rename = "countBoundary")]
	CountBoundary,
}

impl fmt::Display for TierBoundaryTypeEnumType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		utils::provide_enum_display(self, f)
	}
}

pub type CommodityIdType = u32;

pub type TierIdType = u32;

pub type TierCountType = TierIdType;

pub type TierTypeType = TierTypeEnumType;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum TierTypeEnumType {
	#[serde(rename = "fixedCost")]
	FixedCost,
	#[serde(rename = "dynamicCost")]
	DynamicCost,
}

impl fmt::Display for TierTypeEnumType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		utils::provide_enum_display(self, f)
	}
}

pub type IncentiveIdType = u32;

pub type IncentiveCountType = IncentiveIdType;

pub type IncentiveTypeType = IncentiveTypeEnumType;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum IncentiveTypeEnumType {
	#[serde(rename = "absoluteCost")]
	AbsoluteCost,
	#[serde(rename = "relativeCost")]
	RelativeCost,
	#[serde(rename = "renewableEnergyPercentage")]
	RenewableEnergyPercentage,
	#[serde(rename = "co2Emission")]
	Co2Emission,
}

impl fmt::Display for IncentiveTypeEnumType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		utils::provide_enum_display(self, f)
	}
}

pub type IncentivePriorityType = u32;

pub type IncentiveValueTypeType = IncentiveValueTypeEnumType;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum IncentiveValueTypeEnumType {
	#[serde(rename = "value")]
	Value,
	#[serde(rename = "averageValue")]
	AverageValue,
	#[serde(rename = "minValue")]
	MinValue,
	#[serde(rename = "maxValue")]
	MaxValue,
}

impl fmt::Display for IncentiveValueTypeEnumType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		utils::provide_enum_display(self, f)
	}
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TariffOverallConstraintsDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub max_tariff_count: Option<TariffCountType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub max_boundary_count: Option<TierBoundaryCountType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub max_tier_count: Option<TierCountType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub max_incentive_count: Option<IncentiveCountType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub max_boundaries_per_tariff: Option<TierBoundaryCountType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub max_tiers_per_tariff: Option<TierCountType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub max_boundaries_per_tier: Option<TierBoundaryCountType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub max_incentives_per_tier: Option<IncentiveCountType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TariffOverallConstraintsDataElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_tariff_count: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_boundary_count: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_tier_count: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_incentive_count: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_boundaries_per_tariff: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_tiers_per_tariff: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_boundaries_per_tier: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max_incentives_per_tier: Option<commondatatypes::ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TariffDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub tariff_id: Option<TariffIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub active_tier_id: Option<Vec<TierIdType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TariffDataElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tariff_id: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub active_tier_id: Option<commondatatypes::ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TariffListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub tariff_data: Option<Vec<TariffDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TariffListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub tariff_id: Option<TariffIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub active_tier_id: Option<TierIdType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TariffTierRelationDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub tariff_id: Option<TariffIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub tier_id: Option<Vec<TierIdType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TariffTierRelationDataElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tariff_id: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tier_id: Option<commondatatypes::ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TariffTierRelationListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub tariff_tier_relation_data: Option<Vec<TariffTierRelationDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TariffTierRelationListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub tariff_id: Option<TariffIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub tier_id: Option<TierIdType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TariffBoundaryRelationDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub tariff_id: Option<TariffIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub boundary_id: Option<Vec<TierBoundaryIdType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TariffBoundaryRelationDataElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tariff_id: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub boundary_id: Option<commondatatypes::ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TariffBoundaryRelationListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub tariff_boundary_relation_data: Option<Vec<TariffBoundaryRelationDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TariffBoundaryRelationListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub tariff_id: Option<TariffIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub boundary_id: Option<TierBoundaryIdType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TariffDescriptionDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub tariff_id: Option<TariffIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub commodity_id: Option<CommodityIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub measurement_id: Option<measurement::MeasurementIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub tariff_writeable: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub update_required: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub scope_type: Option<commondatatypes::ScopeTypeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub label: Option<commondatatypes::LabelType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<commondatatypes::DescriptionType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub slot_id_support: Option<bool>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TariffDescriptionDataElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tariff_id: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub commodity_id: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub measurement_id: Option<measurement::MeasurementIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tariff_writeable: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub update_required: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub scope_type: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub label: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub slot_id_support: Option<commondatatypes::ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TariffDescriptionListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub tariff_description_data: Option<Vec<TariffDescriptionDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TariffDescriptionListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub tariff_id: Option<TariffIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub commodity_id: Option<CommodityIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub measurement_id: Option<measurement::MeasurementIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub scope_type: Option<commondatatypes::ScopeTypeType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TierBoundaryDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub boundary_id: Option<TierBoundaryIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub time_period: Option<commondatatypes::TimePeriodType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub time_table_id: Option<timetable::TimeTableIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub lower_boundary_value: Option<commondatatypes::ScaledNumberType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub upper_boundary_value: Option<commondatatypes::ScaledNumberType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TierBoundaryDataElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub boundary_id: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub time_period: Option<commondatatypes::TimePeriodElementsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub time_table_id: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub lower_boundary_value: Option<commondatatypes::ScaledNumberElementsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub upper_boundary_value: Option<commondatatypes::ScaledNumberElementsType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TierBoundaryListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub tier_boundary_data: Option<Vec<TierBoundaryDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TierBoundaryListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub boundary_id: Option<TierBoundaryIdType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TierBoundaryDescriptionDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub boundary_id: Option<TierBoundaryIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub boundary_type: Option<TierBoundaryTypeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub valid_for_tier_id: Option<TierIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub switch_to_tier_when_lower: Option<TierIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub switch_to_tier_when_higher: Option<TierIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub boundary_unit: Option<commondatatypes::UnitOfMeasurementType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub label: Option<commondatatypes::LabelType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<commondatatypes::DescriptionType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TierBoundaryDescriptionDataElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub boundary_id: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub boundary_type: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub valid_for_tier_id: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub switch_to_tier_when_lower: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub switch_to_tier_when_higher: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub boundary_unit: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub label: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<commondatatypes::ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TierBoundaryDescriptionListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub tier_boundary_description_data: Option<Vec<TierBoundaryDescriptionDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TierBoundaryDescriptionListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub boundary_id: Option<TierBoundaryIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub boundary_type: Option<TierBoundaryTypeType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CommodityDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub commodity_id: Option<CommodityIdType>,
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
pub struct CommodityDataElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub commodity_id: Option<commondatatypes::ElementTagType>,
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
pub struct CommodityListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub commodity_data: Option<Vec<CommodityDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CommodityListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub commodity_id: Option<CommodityIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub commodity_type: Option<commondatatypes::CommodityTypeType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TierDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub tier_id: Option<TierIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub time_period: Option<commondatatypes::TimePeriodType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub time_table_id: Option<timetable::TimeTableIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub active_incentive_id: Option<Vec<IncentiveIdType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TierDataElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tier_id: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub time_period: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub time_table_id: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub active_incentive_id: Option<commondatatypes::ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TierListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub tier_data: Option<Vec<TierDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TierListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub tier_id: Option<TierIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub active_incentive_id: Option<IncentiveIdType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TierIncentiveRelationDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub tier_id: Option<TierIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub incentive_id: Option<Vec<IncentiveIdType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TierIncentiveRelationDataElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tier_id: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub incentive_id: Option<commondatatypes::ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TierIncentiveRelationListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub tier_incentive_relation_data: Option<Vec<TierIncentiveRelationDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TierIncentiveRelationListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub tier_id: Option<TierIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub incentive_id: Option<IncentiveIdType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TierDescriptionDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub tier_id: Option<TierIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub tier_type: Option<TierTypeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub label: Option<commondatatypes::LabelType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<commondatatypes::DescriptionType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TierDescriptionDataElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tier_id: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub tier_type: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub label: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<commondatatypes::ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TierDescriptionListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub tier_description_data: Option<Vec<TierDescriptionDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TierDescriptionListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub tier_id: Option<TierIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub tier_type: Option<TierTypeType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IncentiveDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub incentive_id: Option<IncentiveIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub value_type: Option<IncentiveValueTypeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub timestamp: Option<commondatatypes::AbsoluteOrRelativeTimeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub time_period: Option<commondatatypes::TimePeriodType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub time_table_id: Option<timetable::TimeTableIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub value: Option<commondatatypes::ScaledNumberType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IncentiveDataElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub incentive_id: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub value_type: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub timestamp: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub time_period: Option<commondatatypes::TimePeriodElementsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub time_table_id: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub value: Option<commondatatypes::ScaledNumberElementsType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IncentiveListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub incentive_data: Option<Vec<IncentiveDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IncentiveListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub incentive_id: Option<IncentiveIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub value_type: Option<IncentiveValueTypeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub timestamp: Option<commondatatypes::TimestampIntervalType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IncentiveDescriptionDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub incentive_id: Option<IncentiveIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub incentive_type: Option<IncentiveTypeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub incentive_priority: Option<IncentivePriorityType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub currency: Option<commondatatypes::CurrencyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub unit: Option<commondatatypes::UnitOfMeasurementType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub label: Option<commondatatypes::LabelType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<commondatatypes::DescriptionType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IncentiveDescriptionDataElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub incentive_id: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub incentive_type: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub incentive_priority: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub currency: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub unit: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub label: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<commondatatypes::ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IncentiveDescriptionListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub incentive_description_data: Option<Vec<IncentiveDescriptionDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IncentiveDescriptionListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub incentive_id: Option<IncentiveIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub incentive_type: Option<IncentiveTypeType>,
}

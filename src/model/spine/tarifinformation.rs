use serde::{Serialize, Deserialize};
use super::commondatatypes;
use super::measurement;
use super::timetable;

pub type TariffIdType = u8;

pub type TariffCountType = TariffIdType;

pub type TierBoundaryIdType = u8;

pub type TierBoundaryCountType = TierBoundaryIdType;

pub type TierBoundaryTypeType = TierBoundaryTypeEnumType;

#[derive(Serialize, Deserialize, Debug)]
pub enum TierBoundaryTypeEnumType {
	#[serde(rename = "powerBoundary")]
	PowerBoundary,
	#[serde(rename = "energyBoundary")]
	EnergyBoundary,
	#[serde(rename = "countBoundary")]
	CountBoundary,
}

pub type CommodityIdType = u8;

pub type TierIdType = u8;

pub type TierCountType = TierIdType;

pub type TierTypeType = TierTypeEnumType;

#[derive(Serialize, Deserialize, Debug)]
pub enum TierTypeEnumType {
	#[serde(rename = "fixedCost")]
	FixedCost,
	#[serde(rename = "dynamicCost")]
	DynamicCost,
}

pub type IncentiveIdType = u8;

pub type IncentiveCountType = IncentiveIdType;

pub type IncentiveTypeType = IncentiveTypeEnumType;

#[derive(Serialize, Deserialize, Debug)]
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

pub type IncentivePriorityType = u8;

pub type IncentiveValueTypeType = IncentiveValueTypeEnumType;

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TariffOverallConstraintsDataType {
  pub max_tariff_count: Option<TariffCountType>,
  pub max_boundary_count: Option<TierBoundaryCountType>,
  pub max_tier_count: Option<TierCountType>,
  pub max_incentive_count: Option<IncentiveCountType>,
  pub max_boundaries_per_tariff: Option<TierBoundaryCountType>,
  pub max_tiers_per_tariff: Option<TierCountType>,
  pub max_boundaries_per_tier: Option<TierBoundaryCountType>,
  pub max_incentives_per_tier: Option<IncentiveCountType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TariffDataType {
  pub tariff_id: Option<TariffIdType>,
  pub active_tier_id: Option<Vec<TierIdType>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TariffListDataType {
  pub tariff_data: Option<Vec<TariffDataType>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TariffListDataSelectorsType {
  pub tariff_id: Option<TariffIdType>,
  pub active_tier_id: Option<TierIdType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TariffTierRelationDataType {
  pub tariff_id: Option<TariffIdType>,
  pub tier_id: Option<Vec<TierIdType>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TariffTierRelationListDataType {
  pub tariff_tier_relation_data: Option<Vec<TariffTierRelationDataType>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TariffTierRelationListDataSelectorsType {
  pub tariff_id: Option<TariffIdType>,
  pub tier_id: Option<TierIdType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TariffBoundaryRelationDataType {
  pub tariff_id: Option<TariffIdType>,
  pub boundary_id: Option<Vec<TierBoundaryIdType>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TariffBoundaryRelationListDataType {
  pub tariff_boundary_relation_data: Option<Vec<TariffBoundaryRelationDataType>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TariffBoundaryRelationListDataSelectorsType {
  pub tariff_id: Option<TariffIdType>,
  pub boundary_id: Option<TierBoundaryIdType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TariffDescriptionDataType {
  pub tariff_id: Option<TariffIdType>,
  pub commodity_id: Option<CommodityIdType>,
  pub measurement_id: Option<measurement::MeasurementIdType>,
  pub tariff_writeable: Option<bool>,
  pub update_required: Option<bool>,
  pub scope_type: Option<commondatatypes::ScopeTypeType>,
  pub label: Option<commondatatypes::LabelType>,
  pub description: Option<commondatatypes::DescriptionType>,
  pub slot_id_support: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TariffDescriptionListDataType {
  pub tariff_description_data: Option<Vec<TariffDescriptionDataType>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TariffDescriptionListDataSelectorsType {
  pub tariff_id: Option<TariffIdType>,
  pub commodity_id: Option<CommodityIdType>,
  pub measurement_id: Option<measurement::MeasurementIdType>,
  pub scope_type: Option<commondatatypes::ScopeTypeType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TierBoundaryDataType {
  pub boundary_id: Option<TierBoundaryIdType>,
  pub time_period: Option<commondatatypes::TimePeriodType>,
  pub time_table_id: Option<timetable::TimeTableIdType>,
  pub lower_boundary_value: Option<commondatatypes::ScaledNumberType>,
  pub upper_boundary_value: Option<commondatatypes::ScaledNumberType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TierBoundaryListDataType {
  pub tier_boundary_data: Option<Vec<TierBoundaryDataType>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TierBoundaryListDataSelectorsType {
  pub boundary_id: Option<TierBoundaryIdType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TierBoundaryDescriptionDataType {
  pub boundary_id: Option<TierBoundaryIdType>,
  pub boundary_type: Option<TierBoundaryTypeType>,
  pub valid_for_tier_id: Option<TierIdType>,
  pub switch_to_tier_when_lower: Option<TierIdType>,
  pub switch_to_tier_when_higher: Option<TierIdType>,
  pub boundary_unit: Option<commondatatypes::UnitOfMeasurementType>,
  pub label: Option<commondatatypes::LabelType>,
  pub description: Option<commondatatypes::DescriptionType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TierBoundaryDescriptionListDataType {
  pub tier_boundary_description_data: Option<Vec<TierBoundaryDescriptionDataType>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TierBoundaryDescriptionListDataSelectorsType {
  pub boundary_id: Option<TierBoundaryIdType>,
  pub boundary_type: Option<TierBoundaryTypeType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CommodityDataType {
  pub commodity_id: Option<CommodityIdType>,
  pub commodity_type: Option<commondatatypes::CommodityTypeType>,
  pub positive_energy_direction: Option<commondatatypes::EnergyDirectionType>,
  pub label: Option<commondatatypes::LabelType>,
  pub description: Option<commondatatypes::DescriptionType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CommodityListDataType {
  pub commodity_data: Option<Vec<CommodityDataType>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CommodityListDataSelectorsType {
  pub commodity_id: Option<CommodityIdType>,
  pub commodity_type: Option<commondatatypes::CommodityTypeType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TierDataType {
  pub tier_id: Option<TierIdType>,
  pub time_period: Option<commondatatypes::TimePeriodType>,
  pub time_table_id: Option<timetable::TimeTableIdType>,
  pub active_incentive_id: Option<Vec<IncentiveIdType>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TierListDataType {
  pub tier_data: Option<Vec<TierDataType>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TierListDataSelectorsType {
  pub tier_id: Option<TierIdType>,
  pub active_incentive_id: Option<IncentiveIdType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TierIncentiveRelationDataType {
  pub tier_id: Option<TierIdType>,
  pub incentive_id: Option<Vec<IncentiveIdType>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TierIncentiveRelationListDataType {
  pub tier_incentive_relation_data: Option<Vec<TierIncentiveRelationDataType>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TierIncentiveRelationListDataSelectorsType {
  pub tier_id: Option<TierIdType>,
  pub incentive_id: Option<IncentiveIdType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TierDescriptionDataType {
  pub tier_id: Option<TierIdType>,
  pub tier_type: Option<TierTypeType>,
  pub label: Option<commondatatypes::LabelType>,
  pub description: Option<commondatatypes::DescriptionType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TierDescriptionListDataType {
  pub tier_description_data: Option<Vec<TierDescriptionDataType>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TierDescriptionListDataSelectorsType {
  pub tier_id: Option<TierIdType>,
  pub tier_type: Option<TierTypeType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IncentiveDataType {
  pub incentive_id: Option<IncentiveIdType>,
  pub value_type: Option<IncentiveValueTypeType>,
  pub timestamp: Option<commondatatypes::AbsoluteOrRelativeTimeType>,
  pub time_period: Option<commondatatypes::TimePeriodType>,
  pub time_table_id: Option<timetable::TimeTableIdType>,
  pub value: Option<commondatatypes::ScaledNumberType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IncentiveListDataType {
  pub incentive_data: Option<Vec<IncentiveDataType>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IncentiveListDataSelectorsType {
  pub incentive_id: Option<IncentiveIdType>,
  pub value_type: Option<IncentiveValueTypeType>,
  pub timestamp: Option<commondatatypes::TimestampIntervalType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IncentiveDescriptionDataType {
  pub incentive_id: Option<IncentiveIdType>,
  pub incentive_type: Option<IncentiveTypeType>,
  pub incentive_priority: Option<IncentivePriorityType>,
  pub currency: Option<commondatatypes::CurrencyType>,
  pub unit: Option<commondatatypes::UnitOfMeasurementType>,
  pub label: Option<commondatatypes::LabelType>,
  pub description: Option<commondatatypes::DescriptionType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IncentiveDescriptionListDataType {
  pub incentive_description_data: Option<Vec<IncentiveDescriptionDataType>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IncentiveDescriptionListDataSelectorsType {
  pub incentive_id: Option<IncentiveIdType>,
  pub incentive_type: Option<IncentiveTypeType>,
}

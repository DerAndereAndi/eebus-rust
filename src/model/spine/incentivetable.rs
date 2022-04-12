use serde::{Serialize, Deserialize};
use super::timetable;
use super::tariffinformation;

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IncentiveTableType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub tariff: Option<tariffinformation::TariffDataType>, // ignoring changes
	#[serde(skip_serializing_if = "Option::is_none")]
  pub incentive_slot: Option<Vec<IncentiveTableIncentiveSlotType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IncentiveTableIncentiveSlotType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub time_interval: Option<timetable::TimeTableDataType>, // ignoring changes
	#[serde(skip_serializing_if = "Option::is_none")]
  pub tier: Option<Vec<IncentiveTableTierType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IncentiveTableTierType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub tier: Option<tariffinformation::TierDataType>, // ignoring changes
	#[serde(skip_serializing_if = "Option::is_none")]
  pub boundary: Option<Vec<tariffinformation::TierBoundaryDataType>>, // ignoring changes
	#[serde(skip_serializing_if = "Option::is_none")]
  pub incentive: Option<Vec<tariffinformation::IncentiveDataType>>, // ignoring changes
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IncentiveTableDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub incentive_table: Option<Vec<IncentiveTableType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IncentiveTableElementsType {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub tariff: Option<tariffinformation::TariffDataElementsType>, // ignoring changes
  #[serde(skip_serializing_if = "Option::is_none")]
  pub incentive_slot: Option<IncentiveTableIncentiveSlotType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IncentiveTableIncentiveSlotElementsType {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub time_interval: Option<timetable::TimeTableDataElementsType>, // ignoring changes
  #[serde(skip_serializing_if = "Option::is_none")]
  pub tier: Option<IncentiveTableTierType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IncentiveTableTierElementsType {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub tier: Option<tariffinformation::TierDataElementsType>, // ignoring changes
  #[serde(skip_serializing_if = "Option::is_none")]
  pub boundary: Option<tariffinformation::TierBoundaryDataElementsType>, // ignoring changes
  #[serde(skip_serializing_if = "Option::is_none")]
  pub incentive: Option<tariffinformation::IncentiveDataElementsType>, // ignoring changes
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IncentiveTableDataElementsType {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub incentive_table: Option<IncentiveTableElementsType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IncentiveTableDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub tariff: Option<tariffinformation::TariffListDataSelectorsType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IncentiveTableDescriptionType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub tariff_description: Option<tariffinformation::TariffDescriptionDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub tier: Option<Vec<IncentiveTableDescriptionTierType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IncentiveTableDescriptionTierType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub tier_description: Option<tariffinformation::TierDescriptionDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub boundary_description: Option<Vec<tariffinformation::TierBoundaryDescriptionDataType>>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub incentive_description: Option<Vec<tariffinformation::IncentiveDescriptionDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IncentiveTableDescriptionDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub incentive_table_description: Option<Vec<IncentiveTableDescriptionType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IncentiveTableDescriptionElementsType {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub tariff_description: Option<tariffinformation::TariffDescriptionDataElementsType>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub tier: Option<IncentiveTableDescriptionTierType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IncentiveTableDescriptionTierElementsType {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub tier_description: Option<tariffinformation::TierDescriptionDataElementsType>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub boundary_description: Option<Vec<tariffinformation::TierBoundaryDescriptionDataElementsType>>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub incentive_description: Option<Vec<tariffinformation::IncentiveDescriptionDataElementsType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IncentiveTableDescriptionDataElementsType {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub incentive_table_description: Option<IncentiveTableDescriptionElementsType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IncentiveTableDescriptionDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub tariff_description: Option<tariffinformation::TariffDescriptionListDataSelectorsType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IncentiveTableConstraintsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub tariff: Option<tariffinformation::TariffDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub tariff_constraints: Option<tariffinformation::TariffOverallConstraintsDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub incentive_slot_constraints: Option<timetable::TimeTableConstraintsDataType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IncentiveTableConstraintsDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub incentive_table_constraints: Option<Vec<IncentiveTableConstraintsType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IncentiveTableConstraintsElementsType {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub tariff: Option<tariffinformation::TariffDataElementsType>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub tariff_constraints: Option<tariffinformation::TariffOverallConstraintsDataElementsType>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub incentive_slot_constraints: Option<timetable::TimeTableConstraintsDataElementsType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IncentiveTableConstraintsDataElementsType {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub incentive_table_constraints: Option<IncentiveTableConstraintsElementsType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IncentiveTableConstraintsDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub tariff: Option<tariffinformation::TariffListDataSelectorsType>,
}

use serde::{Serialize, Deserialize};
use super::commondatatypes;
use super::timetable;
use super::tarifinformation;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IncentiveTableType {
  pub tariff: Option<TariffType>,
  pub incentive_slot: Option<Vec<IncentiveTableIncentiveSlotType>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IncentiveTableIncentiveSlotType {
  pub time_interval: Option<timetable::TimeTableDataType>,
  pub tier: Option<Vec<IncentiveTableTierType>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IncentiveTableTierType {
  pub tier: Option<tarifinformation::TierDataType>,
  pub boundary: Option<Vec<tarifinformation::TierBoundaryDataType>>,
  pub incentive: Option<Vec<IncentiveDataType>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IncentiveTableDataType {
  pub incentive_table: Option<Vec<IncentiveTableType>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IncentiveTableDataSelectorsType {
  pub tariff: Option<tarifinformation::TariffListDataSelectorsType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IncentiveTableDescriptionType {
  pub tariff_description: Option<tarifinformation::TariffDescriptionDataType>,
  pub tier: Option<Vec<IncentiveTableDescriptionTierType>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IncentiveTableDescriptionTierType {
  pub tier_description: Option<tarifinformation::TierDescriptionDataType>,
  pub boundary_description: Option<Vec<tarifinformation::TierBoundaryDescriptionDataType>>,
  pub incentive_description: Option<Vec<IncentiveDescriptionDataType>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IncentiveTableDescriptionDataType {
  pub incentive_table_description: Option<Vec<IncentiveTableDescriptionType>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IncentiveTableDescriptionDataSelectorsType {
  pub tariff_description: Option<tarifinformation::TariffDescriptionListDataSelectorsType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IncentiveTableConstraintsType {
  pub tariff: Option<tarifinformation::TariffDataType>,
  pub tariff_constraints: Option<tarifinformation::TariffOverallConstraintsDataType>,
  pub incentive_slot_constraints: Option<timetable::TimeTableConstraintsDataType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IncentiveTableConstraintsDataType {
  pub incentive_table_constraints: Option<Vec<IncentiveTableConstraintsType>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IncentiveTableConstraintsDataSelectorsType {
  pub tariff: Option<tarifinformation::TariffListDataSelectorsType>,
}

use serde::{Serialize, Deserialize};
use super::timetable;
use super::tarifinformation;

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IncentiveTableType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub tariff: Option<tarifinformation::TariffDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub incentive_slot: Option<Vec<IncentiveTableIncentiveSlotType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IncentiveTableIncentiveSlotType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub time_interval: Option<timetable::TimeTableDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub tier: Option<Vec<IncentiveTableTierType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IncentiveTableTierType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub tier: Option<tarifinformation::TierDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub boundary: Option<Vec<tarifinformation::TierBoundaryDataType>>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub incentive: Option<Vec<tarifinformation::IncentiveDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IncentiveTableDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub incentive_table: Option<Vec<IncentiveTableType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IncentiveTableDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub tariff: Option<tarifinformation::TariffListDataSelectorsType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IncentiveTableDescriptionType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub tariff_description: Option<tarifinformation::TariffDescriptionDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub tier: Option<Vec<IncentiveTableDescriptionTierType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IncentiveTableDescriptionTierType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub tier_description: Option<tarifinformation::TierDescriptionDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub boundary_description: Option<Vec<tarifinformation::TierBoundaryDescriptionDataType>>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub incentive_description: Option<Vec<tarifinformation::IncentiveDescriptionDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IncentiveTableDescriptionDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub incentive_table_description: Option<Vec<IncentiveTableDescriptionType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IncentiveTableDescriptionDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub tariff_description: Option<tarifinformation::TariffDescriptionListDataSelectorsType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct IncentiveTableConstraintsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub tariff: Option<tarifinformation::TariffDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub tariff_constraints: Option<tarifinformation::TariffOverallConstraintsDataType>,
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
pub struct IncentiveTableConstraintsDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub tariff: Option<tarifinformation::TariffListDataSelectorsType>,
}

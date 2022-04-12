use std::fmt;

use super::super::utils;

use serde::{Serialize, Deserialize};
use super::commondatatypes;

pub type UseCaseActorType = UseCaseActorEnumType;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum UseCaseActorEnumType {
	#[serde(rename = "EV")]
	Ev,
}

impl fmt::Display for UseCaseActorEnumType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		utils::provide_enum_display(self, f)
	}
}

pub type UseCaseNameType = UseCaseNameEnumType;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum UseCaseNameEnumType {
	#[serde(rename = "measurementOfElectricityDuringEvCharging")]
	MeasurementOfElectricityDuringEvCharging,
	#[serde(rename = "optimizationOfSelfConsumptionDuringEvCharging")]
	OptimizationOfSelfConsumptionDuringEvCharging,
	#[serde(rename = "overloadProtectionByEvChargingCurrentCurtailment")]
	OverloadProtectionByEvChargingCurrentCurtailment,
	#[serde(rename = "coordinatedEvCharging")]
	CoordinatedEvCharging,
	#[serde(rename = "evCommissioningAndConfiguration")]
	EvCommissioningAndConfiguration,
	#[serde(rename = "evseCommissioningAndConfiguration")]
	EvseCommissioningAndConfiguration,
	#[serde(rename = "evChargingSummary")]
	EvChargingSummary,
	#[serde(rename = "evStateOfCharge")]
	EvStateOfCharge,
}

impl fmt::Display for UseCaseNameEnumType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		utils::provide_enum_display(self, f)
	}
}

pub type UseCaseScenarioSupportType = u32;

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UseCaseSupportType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub use_case_name: Option<UseCaseNameType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub use_case_version: Option<commondatatypes::SpecificationVersionType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub use_case_available: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub scenario_support: Option<Vec<UseCaseScenarioSupportType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UseCaseSupportElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub use_case_name: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub use_case_version: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub use_case_available: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub scenario_support: Option<commondatatypes::ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UseCaseSupportSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub use_case_name: Option<UseCaseNameType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub use_case_version: Option<commondatatypes::SpecificationVersionType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub scenario_support: Option<UseCaseScenarioSupportType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UseCaseInformationDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub address: Option<commondatatypes::FeatureAddressType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub actor: Option<UseCaseActorType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub use_case_support: Option<Vec<UseCaseSupportType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UseCaseInformationDataElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub address: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub actor: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub use_case_support: Option<commondatatypes::ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UseCaseInformationListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub use_case_information_data: Option<Vec<UseCaseInformationDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UseCaseInformationListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub address: Option<commondatatypes::FeatureAddressType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub actor: Option<UseCaseActorType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub use_case_support: Option<UseCaseSupportType>,
}

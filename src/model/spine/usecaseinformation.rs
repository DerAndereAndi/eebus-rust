use serde::{Serialize, Deserialize};
use super::commondatatypes;

pub type UseCaseActorType = UseCaseActorEnumType;

#[derive(Serialize, Deserialize, Debug)]
pub enum UseCaseActorEnumType {
	#[serde(rename = "EV")]
	Ev,
}

pub type UseCaseNameType = UseCaseNameEnumType;

#[derive(Serialize, Deserialize, Debug)]
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

pub type UseCaseScenarioSupportType = u8;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UseCaseSupportType {
  pub use_case_name: Option<UseCaseNameType>,
  pub use_case_version: Option<commondatatypes::SpecificationVersionType>,
  pub use_case_available: Option<bool>,
  pub scenario_support: Option<Vec<UseCaseScenarioSupportType>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UseCaseSupportSelectorsType {
  pub use_case_name: Option<UseCaseNameType>,
  pub use_case_version: Option<commondatatypes::SpecificationVersionType>,
  pub scenario_support: Option<UseCaseScenarioSupportType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UseCaseInformationDataType {
  pub address: Option<commondatatypes::FeatureAddressType>,
  pub actor: Option<UseCaseActorType>,
  pub use_case_support: Option<Vec<UseCaseSupportType>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UseCaseInformationListDataType {
  pub use_case_information_data: Option<Vec<UseCaseInformationDataType>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct UseCaseInformationListDataSelectorsType {
  pub address: Option<commondatatypes::FeatureAddressType>,
  pub actor: Option<UseCaseActorType>,
  pub use_case_support: Option<UseCaseSupportType>,
}

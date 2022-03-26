use serde::{Serialize, Deserialize};
use super::commondatatypes;
use super::measurement;

pub type ElectricalConnectionIdType = u8;

pub type ElectricalConnectionParameterIdType = u8;

pub type ElectricalConnectionMeasurandVariantType = ElectricalConnectionMeasurandVariantEnumType;

#[derive(Serialize, Deserialize, Debug)]
pub enum ElectricalConnectionMeasurandVariantEnumType {
	#[serde(rename = "amplitude")]
	Amplitude,
	#[serde(rename = "rms")]
	Rms,
	#[serde(rename = "instantaneous")]
	Instantaneous,
	#[serde(rename = "angle")]
	Angle,
	#[serde(rename = "cosPhi")]
	CosPhi,
}

pub type ElectricalConnectionVoltageTypeType = ElectricalConnectionVoltageTypeEnumType;

#[derive(Serialize, Deserialize, Debug)]
pub enum ElectricalConnectionVoltageTypeEnumType {
	#[serde(rename = "ac")]
	Ac,
	#[serde(rename = "dc")]
	Dc,
}

pub type ElectricalConnectionAcMeasurementTypeType = ElectricalConnectionAcMeasurementTypeEnumType;

#[derive(Serialize, Deserialize, Debug)]
pub enum ElectricalConnectionAcMeasurementTypeEnumType {
	#[serde(rename = "real")]
	Real,
	#[serde(rename = "reactive")]
	Reactive,
	#[serde(rename = "apparent")]
	Apparent,
	#[serde(rename = "phase")]
	Phase,
}

pub type ElectricalConnectionPhaseNameType = ElectricalConnectionPhaseNameEnumType;

#[derive(Serialize, Deserialize, Debug)]
pub enum ElectricalConnectionPhaseNameEnumType {
	#[serde(rename = "a")]
	A,
	#[serde(rename = "b")]
	B,
	#[serde(rename = "c")]
	C,
	#[serde(rename = "ab")]
	Ab,
	#[serde(rename = "bc")]
	Bc,
	#[serde(rename = "ac")]
	Ac,
	#[serde(rename = "abc")]
	Abc,
	#[serde(rename = "neutral")]
	Neutral,
	#[serde(rename = "ground")]
	Ground,
	#[serde(rename = "none")]
	None,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum ElectricalConnectionConnectionPointType {
	#[serde(rename = "grid")]
	Grid,
	#[serde(rename = "home")]
	Home,
	#[serde(rename = "pv")]
	Pv,
	#[serde(rename = "sd")]
	Sd,
	#[serde(rename = "other")]
	Other,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ElectricalConnectionParameterDescriptionDataType {
	pub electrical_connection_id: Option<ElectricalConnectionIdType>,
	pub parameter_id: Option<ElectricalConnectionParameterIdType>,
	pub measurement_id: Option<measurement::MeasurementIdType>,
	pub voltage_type: Option<ElectricalConnectionVoltageTypeType>,
	pub ac_measured_phases: Option<ElectricalConnectionPhaseNameType>,
	pub ac_measured_in_reference_to: Option<ElectricalConnectionPhaseNameType>,
	pub ac_measurement_type: Option<ElectricalConnectionAcMeasurementTypeType>,
	pub ac_measurement_variant: Option<ElectricalConnectionMeasurandVariantType>,
	pub scope_type: Option<commondatatypes::ScopeTypeType>,
	pub label: Option<commondatatypes::LabelType>,
	pub description: Option<commondatatypes::DescriptionType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ElectricalConnectionParameterDescriptionListDataType {
	pub electrical_connection_parameter_description_data: Option<Vec<ElectricalConnectionParameterDescriptionDataType>>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ElectricalConnectionParameterDescriptionListDataSelectorsType {
	pub electrical_connection_id: Option<ElectricalConnectionIdType>,
	pub parameter_id: Option<ElectricalConnectionParameterIdType>,
	pub measurement_id: Option<measurement::MeasurementIdType>,
	pub scope_type: Option<commondatatypes::ScopeTypeType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ElectricalConnectionPermittedValueSetDataType {
	pub electrical_connection_id: Option<ElectricalConnectionIdType>,
	pub parameter_id: Option<ElectricalConnectionParameterIdType>,
	pub permitted_value_set: Option<Vec<commondatatypes::ScaledNumberSetType>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ElectricalConnectionPermittedValueSetListDataType {
	pub electrical_connection_permitted_value_set_data: Option<Vec<ElectricalConnectionPermittedValueSetDataType>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ElectricalConnectionPermittedValueSetListDataSelectorsType {
	pub electrical_connection_id: Option<ElectricalConnectionIdType>,
	pub parameter_id: Option<ElectricalConnectionParameterIdType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ElectricalConnectionStateDataType {
	pub electrical_connection_id: Option<ElectricalConnectionIdType>,
	pub timestamp: Option<commondatatypes::AbsoluteOrRelativeTimeType>,
	pub current_energy_mode: Option<commondatatypes::EnergyModeType>,
	pub consumption_time: Option<String>,
	pub production_time: Option<String>,
	pub total_consumption_time: Option<String>,
	pub total_production_time: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ElectricalConnectionStateListDataType {
	pub electrical_connection_state_data: Option<Vec<ElectricalConnectionStateDataType>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ElectricalConnectionStateListDataSelectorsType {
	pub electrical_connection_id: Option<ElectricalConnectionIdType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ElectricalConnectionDescriptionDataType {
	pub electrical_connection_id: Option<ElectricalConnectionIdType>,
	pub power_supply_type: Option<ElectricalConnectionVoltageTypeType>,
	pub ac_connected_phases: Option<u8>,
	pub positive_energy_direction: Option<commondatatypes::EnergyDirectionType>,
	pub scope_type: Option<commondatatypes::ScopeTypeType>,
	pub label: Option<commondatatypes::LabelType>,
	pub description: Option<commondatatypes::DescriptionType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ElectricalConnectionDescriptionListDataType {
	pub electrical_connection_description_data: Option<Vec<ElectricalConnectionDescriptionDataType>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ElectricalConnectionDescriptionListDataSelectorsType {
	pub electrical_connection_id: Option<ElectricalConnectionIdType>,
	pub scope_type: Option<commondatatypes::ScopeTypeType>,
}

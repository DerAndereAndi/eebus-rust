use serde::{Serialize, Deserialize};
use super::commondatatypes;
use super::measurement;

pub type ElectricalConnectionIdType = u8;

pub type ElectricalConnectionParameterIdType = u8;

pub type ElectricalConnectionMeasurandVariantType = ElectricalConnectionMeasurandVariantEnumType;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
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

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum ElectricalConnectionVoltageTypeEnumType {
	#[serde(rename = "ac")]
	Ac,
	#[serde(rename = "dc")]
	Dc,
}

pub type ElectricalConnectionAcMeasurementTypeType = ElectricalConnectionAcMeasurementTypeEnumType;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
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

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
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

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
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

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ElectricalConnectionParameterDescriptionDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub electrical_connection_id: Option<ElectricalConnectionIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub parameter_id: Option<ElectricalConnectionParameterIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub measurement_id: Option<measurement::MeasurementIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub voltage_type: Option<ElectricalConnectionVoltageTypeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ac_measured_phases: Option<ElectricalConnectionPhaseNameType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ac_measured_in_reference_to: Option<ElectricalConnectionPhaseNameType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ac_measurement_type: Option<ElectricalConnectionAcMeasurementTypeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ac_measurement_variant: Option<ElectricalConnectionMeasurandVariantType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub scope_type: Option<commondatatypes::ScopeTypeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub label: Option<commondatatypes::LabelType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<commondatatypes::DescriptionType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ElectricalConnectionParameterDescriptionListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub electrical_connection_parameter_description_data: Option<Vec<ElectricalConnectionParameterDescriptionDataType>>
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ElectricalConnectionParameterDescriptionListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub electrical_connection_id: Option<ElectricalConnectionIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub parameter_id: Option<ElectricalConnectionParameterIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub measurement_id: Option<measurement::MeasurementIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub scope_type: Option<commondatatypes::ScopeTypeType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ElectricalConnectionPermittedValueSetDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub electrical_connection_id: Option<ElectricalConnectionIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub parameter_id: Option<ElectricalConnectionParameterIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub permitted_value_set: Option<Vec<commondatatypes::ScaledNumberSetType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ElectricalConnectionPermittedValueSetListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub electrical_connection_permitted_value_set_data: Option<Vec<ElectricalConnectionPermittedValueSetDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ElectricalConnectionPermittedValueSetListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub electrical_connection_id: Option<ElectricalConnectionIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub parameter_id: Option<ElectricalConnectionParameterIdType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ElectricalConnectionStateDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub electrical_connection_id: Option<ElectricalConnectionIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub timestamp: Option<commondatatypes::AbsoluteOrRelativeTimeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub current_energy_mode: Option<commondatatypes::EnergyModeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub consumption_time: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub production_time: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub total_consumption_time: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub total_production_time: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ElectricalConnectionStateListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub electrical_connection_state_data: Option<Vec<ElectricalConnectionStateDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ElectricalConnectionStateListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub electrical_connection_id: Option<ElectricalConnectionIdType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ElectricalConnectionDescriptionDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub electrical_connection_id: Option<ElectricalConnectionIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub power_supply_type: Option<ElectricalConnectionVoltageTypeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub ac_connected_phases: Option<u8>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub positive_energy_direction: Option<commondatatypes::EnergyDirectionType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub scope_type: Option<commondatatypes::ScopeTypeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub label: Option<commondatatypes::LabelType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub description: Option<commondatatypes::DescriptionType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ElectricalConnectionDescriptionListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub electrical_connection_description_data: Option<Vec<ElectricalConnectionDescriptionDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ElectricalConnectionDescriptionListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub electrical_connection_id: Option<ElectricalConnectionIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub scope_type: Option<commondatatypes::ScopeTypeType>,
}

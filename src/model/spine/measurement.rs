use serde::{Serialize, Deserialize};
use super::commondatatypes;
use super::threshold;

pub type MeasurementIdType = u8;

pub type MeasurementTypeType = MeasurementTypeEnumType;

#[derive(Serialize, Deserialize, Debug)]
pub enum MeasurementTypeEnumType {
	#[serde(rename = "acceleration")]
	Acceleration,
	#[serde(rename = "angle")]
	Angle,
	#[serde(rename = "angularVelocity")]
	AngularVelocity,
	#[serde(rename = "area")]
	Area,
	#[serde(rename = "atmosphericPressure")]
	AtmosphericPressure,
	#[serde(rename = "capacity")]
	Capacity,
	#[serde(rename = "concentration")]
	Concentration,
	#[serde(rename = "count")]
	Count,
	#[serde(rename = "current")]
	Current,
	#[serde(rename = "density")]
	Density,
	#[serde(rename = "distance")]
	Distance,
	#[serde(rename = "electricField")]
	ElectricField,
	#[serde(rename = "energy")]
	Energy,
	#[serde(rename = "force")]
	Force,
	#[serde(rename = "frequency")]
	Frequency,
	#[serde(rename = "harmonicDistortion")]
	HarmonicDistortion,
	#[serde(rename = "heat")]
	Heat,
	#[serde(rename = "heatFlux")]
	HeatFlux,
	#[serde(rename = "illuminance")]
	Illuminance,
	#[serde(rename = "impulse")]
	Impulse,
	#[serde(rename = "level")]
	Level,
	#[serde(rename = "magneticField")]
	MagneticField,
	#[serde(rename = "mass")]
	Mass,
	#[serde(rename = "massFlow")]
	MassFlow,
	#[serde(rename = "particles")]
	Particles,
	#[serde(rename = "percentage")]
	Percentage,
	#[serde(rename = "power")]
	Power,
	#[serde(rename = "powerFactor")]
	PowerFactor,
	#[serde(rename = "pressure")]
	Pressure,
	#[serde(rename = "radonActivity")]
	RadonActivity,
	#[serde(rename = "relativeHumidity")]
	RelativeHumidity,
	#[serde(rename = "resistance")]
	Resistance,
	#[serde(rename = "solarRadiation")]
	SolarRadiation,
	#[serde(rename = "speed")]
	Speed,
	#[serde(rename = "temperature")]
	Temperature,
	#[serde(rename = "time")]
	Time,
	#[serde(rename = "torque")]
	Torque,
	#[serde(rename = "unknown")]
	Unknown,
	#[serde(rename = "velocity")]
	Velocity,
	#[serde(rename = "voltage")]
	Voltage,
	#[serde(rename = "volume")]
	Volume,
	#[serde(rename = "volumetricFlow")]
	VolumetricFlow,
}

pub type MeasurementValueTypeType = MeasurementValueTypeEnumType;

#[derive(Serialize, Deserialize, Debug)]
pub enum MeasurementValueTypeEnumType {
	#[serde(rename = "value")]
	Value,
	#[serde(rename = "averageValue")]
	AverageValue,
	#[serde(rename = "minValue")]
	MinValue,
	#[serde(rename = "maxValue")]
	MaxValue,
	#[serde(rename = "standardDeviation")]
	StandardDeviation,
}

pub type MeasurementValueSourceType = MeasurementValueSourceEnumType;

#[derive(Serialize, Deserialize, Debug)]
pub enum MeasurementValueSourceEnumType {
	#[serde(rename = "measuredValue")]
	MeasuredValue,
	#[serde(rename = "calculatedValue")]
	CalculatedValue,
	#[serde(rename = "empiricalValue")]
	EmpiricalValue,
}

pub type MeasurementValueTendencyType = MeasurementValueTendencyEnumType;

#[derive(Serialize, Deserialize, Debug)]
pub enum MeasurementValueTendencyEnumType {
	#[serde(rename = "rising")]
	Rising,
	#[serde(rename = "stable")]
	Stable,
	#[serde(rename = "falling")]
	Falling,
}

pub type MeasurementValueStateType = MeasurementValueStateEnumType;

#[derive(Serialize, Deserialize, Debug)]
pub enum MeasurementValueStateEnumType {
	#[serde(rename = "normal")]
	Normal,
	#[serde(rename = "outOfRange")]
	OutOfRange,
	#[serde(rename = "error")]
	Error,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MeasurementDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub measurement_id: Option<MeasurementIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub value_type: Option<MeasurementValueTypeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub timestamp: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub value: Option<commondatatypes::ScaledNumberType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub evaluation_period: Option<commondatatypes::TimePeriodType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub value_source: Option<MeasurementValueSourceType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub value_tendency: Option<MeasurementValueTendencyType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub value_state: Option<MeasurementValueStateType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MeasurementListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub measurement_data: Option<Vec<MeasurementDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MeasurementListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub measurement_id: Option<MeasurementIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub value_type: Option<MeasurementValueTypeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub timestamp_interval: Option<commondatatypes::TimestampIntervalType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MeasurementConstraintsDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub measurement_id: Option<MeasurementIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub value_range_min: Option<commondatatypes::ScaledNumberType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub value_range_max: Option<commondatatypes::ScaledNumberType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub value_step_size: Option<commondatatypes::ScaledNumberType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MeasurementConstraintsListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub measurement_constraints_data: Option<Vec<MeasurementConstraintsDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MeasurementConstraintsListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub measurement_id: Option<MeasurementIdType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MeasurementDescriptionDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub measurement_id: Option<MeasurementIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub measurement_type: Option<MeasurementTypeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub commodity_type: Option<commondatatypes::CommodityTypeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub unit: Option<commondatatypes::UnitOfMeasurementType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub calibration_value: Option<commondatatypes::ScaledNumberType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub scope_type: Option<commondatatypes::ScopeTypeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub label: Option<commondatatypes::LabelType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<commondatatypes::DescriptionType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MeasurementDescriptionListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub measurement_description_data: Option<Vec<MeasurementDescriptionDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MeasurementDescriptionListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub measurement_id: Option<MeasurementIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub measurement_type: Option<MeasurementTypeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub commodity_type: Option<commondatatypes::CommodityTypeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub scope_type: Option<commondatatypes::ScopeTypeType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MeasurementThresholdRelationDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub measurement_id: Option<MeasurementIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub threshold_id: Option<Vec<threshold::ThresholdIdType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MeasurementThresholdRelationListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub measurement_threshold_relation_data: Option<Vec<MeasurementThresholdRelationDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MeasurementThresholdRelationListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub measurement_id: Option<MeasurementIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub threshold_id: Option<Vec<threshold::ThresholdIdType>>,
}

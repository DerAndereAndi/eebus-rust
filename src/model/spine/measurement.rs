use serde::{Serialize, Deserialize};
use super::commondatatypes;

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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MeasurementDataType {
	pub measurement_id: Option<MeasurementIdType>,
  pub value_type: Option<MeasurementValueTypeType>,
  pub timestamp: Option<String>,
  pub value: Option<commondatatypes::ScaledNumberType>,
  pub evaluation_period: Option<commondatatypes::TimePeriodType>,
  pub value_source: Option<MeasurementValueSourceType>,
  pub value_tendency: Option<MeasurementValueTendencyType>,
  pub value_state: Option<MeasurementValueStateType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MeasurementListDataType {
  pub measurement_data: Option<Vec<MeasurementDataType>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MeasurementListDataSelectorsType {
  pub measurement_id: Option<MeasurementIdType>,
  pub value_type: Option<MeasurementValueTypeType>,
  pub timestamp_interval: Option<commondatatypes::TimestampIntervalType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MeasurementConstraintsDataType {
  pub measurement_id: Option<MeasurementIdType>,
  pub value_range_min: Option<commondatatypes::ScaledNumberType>,
  pub value_range_max: Option<commondatatypes::ScaledNumberType>,
  pub value_step_size: Option<commondatatypes::ScaledNumberType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MeasurementConstraintsListDataType {
  pub measurement_constraints_data: Option<Vec<MeasurementConstraintsDataType>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MeasurementConstraintsListDataSelectorsType {
  pub measurement_id: Option<MeasurementIdType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MeasurementDescriptionDataType {
  pub measurement_id: Option<MeasurementIdType>,
  pub measurement_type: Option<MeasurementTypeType>,
  pub commodity_type: Option<commondatatypes::CommodityTypeType>,
  pub unit: Option<commondatatypes::UnitOfMeasurementType>,
  pub calibration_value: Option<commondatatypes::ScaledNumberType>,
  pub scope_type: Option<commondatatypes::ScopeTypeType>,
  pub label: Option<commondatatypes::LabelType>,
  pub description: Option<commondatatypes::DescriptionType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MeasurementDescriptionListDataType {
  pub measurement_description_data: Option<Vec<MeasurementDescriptionDataType>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MeasurementDescriptionListDataSelectorsType {
  pub measurement_id: Option<MeasurementIdType>,
  pub measurement_type: Option<MeasurementTypeType>,
  pub commodity_type: Option<commondatatypes::CommodityTypeType>,
  pub scope_type: Option<commondatatypes::ScopeTypeType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MeasurementThresholdRelationDataType {
  pub measurement_id: Option<MeasurementIdType>,
  pub threshold_id: Option<Vec<ThresholdIdType>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MeasurementThresholdRelationListDataType {
  pub measurement_threshold_relation_data: Option<Vec<MeasurementThresholdRelationDataType>>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MeasurementThresholdRelationListDataSelectorsType {
  pub measurement_id: Option<MeasurementIdType>,
  pub threshold_id: Option<Vec<ThresholdIdType>>,
}

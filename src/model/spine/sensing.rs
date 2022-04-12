use std::fmt;

use super::super::utils;

use serde::{Serialize, Deserialize};
use super::{commondatatypes};

pub type SensingStateType = SensingStateEnumType;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum SensingStateEnumType {
  On,
  Off,
  Toggle,
  Level,
  LevelUp,
  LevelDown,
  LevelStart,
  LevelStop,
  LevelAbsolute,
  LevelRelative,
  LevelPercentageAbsolute,
  LevelPercentageRelative,
  Pressed,
  LongPressed,
  Released,
  Changed,
  Started,
  Stopped,
  Paused,
  Middle,
  Up,
  Down,
  Forward,
  Backwards,
  Open,
  Closed,
  Opening,
  Closing,
  High,
  Low,
  Day,
  Night,
  Detected,
  NotDetected,
  Alarmed,
  NotAlarmed,
}

impl fmt::Display for SensingStateEnumType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    utils::provide_enum_display(self, f)
  }
}

pub type SensingTypeType = SensingTypeEnumType;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub enum SensingTypeEnumType {
  Switch,
  Button,
  Level,
  LevelSwitch,
  WindowHandle,
  ContactSensor,
  OccupancySensor,
  MotionDetector,
  FireDetector,
  SmokeDetector,
  HeatDetector,
  WaterDetector,
  GasDetector,
  AlarmSensor,
  PowerAlarmSensor,
  DayNightIndicator,
}

impl fmt::Display for SensingTypeEnumType {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    utils::provide_enum_display(self, f)
  }
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SensingDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub timestamp: Option<commondatatypes::AbsoluteOrRelativeTimeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub state: Option<SensingStateType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub value: Option<commondatatypes::ScaledNumberType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SensingDataElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub timestamp: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub state: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub value: Option<commondatatypes::ScaledNumberElementsType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SensingListDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub sensing_data: Option<Vec<SensingDataType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SensingListDataSelectorsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub timestamp_interval: Option<commondatatypes::TimestampIntervalType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SensingDescriptionDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub sensing_type: Option<SensingTypeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub unit: Option<commondatatypes::UnitOfMeasurementType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub scope_type: Option<commondatatypes::ScopeTypeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub label: Option<commondatatypes::LabelType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<commondatatypes::DescriptionType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct SensingDescriptionDataElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub sensing_type: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub unit: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub scope_type: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub label: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub description: Option<commondatatypes::ElementTagType>,
}

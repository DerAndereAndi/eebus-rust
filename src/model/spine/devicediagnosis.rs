use serde::{Serialize, Deserialize};
use super::commondatatypes;

pub type VendorStateCodeType = String;

pub type LastErrorCodeType = String;

pub type DeviceDiagnosisOperatingStateType = DeviceDiagnosisOperatingStateEnumType;

#[derive(Serialize, Deserialize, Debug)]
pub enum DeviceDiagnosisOperatingStateEnumType {
	#[serde(rename = "normalOperation")]
	NormalOperation,
	#[serde(rename = "standby")]
	Standby,
	#[serde(rename = "failure")]
	Failure,
	#[serde(rename = "serviceNeeded")]
	ServiceNeeded,
	#[serde(rename = "overrideDetected")]
	OverrideDetected,
	#[serde(rename = "inAlarm")]
	InAlarm,
	#[serde(rename = "notReachable")]
	NotReachable,
	#[serde(rename = "finished")]
	Finished,
}

pub type PowerSupplyConditionType = PowerSupplyConditionEnumType;

#[derive(Serialize, Deserialize, Debug)]
pub enum PowerSupplyConditionEnumType {
	#[serde(rename = "good")]
	Good,
	#[serde(rename = "low")]
	Low,
	#[serde(rename = "critical")]
	Critical,
	#[serde(rename = "unknown")]
	Unknown,
	#[serde(rename = "error")]
	Error,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDiagnosisStateDataType {
  timestamp: Option<String>,
  operating_state: Option<DeviceDiagnosisOperatingStateType>,
  vendor_state_code: Option<VendorStateCodeType>,
  last_error_code: Option<LastErrorCodeType>,
  up_time: Option<String>,
  total_up_time: Option<String>,
  power_supply_condition: Option<PowerSupplyConditionType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
type DeviceDiagnosisHeartbeatDataType struct {
  timestamp: Option<String>,
  heartbeat_counter: Option<u64>,
  heartbeat_timeout: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
type DeviceDiagnosisServiceDataType struct {
  timestamp: Option<String>,
  installation_time: Option<String>,
  boot_counter: Option<u64>,
  next_service: Option<String>,
}

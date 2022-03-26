use serde::{Serialize, Deserialize};

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
	#[serde(skip_serializing_if = "Option::is_none")]
  timestamp: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
  operating_state: Option<DeviceDiagnosisOperatingStateType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  vendor_state_code: Option<VendorStateCodeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  last_error_code: Option<LastErrorCodeType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  up_time: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
  total_up_time: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
  power_supply_condition: Option<PowerSupplyConditionType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDiagnosisHeartbeatDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  timestamp: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
  heartbeat_counter: Option<u64>,
	#[serde(skip_serializing_if = "Option::is_none")]
  heartbeat_timeout: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct DeviceDiagnosisServiceDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  timestamp: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
  installation_time: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
  boot_counter: Option<u64>,
	#[serde(skip_serializing_if = "Option::is_none")]
  next_service: Option<String>,
}

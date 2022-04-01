use std::fmt;

use super::super::utils;

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]

pub enum ConnectionHelloPhaseType {
	#[serde(rename = "pending")]
  Pending,
	#[serde(rename = "ready")]
  Ready,
	#[serde(rename = "aborted")]
  Aborted,
}

impl fmt::Display for ConnectionHelloPhaseType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		utils::provide_enum_display(self, f)
	}
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionHelloType {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub phase: Option<ConnectionHelloPhaseType>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub waiting: Option<u8>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub prolongation_request: Option<bool>,
}

pub type MessageProtocolFormatType = String;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct MessageProtocolFormatsType {
  pub format: Vec<MessageProtocolFormatType>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum ProtocolHandshakeTypeType {
	#[serde(rename = "announceMax")]
  AnnounceMax,
	#[serde(rename = "select")]
  Select,
}

impl fmt::Display for ProtocolHandshakeTypeType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		utils::provide_enum_display(self, f)
	}
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Version {
  major: u8,
  minor: u8,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MessageProtocolHandshakeType {
	#[serde(skip_serializing_if = "Option::is_none")]
  handshake_type: Option<ProtocolHandshakeTypeType>, // defined as option so we don't have to provide a default value
  version: Version,
  formats: MessageProtocolFormatsType,
}

pub type MessageProtocolHandshakeErrorErrorType = String;

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MessageProtocolHandshakeErrorType {
  error: MessageProtocolHandshakeErrorErrorType,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum PinStateType {
	#[serde(rename = "required")]
  Required,
	#[serde(rename = "optional")]
  Optional,
	#[serde(rename = "pinOk")]
  PinOk,
	#[serde(rename = "none")]
  None,
}

impl fmt::Display for PinStateType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		utils::provide_enum_display(self, f)
	}
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum PinInputPermissionType {
	#[serde(rename = "busy")]
  Busy,
	#[serde(rename = "ok")]
  Ok,
}

impl fmt::Display for PinInputPermissionType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		utils::provide_enum_display(self, f)
	}
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionPinStateType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pin_state: Option<PinStateType>, // defined as option so we don't have to provide a default value
	#[serde(skip_serializing_if = "Option::is_none")]
  input_permission: Option<PinInputPermissionType>,
}

pub type PinValueType = String;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ConnectionPinInputType {
  pin: PinValueType,
}

pub type ConnectionPinErrorErrorType = u8;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ConnectionPinErrorType {
  error: ConnectionPinErrorErrorType,
}

pub type ProtocolIdType = String;

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HeaderType {
  protocol_id: ProtocolIdType,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionType {
	#[serde(skip_serializing_if = "Option::is_none")]
  extension_id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  binary: Option<String>, // HexBinary
  #[serde(skip_serializing_if = "Option::is_none")]
  string: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct DataType {
  header: HeaderType,
  payload: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  extension: Option<ExtensionType>,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum ConnectionClosePhaseType {
  #[serde(rename = "announce")]
  Announce,
  #[serde(rename = "confirm")]
  Confirm,
}

impl fmt::Display for ConnectionClosePhaseType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		utils::provide_enum_display(self, f)
	}
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum ConnectionCloseReasonType {
  #[serde(rename = "unspecific")]
  Unspecific,
  #[serde(rename = "removedConnection")]
  RemovedConnection,
}

impl fmt::Display for ConnectionCloseReasonType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		utils::provide_enum_display(self, f)
	}
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionCloseType {
  #[serde(skip_serializing_if = "Option::is_none")]
  phase: Option<ConnectionClosePhaseType>, // defined as option so we don't have to provide a default value
  #[serde(skip_serializing_if = "Option::is_none")]
  max_time: Option<u8>,
  #[serde(skip_serializing_if = "Option::is_none")]
  reason: Option<ConnectionCloseReasonType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct AccessMethodsRequestType {
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct DnsSdMdns {
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Dns {
  uri: String,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct AccessMethodsType {
  id: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(rename = "dnsSd_mDns")]
  dns_sd_mdns: Option<DnsSdMdns>,
  #[serde(skip_serializing_if = "Option::is_none")]
  dns: Option<Dns>,
}

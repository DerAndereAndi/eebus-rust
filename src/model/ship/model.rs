use std::fmt;

use super::super::utils;

use serde::{Serialize, Deserialize};

pub enum MessageType {
  Init = 0,
  Control = 1,
  Data = 2,
  End = 3
}

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
pub struct ConnectionHello {
  pub connection_hello: ConnectionHelloType,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionHelloType {
  #[serde(skip_serializing_if = "Option::is_none")]
  pub phase: Option<ConnectionHelloPhaseType>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub waiting: Option<u32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub prolongation_request: Option<bool>,
}

#[derive(Serialize, Deserialize, Debug)]
pub enum MessageProtocolFormatType {
	#[serde(rename = "JSON-UTF8")]
  JsonUTF8,
	#[serde(rename = "JSON-UTF16")]
  JsonUTF16,
}

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
#[serde(rename_all = "camelCase")]
pub struct MessageProtocolHandshake {
  pub message_protocol_handshake: MessageProtocolHandshakeType,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct Version {
  pub major: u32,
  pub minor: u32,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MessageProtocolHandshakeType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub handshake_type: Option<ProtocolHandshakeTypeType>, // defined as option so we don't have to provide a default value
  pub version: Version,
  pub formats: MessageProtocolFormatsType,
}

pub type MessageProtocolHandshakeErrorErrorType = u8;

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct MessageProtocolHandshakeErrorType {
  pub error: MessageProtocolHandshakeErrorErrorType,
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
  pub pin_state: Option<PinStateType>, // defined as option so we don't have to provide a default value
	#[serde(skip_serializing_if = "Option::is_none")]
  pub input_permission: Option<PinInputPermissionType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ConnectionPinState {
  pub connection_pin_state: ConnectionPinStateType,
}

pub type PinValueType = String;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ConnectionPinInputType {
  pub pin: PinValueType,
}

pub type ConnectionPinErrorErrorType = u32;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ConnectionPinErrorType {
  pub error: ConnectionPinErrorErrorType,
}

pub type ProtocolIdType = String;

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct HeaderType {
  pub protocol_id: ProtocolIdType,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct ExtensionType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub extension_id: Option<String>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub binary: Option<String>, // HexBinary
  #[serde(skip_serializing_if = "Option::is_none")]
  pub string: Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct DataType {
  pub header: HeaderType,
  pub payload: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub extension: Option<ExtensionType>,
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
  pub phase: Option<ConnectionClosePhaseType>, // defined as option so we don't have to provide a default value
  #[serde(skip_serializing_if = "Option::is_none")]
  pub max_time: Option<u32>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub reason: Option<ConnectionCloseReasonType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AccessMethodsRequest {
  pub access_methods_request: AccessMethodsRequestType,
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
#[serde(rename_all = "camelCase")]
pub struct AccessMethods {
  pub access_methods: AccessMethodsType,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct AccessMethodsType {
  pub id: String,
  #[serde(skip_serializing_if = "Option::is_none")]
  #[serde(rename = "dnsSd_mDns")]
  pub dns_sd_mdns: Option<DnsSdMdns>,
  #[serde(skip_serializing_if = "Option::is_none")]
  pub dns: Option<Dns>,
}

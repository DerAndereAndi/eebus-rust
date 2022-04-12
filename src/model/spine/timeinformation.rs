use serde::{Serialize, Deserialize};
use super::{commondatatypes};

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeInformationDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub utc: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub utc_offset: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub day_of_week: Option<commondatatypes::DayOfWeekType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub calendar_week: Option<commondatatypes::CalendarWeekType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeInformationDataElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub utc: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub utc_offset: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub day_of_week: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub calendar_week: Option<commondatatypes::ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeDistributorDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub is_time_distributor: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub distributor_priority: Option<u32>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeDistributorDataElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub is_time_distributor: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub distributor_priority: Option<commondatatypes::ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimePrecisionDataType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub is_synchronised: Option<bool>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub last_sync_at: Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub clock_drift: Option<i32>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimePrecisionDataElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
  pub is_synchronised: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub last_sync_at: Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
  pub clock_drift: Option<commondatatypes::ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeDistributorEnquiryCallType {}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimeDistributorEnquiryCallElementsType {}

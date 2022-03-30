use serde::{Serialize, Deserialize};
use super::{commondatatypes, electricalconnection, measurement, deviceclassification, deviceconfiguration, devicediagnosis, identification, incentivetable, loadcontrol, timeseries, nodemanagement, result};

pub type MsgCounterType = u64;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum CmdClassifierType {
    Read,
    Reply,
    Notify,
    Write,
    Call,
    Result,
}

pub type FilterIdType = u8;

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FilterType {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub filter_id: Option<FilterIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub cmd_control: Option<CmdControlType>,

	// DataSelectorsChoiceGroup
	#[serde(skip_serializing_if = "Option::is_none")]
	pub electrical_connection_description_list_data_selectors: Option<electricalconnection::ElectricalConnectionDescriptionListDataSelectorsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub electrical_connection_parameter_description_list_data_selectors: Option<electricalconnection::ElectricalConnectionParameterDescriptionListDataSelectorsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub measurement_description_list_data_selectors: Option<measurement::MeasurementDescriptionListDataSelectorsType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub measurement_list_data_selectors: Option<measurement::MeasurementListDataSelectorsType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct CmdControlType {
	#[serde(skip_serializing_if = "Option::is_none")]
	delete:  Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	partial: Option<commondatatypes::ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Filter {
	#[serde(skip_serializing_if = "Option::is_none")]
	filter_id:   Option<FilterIdType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	cmd_control: Option<CmdControlType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct CmdControl {
	#[serde(skip_serializing_if = "Option::is_none")]
	delete:  Option<commondatatypes::ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	partial: Option<commondatatypes::ElementTagType>
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CmdType {
	// CmdOptionGroup
	#[serde(skip_serializing_if = "Option::is_none")]
	pub function: Option<commondatatypes::FunctionType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub filter:   Option<Vec<FilterType>>,

	// DataChoiceGroup
	#[serde(skip_serializing_if = "Option::is_none")]
	pub device_classification_manufacturer_data: Option<deviceclassification::DeviceClassificationManufacturerDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub device_configuration_key_value_description_list_data: Option<deviceconfiguration::DeviceConfigurationKeyValueDescriptionListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub device_configuration_key_value_list_data: Option<deviceconfiguration::DeviceConfigurationKeyValueListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub device_diagnosis_heartbeat_data: Option<devicediagnosis::DeviceDiagnosisHeartbeatDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub device_diagnosis_state_data: Option<devicediagnosis::DeviceDiagnosisStateDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub electrical_connection_description_list_data: Option<electricalconnection::ElectricalConnectionDescriptionListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub electrical_connection_parameter_description_list_data: Option<electricalconnection::ElectricalConnectionParameterDescriptionListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub electrical_connection_permitted_value_set_list_data: Option<electricalconnection::ElectricalConnectionPermittedValueSetListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub identification_list_data: Option<identification::IdentificationListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub incentive_table_description_data: Option<incentivetable::IncentiveTableDescriptionDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub incentive_table_constraints_data: Option<incentivetable::IncentiveTableConstraintsDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub incentive_table_data: Option<incentivetable::IncentiveTableDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub load_control_limit_description_list_data: Option<loadcontrol::LoadControlLimitDescriptionListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub load_control_limit_list_data: Option<loadcontrol::LoadControlLimitListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub node_management_binding_request_call: Option<nodemanagement::NodeManagementBindingRequestCallType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub node_management_detailed_discovery_data: Option<nodemanagement::NodeManagementDetailedDiscoveryDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub node_management_subscription_data: Option<nodemanagement::NodeManagementSubscriptionDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub node_management_subscription_request_call: Option<nodemanagement::NodeManagementSubscriptionRequestCallType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub node_management_subscription_delete_call: Option<nodemanagement::NodeManagementSubscriptionDeleteCallType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub node_management_use_case_data: Option<nodemanagement::NodeManagementUseCaseDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub measurement_constraints_list_data: Option<measurement::MeasurementConstraintsListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub measurement_description_list_data: Option<measurement::MeasurementDescriptionListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub measurement_list_data: Option<measurement::MeasurementListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub reulst_data: Option<result::ResultDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub time_series_constraints_data: Option<timeseries::TimeSeriesConstraintsDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub time_series_constraints_list_data: Option<timeseries::TimeSeriesConstraintsListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub time_series_description_list_data: Option<timeseries::TimeSeriesDescriptionListDataType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub time_series_list_data: Option<timeseries::TimeSeriesListDataType>,

	// DataExtendGroup
}

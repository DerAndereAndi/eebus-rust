use serde::{Serialize, Deserialize};
use super::{commondatatypes, electricalconnection, measurement, deviceclassification, deviceconfiguration, devicediagnosis, identification, incentivetable, loadcontrol, timeseries, nodemanagement, result};

pub type MsgCounterType = u64;

#[derive(Serialize, Deserialize, Debug)]
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

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FilterType {
	pub filter_id: Option<FilterIdType>,
	pub cmd_control: Option<CmdControlType>,

	// DataSelectorsChoiceGroup
	pub electrical_connection_description_list_data_selectors: Option<electricalconnection::ElectricalConnectionDescriptionListDataSelectorsType>,
	pub electrical_connection_parameter_description_list_data_selectors: Option<electricalconnection::ElectricalConnectionParameterDescriptionListDataSelectorsType>,
	pub measurement_description_list_data_selectors: Option<measurement::MeasurementDescriptionListDataSelectorsType>,
	pub measurement_list_data_selectors: Option<measurement::MeasurementListDataSelectorsType>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CmdControlType {
	delete:  Option<commondatatypes::ElementTagType>,
	partial: Option<commondatatypes::ElementTagType>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Filter {
	filter_id:   Option<FilterIdType>,
	cmd_control: Option<CmdControlType>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CmdControl {
	delete:  Option<commondatatypes::ElementTagType>,
	partial: Option<commondatatypes::ElementTagType>
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct CmdType {
	// CmdOptionGroup
	pub function: Option<commondatatypes::FunctionType>,
	pub filter:   Option<Vec<FilterType>>,

	// DataChoiceGroup
	pub device_classification_manufacturer_data: Option<deviceclassification::DeviceClassificationManufacturerDataType>,
	pub device_configuration_key_value_description_list_data: Option<deviceconfiguration::DeviceConfigurationKeyValueDescriptionListDataType>,
	pub device_configuration_key_value_list_data: Option<deviceconfiguration::DeviceConfigurationKeyValueListDataType>,
	pub device_diagnosis_heartbeat_data: Option<devicediagnosis::DeviceDiagnosisHeartbeatDataType>,
	pub device_diagnosis_state_data: Option<devicediagnosis::DeviceDiagnosisStateDataType>,
	pub electrical_connection_description_list_data: Option<electricalconnection::ElectricalConnectionDescriptionListDataType>,
	pub electrical_connection_parameter_description_list_data: Option<electricalconnection::ElectricalConnectionParameterDescriptionListDataType>,
	pub electrical_connection_permitted_value_set_list_data: Option<electricalconnection::ElectricalConnectionPermittedValueSetListDataType>,
	pub identification_list_data: Option<identification::IdentificationListDataType>,
	pub incentive_table_description_data: Option<incentivetable::IncentiveTableDescriptionDataType>,
	pub incentive_table_constraints_data: Option<incentivetable::IncentiveTableConstraintsDataType>,
	pub incentive_table_data: Option<incentivetable::IncentiveTableDataType>,
	pub load_control_limit_description_list_data: Option<loadcontrol::LoadControlLimitDescriptionListDataType>,
	pub load_control_limit_list_data: Option<loadcontrol::LoadControlLimitListDataType>,
	pub node_management_binding_request_call: Option<nodemanagement::NodeManagementBindingRequestCallType>,
	pub node_management_detailed_discovery_data: Option<nodemanagement::NodeManagementDetailedDiscoveryDataType>,
	pub node_management_subscription_data: Option<nodemanagement::NodeManagementSubscriptionDataType>,
	pub node_management_subscription_request_call: Option<nodemanagement::NodeManagementSubscriptionRequestCallType>,
	pub node_management_subscription_delete_call: Option<nodemanagement::NodeManagementSubscriptionDeleteCallType>,
	pub node_management_use_case_data: Option<nodemanagement::NodeManagementUseCaseDataType>,
	pub measurement_constraints_list_data: Option<measurement::MeasurementConstraintsListDataType>,
	pub measurement_description_list_data: Option<measurement::MeasurementDescriptionListDataType>,
	pub measurement_list_data: Option<measurement::MeasurementListDataType>,
	pub reulst_data: Option<result::ResultDataType>,
	pub time_series_constraints_data: Option<timeseries::TimeSeriesConstraintsDataType>,
	pub time_series_constraints_list_data: Option<timeseries::TimeSeriesConstraintsListDataType>,
	pub time_series_description_list_data: Option<timeseries::TimeSeriesDescriptionListDataType>,
	pub time_series_list_data: Option<timeseries::TimeSeriesListDataType>,

	// DataExtendGroup
}

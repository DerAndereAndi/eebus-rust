use serde::{Serialize, Deserialize};
use super::commondatatypes;

pub type MsgCounterType = u64;

#[derive(Serialize, Deserialize, Debug)]
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
	// ElectricalConnectionDescriptionListDataSelectors          *ElectricalConnectionDescriptionListDataSelectorsType          `json:"electricalConnectionDescriptionListDataSelectors,omitempty"`
	// ElectricalConnectionParameterDescriptionListDataSelectors *ElectricalConnectionParameterDescriptionListDataSelectorsType `json:"electricalConnectionParameterDescriptionListDataSelectors,omitempty"`
	// MeasurementDescriptionListDataSelectors                   *MeasurementDescriptionListDataSelectorsType                   `json:"measurementDescriptionListDataSelectors,omitempty"`
	// MeasurementListDataSelectors                              *MeasurementListDataSelectorsType                              `json:"measurementListDataSelectors,omitempty"`
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
	// pub function: Option<FunctionType>,
	// pub filter:   Option<Vec<FilterType>>,

	// DataChoiceGroup
	// DeviceClassificationManufacturerData             *DeviceClassificationManufacturerDataType             `json:"deviceClassificationManufacturerData,omitempty"`
	// DeviceConfigurationKeyValueDescriptionListData   *DeviceConfigurationKeyValueDescriptionListDataType   `json:"deviceConfigurationKeyValueDescriptionListData,omitempty"`
	// DeviceConfigurationKeyValueListData              *DeviceConfigurationKeyValueListDataType              `json:"deviceConfigurationKeyValueListData,omitempty"`
	// DeviceDiagnosisHeartbeatData                     *DeviceDiagnosisHeartbeatDataType                     `json:"deviceDiagnosisHeartbeatData,omitempty"`
	// DeviceDiagnosisStateData                         *DeviceDiagnosisStateDataType                         `json:"deviceDiagnosisStateData,omitempty"`
	// ElectricalConnectionDescriptionListData          *ElectricalConnectionDescriptionListDataType          `json:"electricalConnectionDescriptionListData,omitempty"`
	// ElectricalConnectionParameterDescriptionListData *ElectricalConnectionParameterDescriptionListDataType `json:"electricalConnectionParameterDescriptionListData,omitempty"`
	// ElectricalConnectionPermittedValueSetListData    *ElectricalConnectionPermittedValueSetListDataType    `json:"electricalConnectionPermittedValueSetListData,omitempty"`
	// IdentificationListData                           *IdentificationListDataType                           `json:"identificationListData,omitempty"`
	// IncentiveTableDescriptionData                    *IncentiveTableDescriptionDataType                    `json:"incentiveTableDescriptionData,omitempty"`
	// IncentiveTableConstraintsData                    *IncentiveTableConstraintsDataType                    `json:"incentiveTableConstraintsData,omitempty"`
	// IncentiveTableData                               *IncentiveTableDataType                               `json:"incentiveTableData,omitempty"`
	// LoadControlLimitDescriptionListData              *LoadControlLimitDescriptionListDataType              `json:"loadControlLimitDescriptionListData,omitempty"`
	// LoadControlLimitListData                         *LoadControlLimitListDataType                         `json:"loadControlLimitListData,omitempty"`
	// NodeManagementBindingRequestCall                 *NodeManagementBindingRequestCallType                 `json:"nodeManagementBindingRequestCall,omitempty"`
	// NodeManagementDetailedDiscoveryData              *NodeManagementDetailedDiscoveryDataType              `json:"nodeManagementDetailedDiscoveryData,omitempty"`
	// NodeManagementSubscriptionData                   *NodeManagementSubscriptionDataType                   `json:"nodeManagementSubscriptionData,omitempty"`
	// NodeManagementSubscriptionRequestCall            *NodeManagementSubscriptionRequestCallType            `json:"nodeManagementSubscriptionRequestCall,omitempty"`
	// NodeManagementSubscriptionDeleteCall             *NodeManagementSubscriptionDeleteCallType             `json:"nodeManagementSubscriptionDeleteCall,omitempty"`
	// NodeManagementUseCaseData                        *NodeManagementUseCaseDataType                        `json:"nodeManagementUseCaseData,omitempty"`
	// MeasurementConstraintsListData                   *MeasurementConstraintsListDataType                   `json:"measurementConstraintsListData,omitempty"`
	// MeasurementDescriptionListData                   *MeasurementDescriptionListDataType                   `json:"measurementDescriptionListData,omitempty"`
	// MeasurementListData                              *MeasurementListDataType                              `json:"measurementListData,omitempty"`
	// ResultData                                       *ResultDataType                                       `json:"resultData,omitempty"`
	// TimeSeriesConstraintsData                        *TimeSeriesConstraintsDataType                        `json:"timeSeriesConstraintsData,omitempty"`
	// TimeSeriesConstraintsListData                    *TimeSeriesConstraintsListDataType                    `json:"timeSeriesConstraintsListData,omitempty"`
	// TimeSeriesDescriptionListData                    *TimeSeriesDescriptionListDataType                    `json:"timeSeriesDescriptionListData,omitempty"`
	// TimeSeriesListData                               *TimeSeriesListDataType                               `json:"timeSeriesListData,omitempty"`

	// DataExtendGroup
}

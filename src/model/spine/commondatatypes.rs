#![allow(dead_code)]

use std::fmt;

use super::super::utils;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ElementTagType {
}

pub type LabelType = String;

pub type DescriptionType = String;

pub type SpecificationVersionType = String;

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimePeriodType {
	pub start_time: Option<String>,
	pub end_time:   Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimePeriodElementsType {
	pub start_time: Option<ElementTagType>,
	pub end_time:   Option<ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct TimestampIntervalType {
	pub start_time: Option<String>,
	pub end_time:   Option<String>,
}

pub type AbsoluteOrRelativeTimeType = String;

pub type RecurringIntervalType = RecurringIntervalEnumType;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum RecurringIntervalEnumType {
    Yearly,
    Monthly,
    Weekly,
    Daily,
    Hourly,
    EveryMinute,
		EverySecond,
}

impl fmt::Display for RecurringIntervalEnumType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		utils::provide_enum_display(self, f)
	}
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum MonthType {
    January,
    February,
    March,
    April,
    May,
    June,
		July,
		August,
		September,
		October,
		November,
		December,
}

impl fmt::Display for MonthType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		utils::provide_enum_display(self, f)
	}
}

pub type DayOfMonthType = u8;

pub type CalendarWeekType = u8;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum DayOfWeekType {
    Monday,
    Tuesday,
    Wednesday,
    Thursday,
    Friday,
    Saturday,
		Sunday,
}

impl fmt::Display for DayOfWeekType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		utils::provide_enum_display(self, f)
	}
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct DaysOfWeekType {
	pub monday:    Option<ElementTagType>,
	pub tuesday:   Option<ElementTagType>,
	pub wednesday: Option<ElementTagType>,
	pub thursday:  Option<ElementTagType>,
	pub friday:    Option<ElementTagType>,
	pub saturday:  Option<ElementTagType>,
	pub sunday:    Option<ElementTagType>,
}

pub type OccurrenceType = String;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum OccurrenceEnumType {
	#[serde(rename = "first")]
	First,
	#[serde(rename = "second")]
	Second,
	#[serde(rename = "third")]
	Third,
	#[serde(rename = "fourth")]
	Fourth,
	#[serde(rename = "last")]
	Last,
}

impl fmt::Display for OccurrenceEnumType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		utils::provide_enum_display(self, f)
	}
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AbsoluteOrRecurringTimeType {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub date_time:              Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub month:                  Option<MonthType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub day_of_month:           Option<DayOfMonthType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub calendar_week:          Option<CalendarWeekType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub day_of_week_occurrence: Option<OccurrenceType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub days_of_week:           Option<DaysOfWeekType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub time:                   Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub relative:            		Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct AbsoluteOrRecurringTimeElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub date_time:              Option<ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub month:                  Option<ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub day_of_month:           Option<ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub calendar_week:          Option<ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub day_of_week_occurrence: Option<ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub days_of_week:           Option<ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub time:                   Option<ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub relative:               Option<ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RecurrenceInformationType {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub recurring_interval:      Option<RecurringIntervalType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub recurring_interval_step: Option<u32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub first_execution:         Option<String>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub execution_count:         Option<u32>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub last_execution:          Option<String>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct RecurrenceInformationElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub recurring_interval:      Option<ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub recurring_interval_step: Option<ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub first_execution:         Option<ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub execution_count:         Option<ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub last_execution:          Option<ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ScaledNumberRangeType {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub min: Option<ScaledNumberType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max: Option<ScaledNumberType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ScaledNumberRangeElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub min: Option<ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub max: Option<ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ScaledNumberSetType {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub value: Option<Vec<ScaledNumberType>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub range: Option<Vec<ScaledNumberRangeType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ScaledNumberSetElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub value: Option<ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub range: Option<ScaledNumberRangeElementsType>,
}

pub type NumberType = i64;

pub type ScaleType = i8;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ScaledNumberType {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub number: Option<NumberType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub scale:  Option<ScaleType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct ScaledNumberElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub number: Option<ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub scale:  Option<ElementTagType>,
}

pub type MaxResponseDelayType = String;

pub type CommodityTypeType = CommodityTypeEnumType;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum CommodityTypeEnumType {
	#[serde(rename = "electricity")]
	Electricity,
	#[serde(rename = "gas")]
	Gas,
	#[serde(rename = "oil")]
	Oil,
	#[serde(rename = "water")]
	Water,
	#[serde(rename = "wasteWater")]
	WasteWater,
	#[serde(rename = "domesticHotWater")]
	DomesticHotWater,
	#[serde(rename = "heatingWater")]
	HeatingWater,
	#[serde(rename = "steam")]
	Steam,
	#[serde(rename = "heat")]
	Heat,
	#[serde(rename = "coolingLoad")]
	CoolingLoad,
	#[serde(rename = "air")]
	Air,
}

impl fmt::Display for CommodityTypeEnumType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		utils::provide_enum_display(self, f)
	}
}

pub type EnergyDirectionType = EnergyDirectionEnumType;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum EnergyDirectionEnumType {
	#[serde(rename = "consume")]
	Consume,
	#[serde(rename = "produce")]
	Produce,
}

impl fmt::Display for EnergyDirectionEnumType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		utils::provide_enum_display(self, f)
	}
}

pub type EnergyModeType = EnergyModeEnumType;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum EnergyModeEnumType {
	#[serde(rename = "consume")]
	Consume,
	#[serde(rename = "produce")]
	Produce,
	#[serde(rename = "idle")]
	Idle,
	#[serde(rename = "auto")]
	Auto,
}

impl fmt::Display for EnergyModeEnumType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		utils::provide_enum_display(self, f)
	}
}

pub type UnitOfMeasurementType = UnitOfMeasurementEnumType;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum UnitOfMeasurementEnumType {
	#[serde(rename = "unknown")]
	Unknown,
	#[serde(rename = "1")]
	One,
	#[serde(rename = "m")]
	M,
	#[serde(rename = "kg")]
	Kg,
	#[serde(rename = "s")]
	Ssmall,
	#[serde(rename = "A")]
	A,
	#[serde(rename = "K")]
	K,
	#[serde(rename = "mol")]
	Mol,
	#[serde(rename = "cd")]
	Cd,
	#[serde(rename = "V")]
	V,
	#[serde(rename = "W")]
	W,
	#[serde(rename = "Wh")]
	Wh,
	#[serde(rename = "VA")]
	Va,
	#[serde(rename = "VAh")]
	Vah,
	#[serde(rename = "var")]
	Var,
	#[serde(rename = "varh")]
	Varh,
	#[serde(rename = "degC")]
	DegC,
	#[serde(rename = "degF")]
	DegF,
	#[serde(rename = "Lm")]
	Lm,
	#[serde(rename = "lx")]
	Lx,
	#[serde(rename = "Ohm")]
	Ohm,
	#[serde(rename = "Hz")]
	Hz,
	#[serde(rename = "dB")]
	Db,
	#[serde(rename = "dBm")]
	Dbm,
	#[serde(rename = "pct")]
	Pct,
	#[serde(rename = "ppm")]
	Ppm,
	#[serde(rename = "l")]
	L,
	#[serde(rename = "l/s")]
	Ls,
	#[serde(rename = "l/h")]
	Lh,
	#[serde(rename = "deg")]
	Deg,
	#[serde(rename = "rad")]
	Rad,
	#[serde(rename = "rad/s")]
	Rads,
	#[serde(rename = "sr")]
	Sr,
	#[serde(rename = "Gy")]
	Gy,
	#[serde(rename = "Bq")]
	Bq,
	#[serde(rename = "Bq/m^3")]
	Bqm3,
	#[serde(rename = "Sv")]
	Sv,
	#[serde(rename = "Rd")]
	Rd,
	#[serde(rename = "C")]
	C,
	#[serde(rename = "F")]
	F,
	#[serde(rename = "H")]
	H,
	#[serde(rename = "J")]
	J,
	#[serde(rename = "N")]
	N,
	#[serde(rename = "N_m")]
	Nm,
	#[serde(rename = "N_s")]
	Ns,
	#[serde(rename = "Wb")]
	Wb,
	#[serde(rename = "T")]
	T,
	#[serde(rename = "Pa")]
	Pa,
	#[serde(rename = "bar")]
	Bar,
	#[serde(rename = "atm")]
	Atm,
	#[serde(rename = "psi")]
	Psi,
	#[serde(rename = "mmHg")]
	MmHg,
	#[serde(rename = "m^2")]
	M2,
	#[serde(rename = "m^3")]
	M3,
	#[serde(rename = "m^3/h")]
	M3h,
	#[serde(rename = "m/s")]
	Ms,
	#[serde(rename = "m/s^2")]
	Ms2,
	#[serde(rename = "m^3/s")]
	M3s,
	#[serde(rename = "m/m^3")]
	Mm3,
	#[serde(rename = "kg/m^3")]
	Kgm3,
	#[serde(rename = "kg_m")]
	Kgm,
	#[serde(rename = "m^2/s")]
	M2s,
	#[serde(rename = "W/m_K")]
	WmK,
	#[serde(rename = "J/K")]
	JK,
	#[serde(rename = "1/s")]
	OneS,
	#[serde(rename = "W/m^2")]
	Wm2,
	#[serde(rename = "J/m^2")]
	Jm2,
	#[serde(rename = "S")]
	S,
	#[serde(rename = "S/m")]
	Sm,
	#[serde(rename = "K/s")]
	Ks,
	#[serde(rename = "Pa/s")]
	Pas,
	#[serde(rename = "J/kg_K")]
	JkgK,
	#[serde(rename = "Vs")]
	Vs,
	#[serde(rename = "V/m")]
	Vm,
	#[serde(rename = "V/Hz")]
	VHz,
	#[serde(rename = "As")]
	As,
	#[serde(rename = "A/m")]
	Am,
	#[serde(rename = "Hz/s")]
	Hzs,
	#[serde(rename = "kg/s")]
	Kgs,
	#[serde(rename = "kg_m^2")]
	Kgm2,
	#[serde(rename = "J/Wh")]
	JWh,
	#[serde(rename = "W/s")]
	Ws,
	#[serde(rename = "ft^3")]
	Ft3,
	#[serde(rename = "ft^3/h")]
	Ft3h,
	#[serde(rename = "ccf")]
	Ccf,
	#[serde(rename = "ccf/h")]
	Ccfh,
	#[serde(rename = "US.liq.gal")]
	UsLiqGal,
	#[serde(rename = "US.liq.gal/h")]
	UsLiqGalh,
	#[serde(rename = "img.gal")]
	ImgGal,
	#[serde(rename = "img.gal/h")]
	ImgGalh,
	#[serde(rename = "Btu")]
	Btu,
	#[serde(rename = "Btu/h")]
	Btuh,
	#[serde(rename = "Ah")]
	Ah,
	#[serde(rename = "kg/Wh")]
	KgWh,
}

impl fmt::Display for UnitOfMeasurementEnumType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		utils::provide_enum_display(self, f)
	}
}

pub type CurrencyType = CurrencyEnumType;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[serde(rename_all = "UPPERCASE")]
pub enum CurrencyEnumType {
	AED,
	AFN,
	ALL,
	AMD,
	ANG,
	AOA,
	ARS,
	AUD,
	AWG,
	AZN,
	BAM,
	BBD,
	BDT,
	BGN,
	BHD,
	BIF,
	BMD,
	BND,
	BOB,
	BOV,
	BRL,
	BSD,
	BTN,
	BWP,
	BYR,
	BZD,
	CAD,
	CDF,
	CHE,
	CHF,
	CHW,
	CLF,
	CLP,
	CNY,
	COP,
	COU,
	CRC,
	CUC,
	CUP,
	CVE,
	CZK,
	DJF,
	DKK,
	DOP,
	DZD,
	EGP,
	ERN,
	ETB,
	EUR,
	FJD,
	FKP,
	GBP,
	GEL,
	GHS,
	GIP,
	GMD,
	GNF,
	GTQ,
	GYD,
	HKD,
	HNL,
	HRK,
	HTG,
	HUF,
	IDR,
	ILS,
	INR,
	IQD,
	IRR,
	ISK,
	JMD,
	JOD,
	JPY,
	KES,
	KGS,
	KHR,
	KMF,
	KPW,
	KRW,
	KWD,
	KYD,
	KZT,
	LAK,
	LBP,
	LKR,
	LRD,
	LSL,
	LYD,
	MAD,
	MDL,
	MGA,
	MKD,
	MMK,
	MNT,
	MOP,
	MRO,
	MUR,
	MVR,
	MWK,
	MXN,
	MXV,
	MYR,
	MZN,
	NAD,
	NGN,
	NIO,
	NOK,
	NPR,
	NZD,
	OMR,
	PAB,
	PEN,
	PGK,
	PHP,
	PKR,
	PLN,
	PYG,
	QAR,
	RON,
	RSD,
	RUB,
	RWF,
	SAR,
	SBD,
	SCR,
	SDG,
	SEK,
	SGD,
	SHP,
	SLL,
	SOS,
	SRD,
	SSP,
	STD,
	SVC,
	SYP,
	SZL,
	THB,
	TJS,
	TMT,
	TND,
	TOP,
	TRY,
	TTD,
	TWD,
	TZS,
	UAH,
	UGX,
	USD,
	USN,
	UYI,
	UYU,
	UZS,
	VEF,
	VND,
	VUV,
	WST,
	XAF,
	XAG,
	XAU,
	XBA,
	XBB,
	XBC,
	XBD,
	XCD,
	XDR,
	XOF,
	XPD,
	XPF,
	XPT,
	XSU,
	XTS,
	XUA,
	XXX,
	YER,
	ZAR,
	ZMW,
	ZWL,
}

impl fmt::Display for CurrencyEnumType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		utils::provide_enum_display(self, f)
	}
}

pub type AddressDeviceType = String;

pub type AddressEntityType = u32;

pub type AddressFeatureType = u32;

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct DeviceAddressType {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub device: Option<AddressDeviceType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct DeviceAddressElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub device: Option<ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct EntityAddressType {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub device: Option<AddressDeviceType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub entity: Option<Vec<AddressEntityType>>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct EntityAddressElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub device: Option<ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub entity: Option<ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct FeatureAddressType {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub device:  Option<AddressDeviceType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub entity:  Option<Vec<AddressEntityType>>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub feature: Option<AddressFeatureType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct FeatureAddressElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub device:  Option<ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub entity:  Option<ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub feature: Option<ElementTagType>,
}

pub type ScopeTypeType = ScopeTypeEnumType;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum ScopeTypeEnumType {
	#[serde(rename = "ac")]
	Ac,
	#[serde(rename = "acCosPhiGrid")]
	AcCosPhiGrid,
	#[serde(rename = "acCurrentA")]
	AcCurrentA,
	#[serde(rename = "acCurrentB")]
	AcCurrentB,
	#[serde(rename = "acCurrentC")]
	AcCurrentC,
	#[serde(rename = "acFrequencyGrid")]
	AcFrequencyGrid,
	#[serde(rename = "acPowerA")]
	AcPowerA,
	#[serde(rename = "acPowerB")]
	AcPowerB,
	#[serde(rename = "acPowerC")]
	AcPowerC,
	#[serde(rename = "acPowerLimitPct")]
	AcPowerLimitPct,
	#[serde(rename = "acPowerTotal")]
	AcPowerTotal,
	#[serde(rename = "acVoltageA")]
	AcVoltageA,
	#[serde(rename = "acVoltageB")]
	AcVoltageB,
	#[serde(rename = "acVoltageC")]
	AcVoltageC,
	#[serde(rename = "acYieldDay")]
	AcYieldDay,
	#[serde(rename = "acYieldTotal")]
	AcYieldTotal,
	#[serde(rename = "dcCurrent")]
	DcCurrent,
	#[serde(rename = "dcPower")]
	DcPower,
	#[serde(rename = "dcString1")]
	DcString1,
	#[serde(rename = "dcString2")]
	DcString2,
	#[serde(rename = "dcString3")]
	DcString3,
	#[serde(rename = "dcString4")]
	DcString4,
	#[serde(rename = "dcString5")]
	DcString5,
	#[serde(rename = "dcString6")]
	DcString6,
	#[serde(rename = "dcTotal")]
	DcTotal,
	#[serde(rename = "dcVoltage")]
	DcVoltage,
	#[serde(rename = "dhwTemperature")]
	DhwTemperature,
	#[serde(rename = "flowTemperature")]
	FlowTemperature,
	#[serde(rename = "outsideAirTemperature")]
	OutsideAirTemperature,
	#[serde(rename = "returnTemperature")]
	ReturnTemperature,
	#[serde(rename = "roomAirTemperature")]
	RoomAirTemperature,
	#[serde(rename = "charge")]
	Charge,
	#[serde(rename = "stateOfCharge")]
	StateOfCharge,
	#[serde(rename = "discharge")]
	Discharge,
	#[serde(rename = "gridConsumption")]
	GridConsumption,
	#[serde(rename = "gridFeedIn")]
	GridFeedIn,
	#[serde(rename = "selfConsumption")]
	SelfConsumption,
	#[serde(rename = "overloadProtection")]
	OverloadProtection,
	#[serde(rename = "acPower")]
	AcPower,
	#[serde(rename = "acEnergy")]
	AcEnergy,
	#[serde(rename = "acCurrent")]
	AcCurrent,
	#[serde(rename = "acVoltage")]
	AcVoltage,
	#[serde(rename = "batteryControl")]
	BatteryControl,
	#[serde(rename = "simpleIncentiveTable")]
	SimpleIncentiveTable,
}

impl fmt::Display for ScopeTypeEnumType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		utils::provide_enum_display(self, f)
	}
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum RoleType {
	Client,
	Server,
	Special,
}

impl fmt::Display for RoleType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		utils::provide_enum_display(self, f)
	}
}

pub type FeatureGroupType = String;

pub type DeviceTypeType = DeviceTypeEnumType;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum DeviceTypeEnumType {
	#[serde(rename = "Dishwasher")]
	Dishwasher,
	#[serde(rename = "Dryer")]
	Dryer,
	#[serde(rename = "EnvironmentSensor")]
	EnvironmentSensor,
	#[serde(rename = "Generic")]
	Generic,
	#[serde(rename = "HeatGenerationSystem")]
	HeatGenerationSystem,
	#[serde(rename = "HeatSinkSystem")]
	HeatSinkSystem,
	#[serde(rename = "HeatStorageSystem")]
	HeatStorageSystem,
	#[serde(rename = "HVACController")]
	HVACController,
	#[serde(rename = "SubMeter")]
	SubMeter,
	#[serde(rename = "Washer")]
	Washer,
	#[serde(rename = "ElectricitySupplySystem")]
	ElectricitySupplySystem,
	#[serde(rename = "EnergyManagementSystem")]
	EnergyManagementSystem,
	#[serde(rename = "Inverter")]
	Inverter,
	#[serde(rename = "ChargingStation")]
	ChargingStation,
}

impl fmt::Display for DeviceTypeEnumType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		utils::provide_enum_display(self, f)
	}
}

pub type EntityTypeType = EntityTypeEnumType;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum EntityTypeEnumType {
	#[serde(rename = "Battery")]
	Battery,
	#[serde(rename = "Compressor")]
	Compressor,
	#[serde(rename = "DeviceInformation")]
	DeviceInformation,
	#[serde(rename = "DHWCircuit")]
	DHWCircuit,
	#[serde(rename = "DHWStorage")]
	DHWStorage,
	#[serde(rename = "Dishwasher")]
	Dishwasher,
	#[serde(rename = "Dryer")]
	Dryer,
	#[serde(rename = "ElectricalImmersionHeater")]
	ElectricalImmersionHeater,
	#[serde(rename = "Fan")]
	Fan,
	#[serde(rename = "GasHeatingAppliance")]
	GasHeatingAppliance,
	#[serde(rename = "Generic")]
	Generic,
	#[serde(rename = "HeatingBufferStorage")]
	HeatingBufferStorage,
	#[serde(rename = "HeatingCircuit")]
	HeatingCircuit,
	#[serde(rename = "HeatingObject")]
	HeatingObject,
	#[serde(rename = "HeatingZone")]
	HeatingZone,
	#[serde(rename = "HeatPumpAppliance")]
	HeatPumpAppliance,
	#[serde(rename = "HeatSinkCircuit")]
	HeatSinkCircuit,
	#[serde(rename = "HeatSourceCircuit")]
	HeatSourceCircuit,
	#[serde(rename = "HeatSourceUnit")]
	HeatSourceUnit,
	#[serde(rename = "HVACController")]
	HVACController,
	#[serde(rename = "HVACRoom")]
	HVACRoom,
	#[serde(rename = "InstantDHWHeater")]
	InstantDHWHeater,
	#[serde(rename = "Inverter")]
	Inverter,
	#[serde(rename = "OilHeatingAppliance")]
	OilHeatingAppliance,
	#[serde(rename = "Pump")]
	Pump,
	#[serde(rename = "RefrigerantCircuit")]
	RefrigerantCircuit,
	#[serde(rename = "SmartEnergyAppliance")]
	SmartEnergyAppliance,
	#[serde(rename = "SolarDHWStorage")]
	SolarDHWStorage,
	#[serde(rename = "SolarThermalCircuit")]
	SolarThermalCircuit,
	#[serde(rename = "SubMeterElectricity")]
	SubMeterElectricity,
	#[serde(rename = "TemperatureSensor")]
	TemperatureSensor,
	#[serde(rename = "Washer")]
	Washer,
	#[serde(rename = "BatterySystem")]
	BatterySystem,
	#[serde(rename = "ElectricityGenerationSystem")]
	ElectricityGenerationSystem,
	#[serde(rename = "ElectricityStorageSystem")]
	ElectricityStorageSystem,
	#[serde(rename = "GridConnectionPointOfPremises")]
	GridConnectionPointOfPremises,
	#[serde(rename = "Household")]
	Household,
	#[serde(rename = "PVSystem")]
	PVSystem,
	#[serde(rename = "EV")]
	EV,
	#[serde(rename = "EVSE")]
	EVSE,
	#[serde(rename = "ChargingOutlet")]
	ChargingOutlet,
	#[serde(rename = "CEM")]
	CEM,
}

impl fmt::Display for EntityTypeEnumType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		utils::provide_enum_display(self, f)
	}
}

pub type FeatureTypeType = FeatureTypeEnumType;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum FeatureTypeEnumType {
	#[serde(rename = "ActuratorLevel")]
	ActuratorLevel,
	#[serde(rename = "ActuatorSwitch")]
	ActuatorSwitch,
	#[serde(rename = "Alarm")]
	Alarm,
	#[serde(rename = "DataTunneling")]
	DataTunneling,
	#[serde(rename = "DeviceClassification")]
	DeviceClassification,
	#[serde(rename = "DeviceDiagnosis")]
	DeviceDiagnosis,
	#[serde(rename = "DirectControl")]
	DirectControl,
	#[serde(rename = "ElectricalConnection")]
	ElectricalConnection,
	#[serde(rename = "Generic")]
	Generic,
	#[serde(rename = "Hvac")]
	Hvac,
	#[serde(rename = "LoadControl")]
	LoadControl,
	#[serde(rename = "Measurement")]
	Measurement,
	#[serde(rename = "Messaging")]
	Messaging,
	#[serde(rename = "NetworkManagement")]
	NetworkManagement,
	#[serde(rename = "NodeManagement")]
	NodeManagement,
	#[serde(rename = "OperatingConstraints")]
	OperatingConstraints,
	#[serde(rename = "PowerSequences")]
	PowerSequences,
	#[serde(rename = "Sensing")]
	Sensing,
	#[serde(rename = "Setpoint")]
	Setpoint,
	#[serde(rename = "SmartEnergyManagementPs")]
	SmartEnergyManagementPs,
	#[serde(rename = "TaskManagement")]
	TaskManagement,
	#[serde(rename = "Threshold")]
	Threshold,
	#[serde(rename = "TimeInformation")]
	TimeInformation,
	#[serde(rename = "TimeTable")]
	TimeTable,
	#[serde(rename = "DeviceConfiguration")]
	DeviceConfiguration,
	#[serde(rename = "SupplyCondition")]
	SupplyCondition,
	#[serde(rename = "TimeSeries")]
	TimeSeries,
	#[serde(rename = "TariffInformation")]
	TariffInformation,
	#[serde(rename = "IncentiveTable")]
	IncentiveTable,
	#[serde(rename = "Bill")]
	Bill,
	#[serde(rename = "Identification")]
	Identification,
}

impl fmt::Display for FeatureTypeEnumType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		utils::provide_enum_display(self, f)
	}
}

pub type FeatureSpecificUsageType = FeatureSpecificUsageEnumType;

pub type FeatureSpecificUsageEnumType = String;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum FeatureDirectControlSpecificUsageEnumType {
	#[serde(rename = "History")]
	History,
	#[serde(rename = "RealTime")]
	RealTime,
}

impl fmt::Display for FeatureDirectControlSpecificUsageEnumType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		utils::provide_enum_display(self, f)
	}
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum FeatureHvacSpecificUsageEnumType {
	#[serde(rename = "OperationMode")]
	OperationMode,
	#[serde(rename = "Overrun")]
	Overrun,
}

impl fmt::Display for FeatureHvacSpecificUsageEnumType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		utils::provide_enum_display(self, f)
	}
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum FeatureMeasurementSpecificUsageEnumType {
	#[serde(rename = "Contact")]
	Contact,
	#[serde(rename = "Electrical")]
	Electrical,
	#[serde(rename = "Heat")]
	Heat,
	#[serde(rename = "Level")]
	Level,
	#[serde(rename = "Pressure")]
	Pressure,
	#[serde(rename = "Temperature")]
	Temperature,
}

impl fmt::Display for FeatureMeasurementSpecificUsageEnumType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		utils::provide_enum_display(self, f)
	}
}

pub type FeatureSetpointSpecificUsageEnumType = FeatureMeasurementSpecificUsageEnumType;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum FeatureSmartEnergyManagementPsSpecificUsageEnumType {
	#[serde(rename = "FixedForecast")]
	FixedForecast,
	#[serde(rename = "FlexibleChosenForecast")]
	FlexibleChosenForecast,
	#[serde(rename = "TemperaFlexibleOptionalForecastture")]
	FlexibleOptionalForecast,
	#[serde(rename = "OptionalSequenceBasedImmediateControl")]
	OptionalSequenceBasedImmediateControl,
}

impl fmt::Display for FeatureSmartEnergyManagementPsSpecificUsageEnumType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		utils::provide_enum_display(self, f)
	}
}

pub type FunctionType = FunctionEnumType;

#[derive(Serialize, Deserialize, Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum FunctionEnumType {
	#[serde(rename = "actuatorLevelData")]
	ActuatorLevelData,
	#[serde(rename = "actuatorLevelDescriptionData")]
	ActuatorLevelDescriptionData,
	#[serde(rename = "actuatorSwitchData")]
	ActuatorSwitchData,
	#[serde(rename = "actuatorSwitchDescriptionData")]
	ActuatorSwitchDescriptionData,
	#[serde(rename = "alarmListData")]
	AlarmListData,
	#[serde(rename = "bindingManagementDeleteCall")]
	BindingManagementDeleteCall,
	#[serde(rename = "bindingManagementEntryListData")]
	BindingManagementEntryListData,
	#[serde(rename = "bindingManagementRequestCall")]
	BindingManagementRequestCall,
	#[serde(rename = "dataTunnelingCall")]
	DataTunnelingCall,
	#[serde(rename = "deviceClassificationManufacturerData")]
	DeviceClassificationManufacturerData,
	#[serde(rename = "deviceClassificationUserData")]
	DeviceClassificationUserData,
	#[serde(rename = "deviceDiagnosisHeartbeatData")]
	DeviceDiagnosisHeartbeatData,
	#[serde(rename = "deviceDiagnosisServiceData")]
	DeviceDiagnosisServiceData,
	#[serde(rename = "deviceDiagnosisStateData")]
	DeviceDiagnosisStateData,
	#[serde(rename = "directControlActivityListData")]
	DirectControlActivityListData,
	#[serde(rename = "directControlDescriptionData")]
	DirectControlDescriptionData,
	#[serde(rename = "electricalConnectionDescriptionListData")]
	ElectricalConnectionDescriptionListData,
	#[serde(rename = "electricalConnectionParameterDescriptionListData")]
	ElectricalConnectionParameterDescriptionListData,
	#[serde(rename = "electricalConnectionStateListData")]
	ElectricalConnectionStateListData,
	#[serde(rename = "hvacOperationModeDescriptionListData")]
	HvacOperationModeDescriptionListData,
	#[serde(rename = "hvacOverrunDescriptionListData")]
	HvacOverrunDescriptionListData,
	#[serde(rename = "hvacOverrunListData")]
	HvacOverrunListData,
	#[serde(rename = "hvacSystemFunctionDescriptionListData")]
	HvacSystemFunctionDescriptionListData,
	#[serde(rename = "hvacSystemFunctionListData")]
	HvacSystemFunctionListData,
	#[serde(rename = "hvacSystemFunctionOperationModeRelationListData")]
	HvacSystemFunctionOperationModeRelationListData,
	#[serde(rename = "hvacSystemFunctionPowerSequenceRelationListData")]
	HvacSystemFunctionPowerSequenceRelationListData,
	#[serde(rename = "hvacSystemFunctionSetpointRelationListData")]
	HvacSystemFunctionSetpointRelationListData,
	#[serde(rename = "loadControlEventListData")]
	LoadControlEventListData,
	#[serde(rename = "loadControlStateListData")]
	LoadControlStateListData,
	#[serde(rename = "measurementConstraintsListData")]
	MeasurementConstraintsListData,
	#[serde(rename = "measurementDescriptionListData")]
	MeasurementDescriptionListData,
	#[serde(rename = "measurementListData")]
	MeasurementListData,
	#[serde(rename = "measurementThresholdRelationListData")]
	MeasurementThresholdRelationListData,
	#[serde(rename = "messagingListData")]
	MessagingListData,
	#[serde(rename = "networkManagementAbortCall")]
	NetworkManagementAbortCall,
	#[serde(rename = "networkManagementAddNodeCall")]
	NetworkManagementAddNodeCall,
	#[serde(rename = "networkManagementDeviceDescriptionListData")]
	NetworkManagementDeviceDescriptionListData,
	#[serde(rename = "networkManagementDiscoverCall")]
	NetworkManagementDiscoverCall,
	#[serde(rename = "networkManagementEntityDescriptionListData")]
	NetworkManagementEntityDescriptionListData,
	#[serde(rename = "networkManagementFeatureDescriptionListData")]
	NetworkManagementFeatureDescriptionListData,
	#[serde(rename = "networkManagementJoiningModeData")]
	NetworkManagementJoiningModeData,
	#[serde(rename = "networkManagementModifyNodeCall")]
	NetworkManagementModifyNodeCall,
	#[serde(rename = "networkManagementProcessStateData")]
	NetworkManagementProcessStateData,
	#[serde(rename = "networkManagementRemoveNodeCall")]
	NetworkManagementRemoveNodeCall,
	#[serde(rename = "networkManagementReportCandidateData")]
	NetworkManagementReportCandidateData,
	#[serde(rename = "networkManagementScanNetworkCall")]
	NetworkManagementScanNetworkCall,
	#[serde(rename = "nodeManagementBindingData")]
	NodeManagementBindingData,
	#[serde(rename = "nodeManagementBindingDeleteCall")]
	NodeManagementBindingDeleteCall,
	#[serde(rename = "nodeManagementBindingRequestCall")]
	NodeManagementBindingRequestCall,
	#[serde(rename = "nodeManagementDestinationListData")]
	NodeManagementDestinationListData,
	#[serde(rename = "nodeManagementDetailedDiscoveryData")]
	NodeManagementDetailedDiscoveryData,
	#[serde(rename = "nodeManagementSubscriptionData")]
	NodeManagementSubscriptionData,
	#[serde(rename = "nodeManagementSubscriptionDeleteCall")]
	NodeManagementSubscriptionDeleteCall,
	#[serde(rename = "nodeManagementSubscriptionRequestCall")]
	NodeManagementSubscriptionRequestCall,
	#[serde(rename = "operatingConstraintsDurationListData")]
	OperatingConstraintsDurationListData,
	#[serde(rename = "operatingConstraintsInterruptListData")]
	OperatingConstraintsInterruptListData,
	#[serde(rename = "operatingConstraintsPowerDescriptionListData")]
	OperatingConstraintsPowerDescriptionListData,
	#[serde(rename = "operatingConstraintsPowerLevelListData")]
	OperatingConstraintsPowerLevelListData,
	#[serde(rename = "operatingConstraintsPowerRangeListData")]
	OperatingConstraintsPowerRangeListData,
	#[serde(rename = "operatingConstraintsResumeImplicationListData")]
	OperatingConstraintsResumeImplicationListData,
	#[serde(rename = "powerSequenceAlternativesRelationListData")]
	PowerSequenceAlternativesRelationListData,
	#[serde(rename = "powerSequenceDescriptionListData")]
	PowerSequenceDescriptionListData,
	#[serde(rename = "powerSequenceNodeScheduleInformationData")]
	PowerSequenceNodeScheduleInformationData,
	#[serde(rename = "powerSequencePriceCalculationRequestCall")]
	PowerSequencePriceCalculationRequestCall,
	#[serde(rename = "powerSequencePriceListData")]
	PowerSequencePriceListData,
	#[serde(rename = "powerSequenceScheduleConfigurationRequestCall")]
	PowerSequenceScheduleConfigurationRequestCall,
	#[serde(rename = "powerSequenceScheduleConstraintsListData")]
	PowerSequenceScheduleConstraintsListData,
	#[serde(rename = "powerSequenceScheduleListData")]
	PowerSequenceScheduleListData,
	#[serde(rename = "powerSequenceSchedulePreferenceListData")]
	PowerSequenceSchedulePreferenceListData,
	#[serde(rename = "powerSequenceStateListData")]
	PowerSequenceStateListData,
	#[serde(rename = "powerTimeSlotScheduleConstraintsListData")]
	PowerTimeSlotScheduleConstraintsListData,
	#[serde(rename = "powerTimeSlotScheduleListData")]
	PowerTimeSlotScheduleListData,
	#[serde(rename = "powerTimeSlotValueListData")]
	PowerTimeSlotValueListData,
	#[serde(rename = "resultData")]
	ResultData,
	#[serde(rename = "sensingDescriptionData")]
	SensingDescriptionData,
	#[serde(rename = "sensingListData")]
	SensingListData,
	#[serde(rename = "setpointConstraintsListData")]
	SetpointConstraintsListData,
	#[serde(rename = "setpointDescriptionListData")]
	SetpointDescriptionListData,
	#[serde(rename = "setpointListData")]
	SetpointListData,
	#[serde(rename = "smartEnergyManagementPsConfigurationRequestCall")]
	SmartEnergyManagementPsConfigurationRequestCall,
	#[serde(rename = "smartEnergyManagementPsData")]
	SmartEnergyManagementPsData,
	#[serde(rename = "smartEnergyManagementPsPriceCalculationRequestCall")]
	SmartEnergyManagementPsPriceCalculationRequestCall,
	#[serde(rename = "smartEnergyManagementPsPriceData")]
	SmartEnergyManagementPsPriceData,
	#[serde(rename = "specificationVersionListData")]
	SpecificationVersionListData,
	#[serde(rename = "subscriptionManagementDeleteCall")]
	SubscriptionManagementDeleteCall,
	#[serde(rename = "subscriptionManagementEntryListData")]
	SubscriptionManagementEntryListData,
	#[serde(rename = "subscriptionManagementRequestCall")]
	SubscriptionManagementRequestCall,
	#[serde(rename = "supplyConditionDescriptionListData")]
	SupplyConditionDescriptionListData,
	#[serde(rename = "supplyConditionListData")]
	SupplyConditionListData,
	#[serde(rename = "supplyConditionThresholdRelationListData")]
	SupplyConditionThresholdRelationListData,
	#[serde(rename = "taskManagementJobDescriptionListData")]
	TaskManagementJobDescriptionListData,
	#[serde(rename = "taskManagementJobListData")]
	TaskManagementJobListData,
	#[serde(rename = "taskManagementJobRelationListData")]
	TaskManagementJobRelationListData,
	#[serde(rename = "taskManagementOverviewData")]
	TaskManagementOverviewData,
	#[serde(rename = "thresholdConstraintsListData")]
	ThresholdConstraintsListData,
	#[serde(rename = "thresholdDescriptionListData")]
	ThresholdDescriptionListData,
	#[serde(rename = "thresholdListData")]
	ThresholdListData,
	#[serde(rename = "timeDistributorData")]
	TimeDistributorData,
	#[serde(rename = "timeDistributorEnquiryCall")]
	TimeDistributorEnquiryCall,
	#[serde(rename = "timeInformationData")]
	TimeInformationData,
	#[serde(rename = "timePrecisionData")]
	TimePrecisionData,
	#[serde(rename = "timeTableConstraintsListData")]
	TimeTableConstraintsListData,
	#[serde(rename = "timeTableDescriptionListData")]
	TimeTableDescriptionListData,
	#[serde(rename = "timeTableListData")]
	TimeTableListData,
	#[serde(rename = "deviceConfigurationKeyValueConstraintsListData")]
	DeviceConfigurationKeyValueConstraintsListData,
	#[serde(rename = "deviceConfigurationKeyValueListData")]
	DeviceConfigurationKeyValueListData,
	#[serde(rename = "deviceConfigurationKeyValueDescriptionListData")]
	DeviceConfigurationKeyValueDescriptionListData,
	#[serde(rename = "loadControlLimitConstraintsListData")]
	LoadControlLimitConstraintsListData,
	#[serde(rename = "loadControlLimitDescriptionListData")]
	LoadControlLimitDescriptionListData,
	#[serde(rename = "loadControlLimitListData")]
	LoadControlLimitListData,
	#[serde(rename = "loadControlNodeData")]
	LoadControlNodeData,
	#[serde(rename = "timeSeriesConstraintsListData")]
	TimeSeriesConstraintsListData,
	#[serde(rename = "timeSeriesDescriptionListData")]
	TimeSeriesDescriptionListData,
	#[serde(rename = "timeSeriesListData")]
	TimeSeriesListData,
	#[serde(rename = "tariffOverallConstraintsData")]
	TariffOverallConstraintsData,
	#[serde(rename = "tariffListData")]
	TariffListData,
	#[serde(rename = "tariffBoundaryRelationListData")]
	TariffBoundaryRelationListData,
	#[serde(rename = "tariffTierRelationListData")]
	TariffTierRelationListData,
	#[serde(rename = "tariffDescriptionListData")]
	TariffDescriptionListData,
	#[serde(rename = "tierBoundaryListData")]
	TierBoundaryListData,
	#[serde(rename = "tierBoundaryDescriptionListData")]
	TierBoundaryDescriptionListData,
	#[serde(rename = "commodityListData")]
	CommodityListData,
	#[serde(rename = "tierListData")]
	TierListData,
	#[serde(rename = "tierIncentiveRelationListData")]
	TierIncentiveRelationListData,
	#[serde(rename = "tierDescriptionListData")]
	TierDescriptionListData,
	#[serde(rename = "incentiveListData")]
	IncentiveListData,
	#[serde(rename = "incentiveDescriptionListData")]
	IncentiveDescriptionListData,
	#[serde(rename = "incentiveTableData")]
	IncentiveTableData,
	#[serde(rename = "incentiveTableDescriptionData")]
	IncentiveTableDescriptionData,
	#[serde(rename = "incentiveTableConstraintsData")]
	IncentiveTableConstraintsData,
	#[serde(rename = "electricalConnectionPermittedValueSetListData")]
	ElectricalConnectionPermittedValueSetListData,
	#[serde(rename = "useCaseInformationListData")]
	UseCaseInformationListData,
	#[serde(rename = "nodeManagementUseCaseData")]
	NodeManagementUseCaseData,
	#[serde(rename = "billConstraintsListData")]
	BillConstraintsListData,
	#[serde(rename = "billDescriptionListData")]
	BillDescriptionListData,
	#[serde(rename = "billListData")]
	BillListData,
	#[serde(rename = "identificationListData")]
	IdentificationListData,
}

impl fmt::Display for FunctionEnumType {
	fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
		utils::provide_enum_display(self, f)
	}
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct PossibleOperationsClassifierType {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub partial:      Option<ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct PossibleOperationsReadType {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub partial:      Option<ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct PossibleOperationsWriteType {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub partial:      Option<ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
pub struct PossibleOperationsType {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub read:      Option<PossibleOperationsReadType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub write: Option<PossibleOperationsWriteType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PossibleOperationsElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub read:      Option<ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub write: Option<ElementTagType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FunctionPropertyType {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub function:      Option<FunctionType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub possible_operations: Option<PossibleOperationsType>,
}

#[derive(Serialize, Deserialize, Default, Debug)]
#[serde(rename_all = "camelCase")]
pub struct FunctionPropertyElementsType {
	#[serde(skip_serializing_if = "Option::is_none")]
	pub function:      Option<ElementTagType>,
	#[serde(skip_serializing_if = "Option::is_none")]
	pub possible_operations: Option<PossibleOperationsElementsType>,
}

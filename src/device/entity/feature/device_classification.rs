#![allow(dead_code)]
use super::super::Entity;
use super::function_data::FunctionData;
use super::feature_local::FeatureLocal;
use crate::model::spine::commondatatypes::{FeatureTypeEnumType, RoleType, FunctionEnumType};
use crate::model::spine::deviceclassification::{DeviceClassificationManufacturerDataType, DeviceClassificationUserDataType};

pub struct DeviceClassificationLocal {
    feature: FeatureLocal,
}


impl DeviceClassificationLocal {
    pub fn new_device_classification_local(id: u32, entity: Entity, role: RoleType) -> DeviceClassificationLocal {
        let mut feature = FeatureLocal::new(id, entity, FeatureTypeEnumType::DeviceConfiguration, role);

        feature.base.add_function_data(FunctionData::new(
            FunctionEnumType::DeviceClassificationManufacturerData, 
            DeviceClassificationManufacturerDataType::default()));
        feature.base.add_function_data(FunctionData::new(
            FunctionEnumType::DeviceClassificationUserData, 
            DeviceClassificationUserDataType::default()));
    
        return DeviceClassificationLocal {feature};
    }

    pub fn get_manufacturer_data(&mut self) -> &DeviceClassificationManufacturerDataType {
        match self.feature.base.get_data::<DeviceClassificationManufacturerDataType>(&FunctionEnumType::DeviceClassificationManufacturerData) {
            Some(data) => data,
            None => panic!("function data not found")
        }
    }

    pub fn set_manufacturer_data(&mut self, data: DeviceClassificationManufacturerDataType) {
        self.feature.set_data(&FunctionEnumType::DeviceClassificationManufacturerData, data);
    }
}


#[cfg(test)]
mod tests {
    use crate::device::Device;
    use crate::model::spine::commondatatypes::{EntityTypeType};

    use super::*;

    #[test]
    fn manufacturer_data() {
        let mut sut = create_sut();
        let mut data = DeviceClassificationManufacturerDataType::default();
        data.brand_name = Some("my brand".to_string());

        // Act
        sut.set_manufacturer_data(data);
        let new_data = sut.get_manufacturer_data();
        match &new_data.brand_name {
            Some(name) => assert_eq!("my brand".to_string(), *name),
            None => assert!(false) 
        }
    }

    fn create_sut() -> DeviceClassificationLocal {
        let device = Device::new("address".to_string());
        let entity = Entity::new(vec![0], EntityTypeType::CEM, device);
        return DeviceClassificationLocal::new_device_classification_local(1, entity, RoleType::Server);
    }
}
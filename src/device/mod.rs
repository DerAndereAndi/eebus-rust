#![allow(dead_code)]
pub mod entity;
use entity::Entity;
use super::model::spine::commondatatypes::AddressDeviceType;

pub struct Device {
    address: AddressDeviceType,
    entities: Vec<Entity>,
}

impl Device {
    pub fn new(address: AddressDeviceType) -> Device {
        Device {
            address,
            entities: Vec::new(),
        }
    }

    pub fn add_entity(&mut self, entity: Entity) {
        self.entities.push(entity);
    }
}

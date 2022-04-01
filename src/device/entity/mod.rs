#![allow(dead_code)]
pub mod feature;

use super::Device;
use feature::feature_local::FeatureLocal;
use crate::model::spine::commondatatypes::{EntityTypeType, AddressEntityType};


pub struct Entity {
    address: Vec<AddressEntityType>,
    entity_type: EntityTypeType,
    device: Device,
    features: Vec<FeatureLocal>,
}

impl Entity {
    pub fn new(address: Vec<AddressEntityType>, entity_type: EntityTypeType, device: Device) -> Entity {
        Entity { address, entity_type, device,
            features: Vec::new(),
        }
    }
}



#![allow(dead_code)]
use std::any::Any;

use super::super::Entity;
use super::feature_base::FeatureBase;
use crate::model::spine::commondatatypes::{FeatureTypeEnumType, RoleType, FunctionEnumType};

pub struct FeatureRemote {
    pub base: FeatureBase,
}

impl FeatureRemote {
    pub fn new(id: u32, entity: Entity, feature_type: FeatureTypeEnumType, role: RoleType) -> FeatureRemote {
        FeatureRemote {
            base: FeatureBase::new(id, entity, feature_type, role),
        }
    }

    pub fn set_data<T: Any>(&mut self, function_type: &FunctionEnumType, data: T) {
        self.base.set_data(function_type, data);
        // TODO: publish data changed event
    }
}

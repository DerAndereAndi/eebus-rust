
use std::any::Any;
use super::super::Entity;
use super::feature_base::FeatureBase;

use crate::model::spine::commondatatypes::{FeatureTypeEnumType, RoleType, FunctionEnumType};

pub struct FeatureLocal {
    pub base: FeatureBase,
    // TODO: add here FeatureLocal specific fields like 'Sender'
}

impl FeatureLocal {
    pub fn new(id: u32, entity: Entity, feature_type: FeatureTypeEnumType, role: RoleType) -> FeatureLocal {
        FeatureLocal {
            base: FeatureBase::new(id, entity, feature_type, role),
        }
    }

    pub fn set_data<T: Any>(&mut self, function_type: &FunctionEnumType, data: T) {
        if let Some(function_data) = self.base.get_function_data(function_type)
        {
            function_data.set_data(data);
            let cmd = function_data.notify_cmd(false);
            println!("notify cmd: {:#?}", &cmd);
            // TODO: notify subscribers
        }
    }
}


#![allow(dead_code)]
use super::super::Entity;
use super::function_data::FunctionData;
use crate::model::spine::commondatatypes::{FeatureTypeEnumType, RoleType, FunctionEnumType};

use std::{collections::HashMap, any::Any};

pub struct FeatureBase {
    id: u32,
    entity: Entity,
    feature_type: FeatureTypeEnumType,
    role: RoleType,
    function_data: HashMap<FunctionEnumType, FunctionData>,
}

impl FeatureBase {
    pub fn new(id: u32, entity: Entity, feature_type: FeatureTypeEnumType, role: RoleType) -> FeatureBase {
        FeatureBase {
            id,
            entity,
            feature_type,
            role,
            function_data: HashMap::new(),
        }
    }

    pub fn add_function_data(&mut self, fd: FunctionData) {
        self.function_data.insert(fd.get_type(), fd);
    }

    pub fn get_function_data(&mut self, function_type: &FunctionEnumType) -> Option<&mut FunctionData> {
        self.function_data.get_mut(function_type)
    }

    pub fn get_data<T: Any>(&mut self, function_type: &FunctionEnumType) -> Option<&T> {
        match self.function_data.get_mut(function_type) {
            Some(function_data) => Option::Some(function_data.get_data()),
            None => None
        }
    }

    pub fn set_data<T: Any>(&mut self, function_type: &FunctionEnumType, data: T) {
        if let Some(function_data) = self.function_data.get_mut(function_type) {
            function_data.set_data(data);
        }
    }
}

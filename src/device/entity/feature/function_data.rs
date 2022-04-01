use std::any::Any;
use crate::model::spine::commondatatypes::{FunctionEnumType};
use crate::model::spine::commandframe::{CmdType};
use crate::model::spine::deviceclassification::DeviceClassificationManufacturerDataType;

#[derive(Debug)]
pub struct FunctionData {
    function_type: FunctionEnumType,
    data: Box<dyn Any>,
}

impl FunctionData {
    pub fn new<T: Any>(function_type: FunctionEnumType, data: T) -> FunctionData {
        FunctionData {
            function_type,
            data: Box::new(data),
        }
    }

    pub fn get_type(&self) -> FunctionEnumType {
        return self.function_type;
    }    

    pub fn get_data<T: Any>(&self) -> &T {
        self.data.downcast_ref::<T>().expect("wrong type")
    }

    pub fn set_data<T: Any>(&mut self, data: T) -> &T {
        self.data = Box::new(data);
        // let data_any = &*self.data;
        // println!(
        //     "SetData: typeId:{:?} data:{:?}",
        //     data_any.type_id(),
        //     data_any
        // );
        return self.get_data::<T>();
    }

    pub fn notify_cmd(&self, _partial: bool) -> CmdType {
        let mut cmd = self.create_cmd(false);
        cmd.function = Some(self.function_type);
        // TODO:
        //cmd.filter = create_filter_type(partial);
        return cmd;
    }

    pub fn read_cmd(&self) -> CmdType {        
        return self.create_cmd(true);
    }

    pub fn reply_cmd(&self) -> CmdType {        
        return self.create_cmd(false);
    }

    fn create_cmd(&self, use_default: bool) -> CmdType {
        let mut cmd = CmdType::default();
        if self.function_type == FunctionEnumType::DeviceClassificationManufacturerData {
            cmd.device_classification_manufacturer_data = 
                Some(self.clone_data_or_default::<DeviceClassificationManufacturerDataType>(use_default));
        }
        return cmd;
    }

    fn clone_data_or_default<T: Any + Default + Clone>(&self, default: bool) -> T {
        if default {
            return T::default();
        }
        return self.get_data::<T>().clone();
    }

    // fn create_filter_type(partial: bool) -> Option<Vec<FilterType, Global>> {

    // }
 
}


use serde::{Serialize, Deserialize};

use crate::prelude::Value;
use crate::prelude::FeatureLists;
use crate::prelude::NameSuffix;

#[derive(Default, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct RowsFilterModule{
    #[serde(rename = "@name")]
    method: String,

    #[serde(rename = "@parameter_version")]
    parameter_version: u8,

    parameters: Vec<RowsFilterModuleParameters>
}

impl RowsFilterModule{
    pub fn new() -> Self{
        RowsFilterModule{
            method: "io.github.mzmine.modules.dataprocessing.filter_rowsfilter.RowsFilterModule".to_owned(),
            parameter_version: 2,
            parameters: Vec::new()
        }
    }

    pub fn get_method(&self) -> &str{
        &self.method
    }

    pub fn get_parameters_length(&self) -> usize{
        self.parameters.len()
    }

    pub fn add_parameter(&mut self, parameter:RowsFilterModuleParameters){
        self.parameters.push(parameter);
    }

    pub fn get_parameter(&self, target:&str) -> Option<&RowsFilterModuleParameters>{
        // TODO
        None
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum RowsFilterModuleParameters{
    FeatureLists(FeatureLists),
    NameSuffix(NameSuffix),
    
}

impl RowsFilterModuleParameters{
    pub fn get_value(&self) -> Option<Value>{
        // TODO
        None
    }

    pub fn set_value(&mut self, value:Value){
        // TODO
    }
}
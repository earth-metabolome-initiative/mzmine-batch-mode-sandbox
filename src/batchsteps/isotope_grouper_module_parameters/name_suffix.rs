use serde::{Serialize, Deserialize};

use crate::isotope_grouper_module_parameters::*;

#[derive(Default, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(default, rename_all = "lowercase")]
pub struct NameSuffix{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "$text")]
    value: String,
}

impl NameSuffix{
    pub fn new() -> Self{
        NameSuffix{
            name: "Name suffix".to_owned(),
            value: "deiso".to_owned(),
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn get_value(&self) -> &str{
        &self.value
    }
    
    pub fn set_value(&mut self, value: &str){
        self.value = value.to_owned();
    } 
}
use serde::{Serialize, Deserialize};

use crate::isotope_grouper_module_parameters::*;

#[derive(Default, Serialize, Deserialize)]
pub struct RetentionTimeTolerance{
    #[serde(rename="@name")]
    name: String,

    #[serde(rename="@unit")]
    unit: String,

    #[serde(rename="$text")]
    value: Option<f32>,
}

impl RetentionTimeTolerance{
    pub fn new() -> Self{
        RetentionTimeTolerance{
            name: "Retention time tolerance".to_owned(),
            unit: "MINUTES".to_owned(),
            value: None,
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn get_unit(&self) -> &str{
        &self.unit
    }

    pub fn set_unit(&mut self, unit:&str){
        self.unit = unit.to_owned();
    } 

    pub fn get_value(&self) -> &Option<f32>{
        &self.value
    }

    pub fn set_value(&mut self, value:Option<f32>){
        self.value = value;
    }
}
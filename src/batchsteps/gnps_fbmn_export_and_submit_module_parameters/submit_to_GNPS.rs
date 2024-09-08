use serde::{Serialize, Deserialize};

use crate::rows_filter_module_parameters::Parameter;

#[derive(Default, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "lowercase", rename = "parameter")]
pub struct SubmitToGNPS{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "@selected")]
    selected: bool,

    #[serde(rename = "parameter")]
    parameters: Vec<Parameter>
}

impl SubmitToGNPS{
    pub fn new() -> Self{
        SubmitToGNPS{
            name: "Submit to GNPS".to_owned(),
            selected: false,
            parameters: Vec::new()
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn is_selected(&self) -> &bool{
        &self.selected
    }

    pub fn select(&mut self){
        self.selected = true
    }

    pub fn deselect(&mut self){
        self.selected = false
    }

    pub fn add_parameter(&mut self, parameter:Parameter){
        self.parameters.push(parameter);
    }

    pub fn get_parameters_length(&self) -> usize{
        self.parameters.len()
    }

    // TO DO
    // pub fn get_parameters(&mut self, target:&str){}
}
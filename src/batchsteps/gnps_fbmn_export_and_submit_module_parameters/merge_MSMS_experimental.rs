use serde::{Serialize, Deserialize};

use crate::minimum_search_feature_resolver_module_parameters::Ms1Ms2PrecursorTolerance as ExpectedMassDeviation;
use crate::rows_filter_module_parameters::Parameter;

#[derive(Default, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "lowercase", rename = "parameter")]
pub struct MergeMSMSExperimental{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "@selected")]
    selected: bool,

    #[serde(rename = "parameter")]
    parameters: Vec<MergeMSMSExperimentalParameter>
}

impl MergeMSMSExperimental{
    pub fn new() -> Self{
        MergeMSMSExperimental{
            name: "Merge MS/MS (experimental)".to_owned(),
            selected: false,
            parameters: Vec::new()
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn set_name(&mut self, name:&str){
        self.name = name.to_owned();
    }

    pub fn is_selected(&self) -> &bool{
        &self.selected
    }

    pub fn select(&mut self){
        self.selected = true;
    } 

    pub fn deselect(&mut self){
        self.selected = false
    }

    pub fn add_parameter(&mut self, parameter:MergeMSMSExperimentalParameter){
        self.parameters.push(parameter);
    }

    pub fn get_parameter(&mut self, target:&str) -> Option<&MergeMSMSExperimentalParameter>{
        for parameter in &mut self.parameters{
            match parameter{
                MergeMSMSExperimentalParameter::Parameter(_) if target == "Parameter" => return Some(parameter),
                MergeMSMSExperimentalParameter::ExpectedMassDeviation(_) if target == "ExpectedMassDeviation" => return Some(parameter),
                _ => panic!("No matching parameter {} found", target)
            }
        }
        None
    }

    pub fn get_parameters_length(&self) -> usize{
        self.parameters.len()
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum MergeMSMSExperimentalParameter{
    Parameter(Parameter),
    ExpectedMassDeviation(ExpectedMassDeviation),
}

impl MergeMSMSExperimentalParameter{
    pub fn get_name(&self) -> &str{
        match self{
            MergeMSMSExperimentalParameter::Parameter(_f) => _f.get_name(),
            MergeMSMSExperimentalParameter::ExpectedMassDeviation(_f) => _f.get_name(),
        }
    }
}
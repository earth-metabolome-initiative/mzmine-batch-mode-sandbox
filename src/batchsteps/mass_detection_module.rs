use serde::{Serialize, Deserialize};
use crate::mass_detection_module_parameters::*;
use crate::prelude::Value;

#[derive(Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
pub struct MassDetectionModule {
    #[serde(rename = "@method")]
    method: String,

    #[serde(rename = "@parameter_version")]
    parameter_version: u8,

    parameters: Vec<Parameter>,
}

impl MassDetectionModule{
    pub fn new() -> Self{
        MassDetectionModule{
            method: "io.github.mzmine.modules.dataprocessing.featdet_massdetection.MassDetectionModule".to_owned(),
            parameter_version: 1,
            parameters: Vec::new(),
        }
    }

    pub fn get_method(&self) -> String{
        self.method.clone()
    }

    pub fn get_parameter_version(&self) -> u8{
        self.parameter_version
    }

    pub fn get_parameters_length(&self) -> usize{
        self.parameters.len()
    }

    pub fn add_parameter(&mut self, parameter: Parameter){
        self.parameters.push(parameter);
    }
    
    pub fn get_parameter(&mut self, target: &str) -> &mut Parameter {
        for param in &mut self.parameters {
            match param {
                Parameter::RawDataFiles(_) if target == "RawDataFiles" => return param,
                Parameter::ScanFilters(_) if target == "ScanFilters" => return param,
                Parameter::ScanTypes(_) if target == "ScanTypes" => return param,
                Parameter::MassDetector(_) if target == "MassDetector" => return param,
                Parameter::DenormalizeFragmentScanTraps(_) if target == "DenormalizeFragmentScanTraps" => return param,
                _ => continue,
            }
        }
        panic!("Parameter '{}' not found", target)
    }
    
}

#[derive(Serialize, Deserialize, PartialEq, Clone)]
#[serde(untagged)]
pub enum Parameter{
    RawDataFiles(RawDataFiles),
    ScanFilters(ScanFilters),
    ScanTypes(ScanTypes),
    MassDetector(MassDetector),
    DenormalizeFragmentScanTraps(DenormalizeFragmentScanTraps)
}

impl Parameter {
    pub fn get_name(&self) -> Value {
        match self {
            Parameter::MassDetector(f) => Value::Str(f.get_name()),

            _ => panic!("No matching parameter for get_name()"), // Return None for non-matching cases
        }
    }
}

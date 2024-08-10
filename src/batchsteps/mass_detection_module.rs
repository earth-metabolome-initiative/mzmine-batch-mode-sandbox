use serde::{Serialize, Deserialize};
use crate::prelude::*;

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

    pub fn get_parameter(&self, target: &str) -> Result<&Parameter, &'static str>{
        for param in &self.parameters {
            match param {
                Parameter::RawDataFiles(_) if target == "RawDataFiles" => return Ok(&param),
                Parameter::ScanFilters(_) if target == "ScanFilters" => return Ok(&param),
                Parameter::ScanTypes(_) if target == "ScanTypes" => return Ok(&param),
                Parameter::MassDetector(_) if target == "MassDetector" => return Ok(&param),
                Parameter::DenormalizeFragmentScanTraps(_) if target == "DenormalizeFragmentScanTraps" => return Ok(&param),
                _ => continue,
            }
        }
        Err("Parameter not found")
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
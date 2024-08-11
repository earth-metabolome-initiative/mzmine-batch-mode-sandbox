use serde::{Serialize, Deserialize};

use crate::prelude::*;

#[derive(Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
pub struct SmoothingModule{
    #[serde( rename = "@name")]
    method: String,

    #[serde ( rename = "@name")]
    parameter_version: u8,

    parameter: Vec<SmoothingModuleParameters>
}

impl SmoothingModule{
    pub fn new() -> Self{
        SmoothingModule{
            method: "io.github.mzmine.modules.dataprocessing.featdet_smoothing.SmoothingModule".to_owned(),
            parameter_version: 1,
            parameter: Vec::new(),
        }
    }

    pub fn get_method(&self) -> &str{
        &self.method
    }

    pub fn get_parameter_version(&self) -> &u8{
        &self.parameter_version
    }

    pub fn add_parameter(&mut self, parameter:SmoothingModuleParameters){
        self.parameter.push(parameter);
    }

    pub fn get_parameters_lenght(&self) -> usize{
        self.parameter.len()
    }

    pub fn get_parameter(&self, target: &str) -> Result<&SmoothingModuleParameters, &'static str>{
        for param in &self.parameter {
            match param {
                SmoothingModuleParameters::FeatureList(_) if target == "Feature list" => return Ok(&param),
                SmoothingModuleParameters::OriginalFeatureList(_) if target == "Original feature list" => return Ok(&param),
                SmoothingModuleParameters::SmoothingAlgorithm(_) if target == "Smooth algotithm" => return Ok(&param),
                SmoothingModuleParameters::Suffix(_) if target == "Suffix" => return Ok(&param),
                _ => continue,
            }
        }
        Err("Parameter not found")
    }
}

#[derive(Serialize, Deserialize, PartialEq)]
pub enum SmoothingModuleParameters{
    FeatureList(FeatureLists),
    SmoothingAlgorithm(SmoothingAlgorithm),
    OriginalFeatureList(OriginalFeatureList),
    Suffix(SmoothSuffix),
}

impl SmoothingModuleParameters{
    pub fn get_name(&self) -> &str {
        match self {
            SmoothingModuleParameters::FeatureList(f) => f.get_name(),
            SmoothingModuleParameters::OriginalFeatureList(f) => f.get_name(),
            SmoothingModuleParameters::SmoothingAlgorithm(f) => f.get_name(),
            SmoothingModuleParameters::Suffix(f) => f.get_name(),
        }
    }
}
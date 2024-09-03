use serde::{Serialize, Deserialize};

use crate::batchsteps::rows_filter_module_parameters::parameter::Parameter;
use crate::minimum_search_feature_resolver_module_parameters::FeatureLists;
use crate::minimum_search_feature_resolver_module_parameters::Ms1Ms2PrecursorTolerance as MzToleranceFeatureToScan;

#[derive(Default, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename = "batchstep")]
pub struct IsotopeFinderModule{
    #[serde(rename = "@method")]
    method: String,

    #[serde(rename = "@parameter_version")]
    parameter_version: u8,

    #[serde(rename = "parameter")]
    parameters: Vec<IsotopeFinderModuleParameters>
}

impl IsotopeFinderModule{
    pub fn new() -> Self{
        IsotopeFinderModule{
            method: "io.github.mzmine.modules.dataprocessing.filter_isotopefinder.IsotopeFinderModule".to_owned(),
            parameter_version: 1,
            parameters: Vec::new()
        }
    }

    pub fn get_method(&self) -> &str{
        &self.method
    }

    pub fn get_parameter_version(&self) -> &u8{
        &self.parameter_version
    }

    pub fn get_parameters_length(&self) -> usize{
        self.parameters.len()
    }

    pub fn add_parameter(&mut self, parameter: IsotopeFinderModuleParameters){
        self.parameters.push(parameter);
    }

    pub fn get_parameter(&mut self, target:&str) -> Option<&IsotopeFinderModuleParameters>{
        for parameter in &mut self.parameters{
            match parameter{
                IsotopeFinderModuleParameters::FeatureLists(_f) if target == "FeatureLists"=> return Some(parameter),
                IsotopeFinderModuleParameters::MzToleranceFeatureToScan(_f) if target == "MzToleranceFeatureToScan" => return Some(parameter),
                IsotopeFinderModuleParameters::Parameter(_f) if target == "Parameter"=> return Some(parameter),
                _ => panic!("No matching paramenter {}", target)
            }
        }
        None 
    }

}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum IsotopeFinderModuleParameters{
    FeatureLists(FeatureLists),
    MzToleranceFeatureToScan(MzToleranceFeatureToScan),
    Parameter(Parameter)
}

impl IsotopeFinderModuleParameters{
    pub fn get_name(&self) -> &str{
        match self{
            IsotopeFinderModuleParameters::FeatureLists(_f) => return _f.get_name(),
            IsotopeFinderModuleParameters::MzToleranceFeatureToScan(_f) => return _f.get_name(),
            IsotopeFinderModuleParameters::Parameter(_f) => return _f.get_name(),
            _ => panic!("No matching parameter found")
        }
    }
}
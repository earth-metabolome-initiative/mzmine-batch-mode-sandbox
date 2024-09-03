use serde::{Serialize, Deserialize};

use crate::rows_filter_module_parameters::*;

use crate::prelude::Value;
use crate::minimum_search_feature_resolver_module_parameters::FeatureLists;
use crate::isotope_grouper_module_parameters::NameSuffix;

#[derive(Default, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all ="lowercase", rename = "batchstep")]
pub struct RowsFilterModule{
    #[serde(rename = "@method")]
    method: String,

    #[serde(rename = "@parameter_version")]
    parameter_version: u8,

    #[serde(rename = "parameter")]
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

    pub fn get_parameter_version(&self) -> &u8{
        &self.parameter_version
    }

    pub fn get_parameters_length(&self) -> usize{
        self.parameters.len()
    }

    pub fn add_parameter(&mut self, parameter:RowsFilterModuleParameters){
        self.parameters.push(parameter);
    }

    // TODO : How to select the Parameters?
    // pub fn get_parameter(&mut self, target:&str) -> Option<&RowsFilterModuleParameters>{
    //     for parameter in &mut self.parameters{
    //         match parameter{

    //             RowsFilterModuleParameters::Parameter(_) if target == "Parameter" => return Some(parameter),
    //             _ => panic!("No matching parameter {}", target)
    //         }
    //     }
    //     None
    // }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum RowsFilterModuleParameters{
    FeatureLists(FeatureLists),
    NameSuffix(NameSuffix),
    MinimumAlignedFeaturesSamples(MinimumAlignedFeaturesSamples),
    MinimumFeaturesInAnIsotopePattern(MinimumFeaturesInAnIsotopePattern),
    Validate13CIsotopeRows(Validate13CIsotopeRows),
    KendrickMassDefect(KendrickMassDefect),
    ChromatographicFWHM(ChromatographicFWHM),
    FeaturesDurationRange(FeaturesDurationRange),
    Charge(Charge),
    // If only selected and or value is present, the object is defined like Parameter!
    Parameter(Parameter)
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
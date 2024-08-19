use serde::{Serialize, Deserialize};

use crate::xml_serialization::*;

use crate::isotope_grouper_module_parameters::*;

#[derive(Default, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct IsotopeGrouper{
    method: String,

    parameter_version: u8,

    parameter: Vec<IsotopeGrouperParameters>
}

impl IsotopeGrouper{
    pub fn new() -> Self{
        IsotopeGrouper{
            method: "io.github.mzmine.modules.dataprocessing.filter_isotopegrouper.IsotopeGrouperModule".to_owned(),
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

    pub fn get_parameters_length(&self) -> usize{
        self.parameter.len()
    }

    pub fn add_parameter(&mut self, parameter: IsotopeGrouperParameters){
        self.parameter.push(parameter)
    } 

    pub fn get_parameter(&mut self, target:&str) -> Option<&IsotopeGrouperParameters>{
        for parameter in &mut self.parameter{
            match parameter{
                IsotopeGrouperParameters::FeatureLists(_) if target == "" => return Some(parameter),

                _ => panic!("No target matching parameter")
            }
        }
        None
    }
}


#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(untagged)]
pub enum IsotopeGrouperParameters{
    FeatureLists(FeatureLists),
    NameSuffix(NameSuffix),
    MzToleranceIntraSample(MzToleranceIntraSample),
    MobilityTolerance(MobilityTolerance),
    MonotonicShape(MonotonicShape),
    MaximumCharge(MaximumCharge),
    RepresentativeIsotope(RepresentativeIsotope)
}

impl IsotopeGrouperParameters{
    pub fn get_value(&self){
        // TODO
    }

    pub fn set_value(&mut self){
        // TODO
    }
}
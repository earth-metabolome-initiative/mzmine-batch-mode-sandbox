use serde::{Serialize, Deserialize};

use crate::prelude::Value;

use crate::isotope_grouper_module_parameters::*;
use crate::minimum_search_feature_resolver_module_parameters::FeatureLists;
use crate::minimum_search_feature_resolver_module_parameters::Ms1Ms2PrecursorTolerance as MzToleranceIntraSample;

#[derive(Default, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(default, rename_all= "lowercase", rename = "batchstep")]
pub struct IsotopeGrouper{
    #[serde(rename = "@method")]
    method: String,

    #[serde(rename = "@parameter_version")]
    parameter_version: u8,

    #[serde(rename = "parameter")]
    parameters: Vec<IsotopeGrouperParameters>
}

impl IsotopeGrouper{
    pub fn new() -> Self{
        IsotopeGrouper{
            method: "io.github.mzmine.modules.dataprocessing.filter_isotopegrouper.IsotopeGrouperModule".to_owned(),
            parameter_version: 1,
            parameters: Vec::new(),
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

    pub fn add_parameter(&mut self, parameter: IsotopeGrouperParameters){
        self.parameters.push(parameter)
    } 

    pub fn get_parameter(&mut self, target: &str) -> &mut IsotopeGrouperParameters {
        for parameter in &mut self.parameters {
            match parameter {
                IsotopeGrouperParameters::FeatureLists(_) if target == "FeatureLists" => return parameter,
                IsotopeGrouperParameters::NameSuffix(_) if target == "NameSuffix" => return parameter,
                IsotopeGrouperParameters::MzToleranceIntraSample(_) if target == "MzToleranceIntraSample" => return parameter,
                IsotopeGrouperParameters::MobilityTolerance(_) if target == "MobilityTolerance" => return parameter,
                IsotopeGrouperParameters::MonotonicShape(_) if target == "MonotonicShape" => return parameter,
                IsotopeGrouperParameters::MaximumCharge(_) if target == "MaximumCharge" => return parameter,
                IsotopeGrouperParameters::RepresentativeIsotope(_) if target == "RepresentativeIsotope" => return parameter,
                IsotopeGrouperParameters::NeverRemoveFeatureWithMs2(_) if target == "NeverRemoveFeatureWithMs2" => return parameter,
                IsotopeGrouperParameters::OriginalFeatureList(_) if target == "OriginalFeatureList" => return parameter,
                IsotopeGrouperParameters::RetentionTimeTolerance(_) if target == "RetentionTimeTolerance" => return parameter,
                _ => continue,
            }
        }
        panic!("Parameter '{}' not found", target)
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
    RepresentativeIsotope(RepresentativeIsotope),
    OriginalFeatureList(OriginalFeatureList),
    NeverRemoveFeatureWithMs2(NeverRemoveFeatureWithMs2),
    RetentionTimeTolerance(RetentionTimeTolerance)
}

impl IsotopeGrouperParameters{

    pub fn get_name(&self) -> &str{
        match self{
            IsotopeGrouperParameters::FeatureLists(_f) => _f.get_name(),
            IsotopeGrouperParameters::NameSuffix(_f) => _f.get_name(),
            IsotopeGrouperParameters::MzToleranceIntraSample(_f) => _f.get_name(),
            IsotopeGrouperParameters::MobilityTolerance(_f) => _f.get_name(),
            IsotopeGrouperParameters::MonotonicShape(_f) => _f.get_name(),
            IsotopeGrouperParameters::MaximumCharge(_f) => _f.get_name(),
            IsotopeGrouperParameters::RepresentativeIsotope(_f) => _f.get_name(),
            IsotopeGrouperParameters::OriginalFeatureList(_f) => _f.get_name(),
            IsotopeGrouperParameters::NeverRemoveFeatureWithMs2(_f) => _f.get_name(),
            IsotopeGrouperParameters::RetentionTimeTolerance(_f) => _f.get_name(),
            _ => panic!("No {} matching", self.get_name())
        }
    }

    pub fn get_value(&self) -> Value{
        match self{
            // IsotopeGrouperParameters::FeatureLists(_f) => _f.get(writer)?,
            IsotopeGrouperParameters::NameSuffix(_f) => Value::Str(_f.get_value()),
            //IsotopeGrouperParameters::MzToleranceIntraSample(_f) => Value::Float(_f.get_value()),
            IsotopeGrouperParameters::MobilityTolerance(_f) => Value::Float(_f.get_value()),
            IsotopeGrouperParameters::MonotonicShape(_f) => Value::Bool(_f.get_value()),
            IsotopeGrouperParameters::MaximumCharge(_f) => Value::Unsigned8(_f.get_value()),
            IsotopeGrouperParameters::RepresentativeIsotope(_f) => Value::Str(_f.get_value()),
            IsotopeGrouperParameters::OriginalFeatureList(_f) => Value::Str(_f.get_value()),
            IsotopeGrouperParameters::NeverRemoveFeatureWithMs2(_f) => Value::Bool(_f.get_value()),
            IsotopeGrouperParameters::RetentionTimeTolerance(_f) => Value::Float(_f.get_value()),

            _ => panic!("No matching parameter")
        }
    }

    pub fn get_type(&self) -> Value{
        match self {
            IsotopeGrouperParameters::FeatureLists(_f) => Value::Str(_f.get_type()),
            _ => panic!("Not Feature list parameter")
        }
    }

    pub fn get_unit(&self) -> Value{
        match self {
            IsotopeGrouperParameters::RetentionTimeTolerance(_f) => Value::Str(_f.get_unit()),
            _ => panic!("Not Retention time tolerance parameter")
        }
    }

    pub fn get_selected(&self) -> Value{
        match self {
            IsotopeGrouperParameters::MobilityTolerance(_f) => Value::Bool(_f.is_selected()),
            _ => panic!("Not Mobility tolerance parameter")
        }
    }

    // TODO: implement method to set/get ppm/absolute tolerance di MzTolerance

    pub fn set_value(&mut self, value: Value){
        match self{
            IsotopeGrouperParameters::NameSuffix(_f) => _f.set_value(value.get_str_value()),
            //IsotopeGrouperParameters::MzToleranceIntraSample(_f) => _f.set_value(*value.get_float_value()),
            IsotopeGrouperParameters::MobilityTolerance(_f) => _f.set_value(*value.get_float_value()),
            IsotopeGrouperParameters::MonotonicShape(_f) => _f.set_value(*value.get_bool_value()),
            IsotopeGrouperParameters::MaximumCharge(_f) => _f.set_value(*value.get_u8_value()),
            IsotopeGrouperParameters::RepresentativeIsotope(_f) => _f.set_value(value.get_str_value()),
            IsotopeGrouperParameters::OriginalFeatureList(_f) => _f.set_value(value.get_str_value()),
            IsotopeGrouperParameters::NeverRemoveFeatureWithMs2(_f) => _f.set_value(*value.get_bool_value()),
            IsotopeGrouperParameters::RetentionTimeTolerance(_f) => _f.set_value(*value.get_float_value()),

            _ => panic!("No matching parameter")
        }
    }
}
use core::panic;

use serde::{Serialize, Deserialize};
use crate::prelude::*;

#[derive(Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
pub struct MinimumSearchFeatureResolverModule{
    #[serde(rename="@name")]
    method: String,
    
    #[serde(rename="@name")]
    parameter_version: u8,

    parameters: Vec<MinimumSearchFeatureResolverModuleParameters>
}

impl MinimumSearchFeatureResolverModule{
    pub fn new(&self) -> &str{
        &self.method
    }

    pub fn get_parameter_version(&self) -> &u8{
        &self.parameter_version
    }

    pub fn get_parameters_length(&self) -> usize{
        self.parameters.len()
    }

    pub fn add_parameter(&mut self, parameter:MinimumSearchFeatureResolverModuleParameters){
        self.parameters.push(parameter);
    }

    pub fn get_parameter(&mut self, target:&str) -> Option<&MinimumSearchFeatureResolverModuleParameters>{
        for parameter in &mut self.parameters{
            match parameter{
                MinimumSearchFeatureResolverModuleParameters::ChromaticThreshold(_) if target == "Chromatic threshold" => return Some(parameter),
                MinimumSearchFeatureResolverModuleParameters::Dimension(_) if target == "Dimension" => return Some(parameter),
                _ => panic!("No matching parameter!")
            }
        }
        None
    }
}

#[derive(Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MinimumSearchFeatureResolverModuleParameters{
    ChromaticThreshold(ChromatographicThreshold),
    Dimension(Dimension),
    FeatureList(FeatureLists),
    LimitByIonMobilityEdges(LimitByIonMobilityEdges),
    MinRatioOfPeakTopEdge(MinRatioOfPeakTopEdge),
    MinimumAbsoluteHeight(ResolverMinimumAbsoluteHeight),
    MinimumRelativeHeight(MinimumRelativeHeight),
    MinimumScansDataPoints(MinimumScansDataPoints),
    MinimumSearchRangeRTMobilityAbsolute(MinimumSearchRangeRTMobilityAbsolute),
    MsMsScanPairing(MsMsScanPairing),
    OriginalFeatureList(OriginalFeatureList),
    RetentionTimeFilter(RetentionTimeFilter),
    Suffix(Suffix)
}

impl MinimumSearchFeatureResolverModuleParameters {
    /// gets parameter value, it can be of type &Option<f32> (ChromaticThreshold) or &str (Dimension)
    pub fn get_value(&self) -> Value {
        match self {
            MinimumSearchFeatureResolverModuleParameters::ChromaticThreshold(_f) => Value::Float(_f.get_value()), 
            MinimumSearchFeatureResolverModuleParameters::Dimension(_f) => Value::Str(_f.get_value()),
            MinimumSearchFeatureResolverModuleParameters::LimitByIonMobilityEdges(_f) => Value::Bool(_f.get_value()),
            MinimumSearchFeatureResolverModuleParameters::MinRatioOfPeakTopEdge(_f) => Value::Float(_f.get_value()),
            MinimumSearchFeatureResolverModuleParameters::MinimumAbsoluteHeight(_f) => Value::Float(_f.get_value()),
            MinimumSearchFeatureResolverModuleParameters::MinimumRelativeHeight(_f) => Value::Float(_f.get_value()),
            MinimumSearchFeatureResolverModuleParameters::MinimumScansDataPoints(_f) => Value::Unsigned8(_f.get_value()),
            _ => panic!("No matching parameter found")
        }
    }

    pub fn get_type(&self) -> &str{
        match self{
            MinimumSearchFeatureResolverModuleParameters::FeatureList(_f) => _f.get_type(),
            _=> panic!("No matching parameter found")
        }
    }

    pub fn set_value(&mut self, value:Value) {
        match self{
            MinimumSearchFeatureResolverModuleParameters::ChromaticThreshold(_f) => _f.set_value(*value.get_float_value()),
            MinimumSearchFeatureResolverModuleParameters::FeatureList(_f) => _f.set_value(*value.get_float_value()),
            MinimumSearchFeatureResolverModuleParameters::Dimension(_f) => _f.set_value(value.get_str_value()),
            MinimumSearchFeatureResolverModuleParameters::LimitByIonMobilityEdges(_f) => _f.set_value(*value.get_bool_value()),
            MinimumSearchFeatureResolverModuleParameters::MinRatioOfPeakTopEdge(_f) => _f.set_value(*value.get_float_value()),
            MinimumSearchFeatureResolverModuleParameters::MinimumAbsoluteHeight(_f) => _f.set_value(*value.get_float_value()),
            MinimumSearchFeatureResolverModuleParameters::MinimumRelativeHeight(_f) => _f.set_value(*value.get_float_value()),
            MinimumSearchFeatureResolverModuleParameters::MinimumScansDataPoints(_f) => _f.set_value(*value.get_u8_value()),
            _ => panic!("No matching parameter found"),
        }
    }

    pub fn set_type(&mut self, _type:&str){
        match self{
            MinimumSearchFeatureResolverModuleParameters::FeatureList(_f) => _f.set_type(_type),
            _ => panic!("No matching parameter found")
        }
    }
}

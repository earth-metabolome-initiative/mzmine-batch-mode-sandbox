use serde::{Serialize, Deserialize};

use crate::minimum_search_feature_resolver_module_parameters::*;
use crate::batchsteps::return_types::*;

#[derive(Default, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(default, rename_all = "lowercase", rename = "batchstep")]
pub struct MinimumSearchFeatureResolverModule{
    #[serde(rename="@method")]
    method: String,
    
    #[serde(rename="@parameter_version")]
    parameter_version: u8,

    #[serde(rename="parameter")]
    parameters: Vec<MinimumSearchFeatureResolverModuleParameters>
}

impl MinimumSearchFeatureResolverModule{
    pub fn new() -> Self{
        MinimumSearchFeatureResolverModule{
            method: "io.github.mzmine.modules.dataprocessing.featdet_chromatogramdeconvolution.minimumsearch.MinimumSearchFeatureResolverModule".to_owned(),
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

    pub fn add_parameter(&mut self, parameter:MinimumSearchFeatureResolverModuleParameters){
        self.parameters.push(parameter);
    }

    pub fn get_parameter(&mut self, target:&str) -> Option<&MinimumSearchFeatureResolverModuleParameters>{
        for parameter in &mut self.parameters{
            match parameter{
                MinimumSearchFeatureResolverModuleParameters::ChromaticThreshold(_) if target == "Chromatic threshold" => return Some(parameter),
                MinimumSearchFeatureResolverModuleParameters::Dimension(_) if target == "Dimension" => return Some(parameter),
                MinimumSearchFeatureResolverModuleParameters::FeatureList(_) if target == "FeatureList" => return Some(parameter),
                MinimumSearchFeatureResolverModuleParameters::LimitByIonMobilityEdges(_) if target == "LimitByIonMobilityEdges" => return Some(parameter),
                MinimumSearchFeatureResolverModuleParameters::MinRatioOfPeakTopEdge(_) if target == "MinRatioOfPeakTopEdge" => return Some(parameter),
                MinimumSearchFeatureResolverModuleParameters::MinimumAbsoluteHeight(_) if target == "MinimumAbsoluteHeight" => return Some(parameter),
                MinimumSearchFeatureResolverModuleParameters::MinimumRelativeHeight(_) if target == "MinimumRelativeHeight" => return Some(parameter),
                MinimumSearchFeatureResolverModuleParameters::MinimumScansDataPoints(_) if target == "MinimumScansDataPoints" => return Some(parameter),
                MinimumSearchFeatureResolverModuleParameters::MinimumSearchRangeRTMobilityAbsolute(_) if target == "MinimumSearchRangeRTMobilityAbsolute" => return Some(parameter),
                MinimumSearchFeatureResolverModuleParameters::MsMsScanPairing(_) if target == "MsMsScanPairing" => return Some(parameter),
                MinimumSearchFeatureResolverModuleParameters::OriginalFeatureList(_) if target == "OriginalFeatureList" => return Some(parameter),
                MinimumSearchFeatureResolverModuleParameters::RetentionTimeFilter(_) if target == "RetentionTimeFilter" => return Some(parameter),
                MinimumSearchFeatureResolverModuleParameters::Suffix(_) if target == "Suffix" => return Some(parameter),
                _ => panic!("No matching parameter!")
            }
        }
        None
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum MinimumSearchFeatureResolverModuleParameters{
    ChromaticThreshold(ChromatographicThreshold),
    Dimension(Dimension),
    FeatureList(FeatureLists),
    LimitByIonMobilityEdges(LimitByIonMobilityEdges),
    MinRatioOfPeakTopEdge(MinRatioOfPeakTopEdge),
    MinimumAbsoluteHeight(MinimumAbsoluteHeight),
    MinimumRelativeHeight(MinimumRelativeHeight),
    MinimumScansDataPoints(MinimumScansDataPoints),
    MinimumSearchRangeRTMobilityAbsolute(MinimumSearchRangeRTMobilityAbsolute),
    MsMsScanPairing(MsMsScanPairing),
    OriginalFeatureList(OriginalFeatureList),
    RetentionTimeFilter(RetentionTimeFilter),
    Suffix(Suffix),
    PeakDuration(PeakDuration)
}

impl MinimumSearchFeatureResolverModuleParameters {
    pub fn get_name(&self) -> &str{
        match self{
            MinimumSearchFeatureResolverModuleParameters::ChromaticThreshold(_f) => _f.get_name(),
            MinimumSearchFeatureResolverModuleParameters::Dimension(_f) => _f.get_name(),
            MinimumSearchFeatureResolverModuleParameters::FeatureList(_f) => _f.get_name(),
            MinimumSearchFeatureResolverModuleParameters::FeatureList(_f) => _f.get_name(),
            MinimumSearchFeatureResolverModuleParameters::LimitByIonMobilityEdges(_f) => _f.get_name(),
            MinimumSearchFeatureResolverModuleParameters::MinRatioOfPeakTopEdge(_f) => _f.get_name(),
            MinimumSearchFeatureResolverModuleParameters::MinimumAbsoluteHeight(_f) => _f.get_name(),
            MinimumSearchFeatureResolverModuleParameters::MinimumRelativeHeight(_f) => _f.get_name(),
            MinimumSearchFeatureResolverModuleParameters::MinimumScansDataPoints(_f) => _f.get_name(),
            MinimumSearchFeatureResolverModuleParameters::MinimumSearchRangeRTMobilityAbsolute(_f) => _f.get_name(),
            MinimumSearchFeatureResolverModuleParameters::MsMsScanPairing(_f) => _f.get_name(),
            MinimumSearchFeatureResolverModuleParameters::OriginalFeatureList(_f) => _f.get_name(),
            MinimumSearchFeatureResolverModuleParameters::RetentionTimeFilter(_f) => _f.get_name(),
            MinimumSearchFeatureResolverModuleParameters::Suffix(_f) => _f.get_name(),
            _ => panic!("No matching parameter found")
        }
    }

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
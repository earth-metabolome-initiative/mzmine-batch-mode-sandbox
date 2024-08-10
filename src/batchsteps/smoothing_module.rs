use serde::{Serialize, Deserialize};

use super::smoothing_module_parameters::feature_lists::FeatureLists;
use super::smoothing_module_parameters::original_feature_list::OriginalFeatureList;

#[derive(Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
pub struct SmoothingModule{
    #[serde( rename = "@name")]
    method: bool,

    #[serde ( rename = "@name")]
    parameter_version: u8,

    parameter: Vec<SmoothingModuleParameters>

}

#[derive(Serialize, Deserialize, PartialEq)]
enum SmoothingModuleParameters{
    FeatureList(FeatureLists),
    OriginalFeatureList(OriginalFeatureList),

}
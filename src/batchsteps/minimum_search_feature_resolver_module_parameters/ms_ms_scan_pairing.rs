use serde::{Serialize, Deserialize};

use crate::minimum_search_feature_resolver_module_parameters::*;


#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(default, rename_all = "lowercase", rename = "parameter")]
pub struct MsMsScanPairing{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "@selected")]
    selected: bool,

    #[serde(rename = "parameter")]
    parameters: Vec<MsMsScanPairingParameters>
}

impl MsMsScanPairing{
    pub fn new() -> Self{
        MsMsScanPairing{
            name: "MS/MS scan pairing".to_owned(),
            selected: true,
            parameters: Vec::new(),
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn is_selected(&self) -> &bool{
        &self.selected
    }

    pub fn select(&mut self){
        self.selected = true;
    }

    pub fn deselect(&mut self){
        self.selected = false;
    }

    pub fn get_parameters_length(&self) -> usize{
        self.parameters.len()
    }

    pub fn add_parameter(&mut self, parameter:MsMsScanPairingParameters){
        self.parameters.push(parameter);
    }

    pub fn get_parameter(&self, target: &str) -> Option<&MsMsScanPairingParameters> {
        for parameter in &self.parameters {
            match parameter {
                MsMsScanPairingParameters::MergeMsMsSpectraTIMS(_) if target == "MergeMsMsSpectraTIMS" => return Some(parameter),
                MsMsScanPairingParameters::MinimumRelativeFeatureHeight(_) if target == "MinimumRelativeFeatureHeight" => return Some(parameter),
                MsMsScanPairingParameters::MinimumRequiredSignals(_) if target == "MinimumRequiredSignals" => return Some(parameter),
                MsMsScanPairingParameters::MinimumSignalIntensityAbsoluteTIMS(_) if target == "MinimumSignalIntensityAbsoluteTIMS" => return Some(parameter),
                MsMsScanPairingParameters::MinimumSignalIntensityRelativeTIMS(_) if target == "MinimumSignalIntensityRelativeTIMS" => return Some(parameter),
                _ => continue,
            }
        }
        None
    }
    

}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(untagged)]
pub enum MsMsScanPairingParameters{
    Ms1ToMs2PrecursorTolerance(Ms1Ms2PrecursorTolerance),
    RetentionTimeFilter(RetentionTimeFilter),
    MinimumRelativeFeatureHeight(MinimumRelativeFeatureHeight),
    MinimumRequiredSignals(MinimumRequiredSignals),
    LimitByIonMobilityEdges(LimitByIonMobilityEdges),
    MergeMsMsSpectraTIMS(MergeMsMsSpectraTIMS),
    MinimumSignalIntensityAbsoluteTIMS(MinimumSignalIntensityAbsoluteTIMS),
    MinimumSignalIntensityRelativeTIMS(MinimumSignalIntensityRelativeTIMS),
}

impl MsMsScanPairingParameters{
    pub fn get_name(&self) -> &str{
        match self{
            MsMsScanPairingParameters::MergeMsMsSpectraTIMS(_f) => return _f.get_name(),
            MsMsScanPairingParameters::MinimumRelativeFeatureHeight(_f) => return _f.get_name(),
            MsMsScanPairingParameters::MinimumRequiredSignals(_f) => return _f.get_name(),
            MsMsScanPairingParameters::MinimumSignalIntensityAbsoluteTIMS(_f) => return _f.get_name(),
            MsMsScanPairingParameters::MinimumSignalIntensityRelativeTIMS(_f) => return _f.get_name(),
            _ => panic!("No matching parameter")
        }
    }
}
use serde::{Serialize, Deserialize};

use crate::batchsteps::minimum_search_feature_resolver_module_parameters::ms_ms_scan_pairing_parameters::merge_ms_ms_spectra_TIMS::MergeMsMsSpectraTIMS;
use crate::batchsteps::minimum_search_feature_resolver_module_parameters::ms_ms_scan_pairing_parameters::minimum_relative_feature_height::MinimumRelativeFeatureHeight;
use crate::batchsteps::minimum_search_feature_resolver_module_parameters::ms_ms_scan_pairing_parameters::minimum_required_signals::MinimumRequiredSignals;
use crate::batchsteps::minimum_search_feature_resolver_module_parameters::ms_ms_scan_pairing_parameters::minimum_signal_intensity_absolute_TIMS::MinimumSignalIntensityAbsoluteTIMS;
use crate::batchsteps::minimum_search_feature_resolver_module_parameters::ms_ms_scan_pairing_parameters::minimum_signal_intensity_relative_TIMS::MinimumSignalIntensityRelativeTIMS;


#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(default, rename_all = "lowercase")]
pub struct MsMsScanPairing{
    #[serde(rename = "@name")]
    name: String,

    parameter: Vec<MsMsScanPairingParameters>
}

impl MsMsScanPairing{
    pub fn new() -> Self{
        MsMsScanPairing{
            name: "MS1 to MS2 precursor tolerance (m/z)".to_owned(),

            parameter: Vec::new(),
        }
    }

}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(untagged)]
pub enum MsMsScanPairingParameters{
    MergeMsMsSpectraTIMS(MergeMsMsSpectraTIMS),
    MinimumRelativeFeatureHeight(MinimumRelativeFeatureHeight),
    MinimumRequiredSignals(MinimumRequiredSignals),
    MinimumSignalIntensityAbsoluteTIMS(MinimumSignalIntensityAbsoluteTIMS),
    MinimumSignalIntensityRelativeTIMS(MinimumSignalIntensityRelativeTIMS),
}
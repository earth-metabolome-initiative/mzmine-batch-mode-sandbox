use serde::{Serialize, Deserialize};

use crate::prelude::RawDataFiles;
use crate::prelude::ScanFilters;

use crate::prelude::AllowSingleScanChromatograms;
use crate::prelude::MinimumConsecutiveScans;
use crate::prelude::MinimumIntensityForConsecutiveScans;
use crate::prelude::MinimumAbsoluteHeight;
use crate::prelude::MzToleranceScanToScan;
use crate::prelude::Suffix;

#[derive(Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
pub struct ModularADAPChromatogramBuilderModule{
    #[serde(rename = "@method")]
    method: String,
    #[serde(rename = "@parameter_version")]
    parameter_version: u8,

    parameters: Vec<ModularADAPChromatogramBuilderModuleParameter>
}

impl ModularADAPChromatogramBuilderModule{
    pub fn new() -> Self{
        ModularADAPChromatogramBuilderModule{
            method: "io.github.mzmine.modules.dataprocessing.featdet_adapchromatogrambuilder.ModularADAPChromatogramBuilderModule".to_owned(),
            parameter_version: 1,
            parameters: Vec::new(),
        }
    }

    pub fn add_parameter(&mut self, parameter:ModularADAPChromatogramBuilderModuleParameter){
        self.parameters.push(parameter);
    }
}

#[derive(Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
enum ModularADAPChromatogramBuilderModuleParameter{
    RawDataFiles(RawDataFiles),
    ScanFilters(ScanFilters),
    MinimumConsecutiveScans(MinimumConsecutiveScans),
    MinimumIntensityForConsecutiveScans(MinimumIntensityForConsecutiveScans),
    MinimumAbsoluteHeight(MinimumAbsoluteHeight),
    MzToleranceScanToScan(MzToleranceScanToScan),
    Suffix(Suffix),
    AllowSingleScanChromatograms(AllowSingleScanChromatograms),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mADAP_chr_builder_module_initialization(){
        let obj = ModularADAPChromatogramBuilderModule::new();
        assert_eq!(obj.method, "io.github.mzmine.modules.dataprocessing.featdet_adapchromatogrambuilder.ModularADAPChromatogramBuilderModule");
        assert_eq!(obj.parameter_version, 1);
        assert_eq!(obj.parameters.len(), 0);
    }

    #[test]
    fn test_mADAP_chr_builder_module_add_parameter(){
        let mut obj = ModularADAPChromatogramBuilderModule::new();
        assert_eq!(obj.parameters.len(), 0);
        obj.add_parameter(ModularADAPChromatogramBuilderModuleParameter::RawDataFiles(RawDataFiles::new()));
        assert_eq!(obj.parameters.len(), 1);
    }
}
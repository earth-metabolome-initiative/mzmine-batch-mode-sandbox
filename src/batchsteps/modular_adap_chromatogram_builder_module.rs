use serde::{Serialize, Deserialize};

use crate::prelude::RawDataFiles;
use crate::prelude::ScanFilters;

use crate::prelude::AllowSingleScanChromatograms;
use crate::prelude::MinimumConsecutiveScans;
use crate::prelude::MinimumIntensityForConsecutiveScans;
use crate::prelude::MinimumAbsoluteHeight;
use crate::prelude::MzToleranceScanToScan;
use crate::prelude::ADAPSuffix;

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

    pub fn get_method(&self) -> &str{
        &self.method
    }

    pub fn get_parameter_version(&self) -> &u8{
        &self.parameter_version
    }

    pub fn get_parameters_length(&self) -> usize{
        self.parameters.len()
    }

    pub fn add_parameter(&mut self, parameter:ModularADAPChromatogramBuilderModuleParameter){
        self.parameters.push(parameter);
    }

    // pub fn get_parameter(&self, target:&str) -> &ModularADAPChromatogramBuilderModuleParameter{
    //     
    // }
}

#[derive(Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ModularADAPChromatogramBuilderModuleParameter{
    RawDataFiles(RawDataFiles),
    ScanFilters(ScanFilters),
    MinimumConsecutiveScans(MinimumConsecutiveScans),
    MinimumIntensityForConsecutiveScans(MinimumIntensityForConsecutiveScans),
    MinimumAbsoluteHeight(MinimumAbsoluteHeight),
    MzToleranceScanToScan(MzToleranceScanToScan),
    Suffix(ADAPSuffix),
    AllowSingleScanChromatograms(AllowSingleScanChromatograms),
}
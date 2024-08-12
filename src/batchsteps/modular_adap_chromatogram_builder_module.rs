use core::panic;

use serde::{Serialize, Deserialize};

use crate::prelude::Value;

use crate::prelude::RawDataFiles;
use crate::prelude::ScanFilters;

use crate::prelude::AllowSingleScanChromatograms;
use crate::prelude::MinimumConsecutiveScans;
use crate::prelude::MinimumIntensityForConsecutiveScans;
use crate::prelude::ADAPMinimumAbsoluteHeight;
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

    pub fn get_parameter(&mut self, target:&str) -> Option<&ModularADAPChromatogramBuilderModuleParameter>{
        for parameter in &mut self.parameters{
            match parameter{
                ModularADAPChromatogramBuilderModuleParameter::RawDataFiles(_) if target == "Raw data files" => return Some(parameter),
                ModularADAPChromatogramBuilderModuleParameter::ScanFilters(_) if target == "Scan filters" => return Some(parameter),
                ModularADAPChromatogramBuilderModuleParameter::MinimumConsecutiveScans(_) if target == "Minimum consecutive scans" => return Some(parameter),
                ModularADAPChromatogramBuilderModuleParameter::MinimumIntensityForConsecutiveScans(_) if target == "Minimum intensity for consecutive scans" => return Some(parameter),
                ModularADAPChromatogramBuilderModuleParameter::MinimumAbsoluteHeight(_) if target == "Minimum absolute height" => return Some(parameter),
                ModularADAPChromatogramBuilderModuleParameter::MzToleranceScanToScan(_) if target == "Mz tolerance scan to scan" => return Some(parameter),
                ModularADAPChromatogramBuilderModuleParameter::Suffix(_) if target == "Suffix" => return Some(parameter),
                ModularADAPChromatogramBuilderModuleParameter::AllowSingleScanChromatograms(_) if target == "Allow single scan chromatogram" => return Some(parameter),
                _ => panic!("No target matched")
            }
        }
        None
    }
}

#[derive(Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ModularADAPChromatogramBuilderModuleParameter{
    RawDataFiles(RawDataFiles),
    ScanFilters(ScanFilters),
    MinimumConsecutiveScans(MinimumConsecutiveScans),
    MinimumIntensityForConsecutiveScans(MinimumIntensityForConsecutiveScans),
    MinimumAbsoluteHeight(ADAPMinimumAbsoluteHeight),
    MzToleranceScanToScan(MzToleranceScanToScan),
    Suffix(ADAPSuffix),
    AllowSingleScanChromatograms(AllowSingleScanChromatograms),
}

impl ModularADAPChromatogramBuilderModuleParameter{
    pub fn get_type(&self) -> &str{
        match self { 
            ModularADAPChromatogramBuilderModuleParameter::RawDataFiles(_f) => _f.get_type(),
            _ => panic!("No parameter match")
        }
    }

    pub fn get_value(&self) -> Value{
        match self{
            ModularADAPChromatogramBuilderModuleParameter::MinimumConsecutiveScans(_f) => Value::Unsigned8(_f.get_value()),
            ModularADAPChromatogramBuilderModuleParameter::MinimumIntensityForConsecutiveScans(_f) => Value::Float(_f.get_value()),
            ModularADAPChromatogramBuilderModuleParameter::MinimumAbsoluteHeight(_f) => Value::Float(_f.get_value()),
            ModularADAPChromatogramBuilderModuleParameter::Suffix(_f) => Value::Str(_f.get_value()),
            ModularADAPChromatogramBuilderModuleParameter::AllowSingleScanChromatograms(_f) => Value::Float(_f.get_value()),
            _ => panic!("No parameter matching &Option<f32> as return type")
        }
    }

    pub fn set_value(&mut self, value:Value){
        match self{
            ModularADAPChromatogramBuilderModuleParameter::MinimumConsecutiveScans(_f) => _f.set_value(*value.get_u8_value()),
            ModularADAPChromatogramBuilderModuleParameter::MinimumIntensityForConsecutiveScans(_f) => _f.set_value(*value.get_float_value()),
            ModularADAPChromatogramBuilderModuleParameter::MinimumAbsoluteHeight(_f) => _f.set_value(*value.get_float_value()),
            ModularADAPChromatogramBuilderModuleParameter::Suffix(_f) => _f.set_value(value.get_str_value()),
            ModularADAPChromatogramBuilderModuleParameter::AllowSingleScanChromatograms(_f) => _f.set_value(*value.get_float_value()),
            _ => panic!("No parameter matching &Option<f32> as return type")
        }
    } 

    pub fn get_ppm_tolerance(&self) -> Value{
        match self{
            ModularADAPChromatogramBuilderModuleParameter::MzToleranceScanToScan(_f) => Value::Float(_f.get_ppm_tolerance()),
            _ => panic!("Parameter does not have get_ppm_tolerance() method"),
        }
    }

    pub fn set_ppm_tolerance(&mut self, value:Option<f32>){
        match self{
            ModularADAPChromatogramBuilderModuleParameter::MzToleranceScanToScan(_f) => _f.set_ppm_tolerance(value),
            _ => panic!("Parameter does not have get_ppm_tolerance() method"),
        }
    }

    pub fn set_absolute_tolerance(&mut self, value:Option<f32>){
        match self{
            ModularADAPChromatogramBuilderModuleParameter::MzToleranceScanToScan(_f) => _f.set_absolute_tolerance(value),
            _ => panic!("Parameter does not have get_ppm_tolerance() method"),
        }
    }

    pub fn get_absolute_tolerance(&self) -> Value{
        match self{
            ModularADAPChromatogramBuilderModuleParameter::MzToleranceScanToScan(_f) => Value::Float(_f.get_absolute_tolerance()),
            _ => panic!("Parameter does not have get_absolute_tolerance() method")
        }
    }
}
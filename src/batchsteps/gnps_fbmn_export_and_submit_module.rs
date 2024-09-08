use serde::{Serialize, Deserialize};

use crate::batchsteps::rows_filter_module_parameters::parameter::Parameter;
use crate::gnps_fbmn_export_and_submit_module_parameters::{MergeMSMSExperimental, SubmitToGNPS};
use crate::minimum_search_feature_resolver_module_parameters::FeatureLists;
use crate::all_spectral_data_import_module_parameters::{MetaData as FileName, MetaDataFile};

#[derive(Default, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "lowercase", rename = "batchstep")]
pub struct GnpsFbmnExportAndSubmitModule{
    #[serde(rename = "@method")]
    method: String,

    #[serde(rename = "@parameter_version")]
    parameter_version: u8,

    #[serde(rename = "parameter")]
    parameters: Vec<GnpsParameter>
}

impl GnpsFbmnExportAndSubmitModule{
    pub fn new() -> Self{
        GnpsFbmnExportAndSubmitModule{
            method: "io.github.mzmine.modules.io.export_features_gnps.fbmn.GnpsFbmnExportAndSubmitModule".to_owned(),
            parameter_version: 2,
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

    pub fn add_parameter(&mut self, parameter: GnpsParameter){
        self.parameters.push(parameter);
    }

    pub fn get_parameter(&mut self, target:&str) -> Option<&GnpsParameter>{
        for parameter in &mut self.parameters{
            match parameter{
                GnpsParameter::FeatureLists(_) if target == "FeatureLists" => return Some(parameter),
                GnpsParameter::Parameter(_) if target == "Parameter" => return Some(parameter),
                _ => panic!("No matching parameter {} found", target)
            }
        }
        None
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum GnpsParameter{
    FeatureLists(FeatureLists),
    Parameter(Parameter),
    FileName(FileName),
    MergeMSMSExperimental(MergeMSMSExperimental),
    SubmitToGNPS(SubmitToGNPS)
}

impl GnpsParameter{
    pub fn get_name(&self) -> &str{
        match self{
            GnpsParameter::FeatureLists(_f) => _f.get_name(),
            GnpsParameter::Parameter(_f) => _f.get_name(),
            GnpsParameter::FileName(_f) => _f.get_name(),
            GnpsParameter::MergeMSMSExperimental(_f) => _f.get_name(),
            GnpsParameter::SubmitToGNPS(_f) => _f.get_name(),
        }
    }

    pub fn get_current_file(&self) -> Option<&MetaDataFile>{
        match self{
            GnpsParameter::FileName(_f) => Some(_f.get_current_file()),
            GnpsParameter::FeatureLists(_) => panic!("Method not available for FeatureLists"),
            GnpsParameter::Parameter(_) => panic!("Method not available for Parameter"),
            GnpsParameter::MergeMSMSExperimental(_) => panic!("Method not available for MergeMSMSExperimental"),
            GnpsParameter::SubmitToGNPS(_) => panic!("Method not available for SubmitToGNPS"),
        }
    }

    pub fn set_current_file(&mut self, file: MetaDataFile){
        match self{
            GnpsParameter::FileName(_f) => _f.set_current_file(file),
            GnpsParameter::FeatureLists(_) => panic!("Method not available for FeatureLists"),
            GnpsParameter::Parameter(_) => panic!("Method not available for Parameter"),
            GnpsParameter::MergeMSMSExperimental(_) => panic!("Method not available for MergeMSMSExperimental"),
            GnpsParameter::SubmitToGNPS(_) => panic!("Method not available for SubmitToGNPS"),
        }
    }

    pub fn get_last_file(&self, file:&str) -> Option<&MetaDataFile>{
        match self{
            GnpsParameter::FileName(_f) => Some(_f.get_last_file(file)),
            GnpsParameter::FeatureLists(_) => panic!("Method not available for FeatureLists"),
            GnpsParameter::Parameter(_) => panic!("Method not available for Parameter"),
            GnpsParameter::MergeMSMSExperimental(_) => panic!("Method not available for MergeMSMSExperimental"),
            GnpsParameter::SubmitToGNPS(_) => panic!("Method not available for SubmitToGNPS"),
        }
    }

    pub fn set_last_file(&mut self, file: MetaDataFile){
        match self{
            GnpsParameter::FileName(_f) => _f.add_last_file_name(file),
            GnpsParameter::FeatureLists(_) => panic!("Method not available for FeatureLists"),
            GnpsParameter::Parameter(_) => panic!("Method not available for Parameter"),
            GnpsParameter::MergeMSMSExperimental(_) => panic!("Method not available for MergeMSMSExperimental"),
            GnpsParameter::SubmitToGNPS(_) => panic!("Method not available for SubmitToGNPS"),
        }
    }
}
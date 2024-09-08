use serde::{Serialize, Deserialize};
use crate::batchsteps::rows_filter_module_parameters::parameter::Parameter;
use crate::gnps_fbmn_export_and_submit_module_parameters::MergeMSMSExperimental as MergeMSMS;
use crate::minimum_search_feature_resolver_module_parameters::{FeatureLists, Ms1Ms2PrecursorTolerance as MZTolerance};
use crate::all_spectral_data_import_module_parameters::MetaData;


#[derive(Default, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "lowercase", rename = "batchstep")]
pub struct SiriusExportModule{
    #[serde(rename = "@method")]
    method: String,

    #[serde(rename = "@parameter_version")]
    parameter_version: u8,

    #[serde(rename = "parameter")]
    parameters: Vec<SiriusExportModuleParameter>
}

impl SiriusExportModule{
    pub fn new() -> Self{
        SiriusExportModule{
            method: "io.github.mzmine.modules.io.export_features_sirius.SiriusExportModule".to_owned(),
            parameter_version: 1u8,
            parameters: Vec::new()
        }
    }

    pub fn get_method(&self) -> &str{
        &self.method
    }

    pub fn get_parameter_version(&self) -> &u8{
        &self.parameter_version
    }

    pub fn add_parameter(&mut self, parameter:SiriusExportModuleParameter){
        self.parameters.push(parameter);
    }

    pub fn get_parameters_length(&self) -> usize{
        self.parameters.len()
    }

    // get_parameter?
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum SiriusExportModuleParameter{
    Parameter(Parameter),
    FeatureLists(FeatureLists),
    FileName(MetaData),
    MergeMSMS(MergeMSMS),
    MZTolerance(MZTolerance)
}
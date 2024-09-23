use std::collections::HashMap;

use serde::{Serialize, Deserialize};
use crate::prelude::*;

#[derive(Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase", rename = "batchstep")]
pub struct AllSpectralDataImportModule {
    #[serde(rename = "@method")]
    pub method: String,

    #[serde(rename = "@parameter_version")]
    pub parameter_version: u8,

    #[serde(rename = "parameter")]
    pub parameters: Vec<Parameter>,
}

impl AllSpectralDataImportModule{
    pub fn new() -> Self{
        AllSpectralDataImportModule{
            method: "io.github.mzmine.modules.io.import_rawdata_all.AllSpectralDataImportModule".to_owned(),
            parameter_version: 1,
            parameters: Vec::new(),
        }
    }

    /// to generate the batchstep we get the HashMap to know which parameter and associated values
    pub fn generate(parameters:&HashMap<String, Vec<String>>) -> Self{
        // store all generated parameters from string matching
        let mut generated_parameters = Vec::new();

        // iterate through the parameters and retrieve the needed key-value pairs
        for (key, _values) in parameters {
            // match the key and pass the values associated
            match key.as_str() {
                "FileNames" => {
                    generated_parameters.push(Parameter::FileNames(FileNames::generate(_values.to_owned())));
                }
                "AdvancedImport" => {
                    generated_parameters.push(Parameter::AdvancedImport(AdvancedImport::generate(_values.to_owned())));
                }
                "MetadataFile" => {
                    generated_parameters.push(Parameter::MetadataFile(MetaData::generate(_values.to_owned())));
                }
                "SpectralLibraryFiles" => {
                    generated_parameters.push(Parameter::SpectralLibraryFiles(SpectralLibrary::generate(_values.to_owned())));
                }
                _ => continue,
            }
        }

        AllSpectralDataImportModule{
            method: "io.github.mzmine.modules.io.import_rawdata_all.AllSpectralDataImportModule".to_owned(),
            parameter_version: 1,
            parameters: generated_parameters      
        }
    }

    pub fn add_parameter(&mut self, parameter:Parameter){
        self.parameters.push(parameter);
    }

    pub fn files(&self) -> &[Parameter]{
        &self.parameters
    }
}

#[derive(Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum Parameter {
    FileNames(FileNames),
    AdvancedImport(AdvancedImport),
    MetadataFile(MetaData),
    SpectralLibraryFiles(SpectralLibrary),
}

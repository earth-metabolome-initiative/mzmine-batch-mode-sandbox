use serde::{Serialize, Deserialize};
use crate::prelude::*;

#[derive(Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
pub struct AllSpectralDataImportModule {
    #[serde(rename = "@method")]
    pub method: String,

    #[serde(rename = "@parameter_version")]
    pub parameter_version: u8,

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

    pub fn add_parameter(&mut self, parameter:Parameter){
        self.parameters.push(parameter);
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

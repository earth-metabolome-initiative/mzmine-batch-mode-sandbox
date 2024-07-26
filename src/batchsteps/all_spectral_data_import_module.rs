use serde::{Serialize, Deserialize};
use crate::prelude::*;

#[derive(Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
pub struct AllSpectralDataImportModule {
    #[serde(rename = "@method")]
    method: String,

    #[serde(rename = "@parameter_version")]
    parameter_version: u8,

    parameters: Vec<Parameter>,
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
enum Parameter {
    FileNames(FileNames),
    AdvancedImport(AdvancedImport),
    MetadataFile(MetaData),
    SpectralLibraryFiles(SpectralLibrary),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_spectral_data_import_module_initialization(){
        let all_spectral_data_import_module_obj = AllSpectralDataImportModule::new();
        assert_eq!(all_spectral_data_import_module_obj.method, "io.github.mzmine.modules.io.import_rawdata_all.AllSpectralDataImportModule");
        assert_eq!(all_spectral_data_import_module_obj.parameter_version, 1);
        assert_eq!(all_spectral_data_import_module_obj.parameters.len(), 0);
    }

    #[test]
    fn test_all_spectral_data_import_module_add_parameter(){
        let mut all_spectral_data_import_module_obj = AllSpectralDataImportModule::new();
        assert_eq!(all_spectral_data_import_module_obj.parameters.len(), 0);
        all_spectral_data_import_module_obj.add_parameter(Parameter::FileNames(FileNames::new()));
        assert_eq!(all_spectral_data_import_module_obj.parameters.len(), 1);       
    }
}

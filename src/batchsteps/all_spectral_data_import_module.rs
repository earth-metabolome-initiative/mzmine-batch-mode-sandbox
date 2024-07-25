use serde::{Serialize, Deserialize};
use crate::prelude::*;

#[derive(Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
pub struct AllSpectralDataImportModule {
    #[serde(rename = "@method")]
    method: String,

    #[serde(rename = "@parameter_version")]
    parameter_version: u8,

    parameter: Vec<Parameter>,
}

impl Default for AllSpectralDataImportModule {
    fn default() -> Self {
        AllSpectralDataImportModule {
            method: "io.github.mzmine.modules.io.import_rawdata_all.AllSpectralDataImportModule".to_owned(),
            parameter_version: 1,
            parameter: vec![
                Parameter::FileNames(FileNames::default()),
                Parameter::AdvancedImport(AdvancedImport::default()),
            ],
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
enum Parameter {
    FileNames(FileNames),
    AdvancedImport(AdvancedImport),
    // MetadataFile(MetadataFile),
    // SpectralLibraryFiles(SpectralLibraryFiles),
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default(){
        let all_spectral_data_import_module = AllSpectralDataImportModule::default();
        assert_eq!(all_spectral_data_import_module.parameter.len(), 2)
    }
}

use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")] // rinominare oggetto come lo vogliamo stampato
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
            ],
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
enum Parameter {
    FileNames(FileNames),
}

#[derive(Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
struct FileNames {
    #[serde(rename = "@name")]
    name: String,
    //#[serde(rename = "@file")]
    file: Vec<File>,
}

impl Default for FileNames {
    fn default() -> Self {
        FileNames {
            name: "File names".to_owned(),
            file: vec![File::default(),File::default()],
        }
    }
}


#[derive(Serialize, Deserialize, PartialEq)]
#[serde(default, rename = "file", rename_all = "lowercase")]
struct File {
    #[serde(rename = "$text")]
    name: String
}

impl Default for File {
    fn default() -> Self {
        File {
            name: "File_name".to_owned(),
        }
    }
}

use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
pub struct SpectralLibrary {
    #[serde(rename = "@name")]
    name: String,

    files: Vec<SpectralLibraryFile>
}

impl SpectralLibrary{
    pub fn new() -> Self {
        SpectralLibrary {
            name: "Spectral library files".to_owned(),
            files: Vec::new(),
        }
    }

    pub fn get_name(&self) -> String{
        self.name.clone()
    }
 
    pub fn add_file(&mut self, element: SpectralLibraryFile){
        self.files.push(element);
    }

    pub fn get_files_length(&self) -> usize{
        self.files.len()
    }

    pub fn remove_file_name(&mut self, index: usize){
        self.files.remove(index);
    }

    pub fn get_file_by_name(&self, name:String) -> Result<&SpectralLibraryFile, &'static str>{
        for file in &self.files{
            if file.get_name() == name{
                return Ok(&file);
            }
        }
        Err("File not found")
    }
}

#[derive(Default, Serialize, Deserialize, PartialEq, Clone)]
#[serde(default, rename_all = "lowercase", rename = "file")]
pub struct SpectralLibraryFile{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "$text")]
    file_name: String,
}

impl SpectralLibraryFile{
    pub fn new() -> Self{
        SpectralLibraryFile{
            name: "Spectral library files".to_owned(),
            file_name: "File name".to_owned(),
        }
    }

    pub fn get_name(&self) -> String{
        self.name.clone()
    }

    pub fn get_file_name(&self) -> String{
        self.file_name.clone()
    }

    pub fn change_file_name(&mut self, name: String){
        self.file_name = name;
    }
}
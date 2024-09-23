use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase", rename = "parameter")]
pub struct SpectralLibrary {
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "file")]
    files: Vec<SpectralLibraryFile>
}

impl SpectralLibrary{
    pub fn new() -> Self {
        SpectralLibrary {
            name: "Spectral library files".to_owned(),
            files: Vec::new(),
        }
    }

    pub fn generate(files:Vec<String>) -> Self{
        let mut generated_files = Vec::new();
        for file in files{
            generated_files.push(SpectralLibraryFile::generate(file))
        }
        SpectralLibrary {
            name: "Spectral library files".to_owned(),
            files: generated_files,
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
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

    pub fn get_file_by_name(&self, name:&str) -> Result<&SpectralLibraryFile, &'static str>{
        for file in &self.files{
            if file.get_file_name() == name{
                return Ok(&file);
            }
        }
        Err("File not found")
    }
}

#[derive(Default, Serialize, Deserialize, PartialEq, Clone)]
#[serde(default, rename = "file", rename_all = "lowercase")]
pub struct SpectralLibraryFile{
    #[serde(rename = "$text")]
    file_name: String,
}

impl SpectralLibraryFile{
    pub fn new() -> Self{
        SpectralLibraryFile{
            file_name: "".to_string(),
        }
    }

    pub fn generate(file_name:String) -> Self{
        SpectralLibraryFile{
            file_name: file_name,
        }
    }

    pub fn get_file_name(&self) -> &str{
        &self.file_name
    }

    pub fn change_file_name(&mut self, name: String){
        self.file_name = name;
    }
}
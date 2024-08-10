use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
pub struct MetaData {
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "@selected")]
    selected: bool,

    #[serde(rename = "@current_file")]
    current_file: MetaDataFile,
    
    last_files: Vec<MetaDataFile>,
}

impl MetaData {
    pub fn new() -> Self {
        MetaData {
            name: "Metadata file".to_owned(),
            selected: true,
            current_file: MetaDataFile::new(),
            last_files: Vec::new(),
        }
    }

    pub fn get_name(&mut self) -> String{
        self.name.clone()
    }

    pub fn is_selected(&self) -> bool{
        self.selected
    }

    pub fn select(&mut self){
        self.selected = true;
    }

    pub fn deselect(&mut self){
        self.selected = false;
    }

    pub fn last_files_length(&self) -> usize{
        self.last_files.len()
    }

    pub fn get_current_file(&self) -> MetaDataFile{
        self.current_file.clone()
    }

    pub fn set_current_file(&mut self, file: MetaDataFile){
        self.current_file = file;
    }

    pub fn add_last_file_name(&mut self, element: MetaDataFile){
        self.last_files.push(element);
    }

    pub fn remove_last_file_name(&mut self, index: usize){
        self.last_files.remove(index);
    }

    pub fn get_last_file(&self, name: &str) -> Result<&MetaDataFile, &'static str>{
        for file in &self.last_files {
            if file.get_name() == name {
                return Ok(&file);
            }
        }
        Err("File not found")
    }
    


}

#[derive(Default, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(default, rename = "last_file", rename_all = "lowercase")]
pub struct MetaDataFile {
    #[serde(rename = "$text")]
    name: String
}

impl MetaDataFile {
    pub fn new() -> Self {
        MetaDataFile {
            name: "".to_owned(),
        }
    }

    pub fn get_name(&self) -> String{
        self.name.clone()
    }

    pub fn set_name(&mut self, file_name: String){
        self.name = file_name;
    }
}
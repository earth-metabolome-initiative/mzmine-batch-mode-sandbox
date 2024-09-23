use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(default, rename_all = "lowercase", rename = "parameter")]
pub struct MetaData {
    #[serde(rename = "@name")]
    name: String,

    #[serde(skip_serializing_if = "Option::is_none", rename = "@selected")]
    selected: Option<bool>,

    #[serde(rename = "current_file")]
    current_file: MetaDataFile,
    
    #[serde(rename = "last_file")]
    last_files: Vec<MetaDataFile>,
}

impl Default for MetaData{
    fn default() -> Self{
        MetaData {
            name: "Metadata file".to_owned(),
            selected: Some(true),
            current_file: MetaDataFile::default(),
            last_files: Vec::new(),
        }
    }
}

impl MetaData {
    pub fn new(current: &str, last: Vec<MetaDataFile>) -> Self {
        MetaData {
            name: "Metadata file".to_owned(),
            selected: Some(true),
            current_file: MetaDataFile::new(current),
            last_files: last,
        }
    }

    /// First element is the current file, every other is last_files
    pub fn generate(values: Vec<String>) -> Self {
        let current = &values[0];

        let mut last_files = Vec::new();

        // All but the first element
        for file in &values[1..] {
            last_files.push(MetaDataFile::new(file));
        }

        MetaData {
            name: "Metadata file".to_owned(),
            selected: Some(true),
            current_file: MetaDataFile::new(current),
            last_files: last_files,
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn set_name(&mut self, name:&str){
        self.name = name.to_owned();
    }

    pub fn is_selected(&self) -> &Option<bool>{
        &self.selected
    }

    pub fn select(&mut self){
        self.selected = Some(true);
    }

    pub fn deselect(&mut self){
        self.selected = Some(false);
    }

    pub fn set_selected_to_none(&mut self){
        self.selected = None;
    }

    pub fn get_last_files(&self) -> &Vec<MetaDataFile>{
        &self.last_files
    }

    pub fn last_files_length(&self) -> usize{
        self.last_files.len()
    }

    pub fn get_current_file(&self) -> &MetaDataFile{
        &self.current_file
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

    pub fn get_last_file(&self, name: &str) -> &MetaDataFile{
        for file in &self.last_files {
            if file.get_name() == name {
                return &file;
            }
        }
        panic!("No file found")
    }

}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(default, rename = "last_file", rename_all = "lowercase")]
pub struct MetaDataFile {
    #[serde(rename = "$text")]
    name: String
}

impl Default for MetaDataFile{
    fn default() -> Self{
        MetaDataFile {
            name: "".to_owned(),
        }
    }
}

impl MetaDataFile {
    pub fn new(name: &str) -> Self {
        MetaDataFile {
            name: name.to_owned(),
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn set_name(&mut self, file_name: &str){
        self.name = file_name.to_owned();
    }
}
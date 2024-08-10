use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
pub struct FileNames {
    #[serde(rename = "@name")]
    name: String,
    
    files: Vec<InputFile>,
}


impl FileNames{
    pub fn new() -> Self {
        FileNames {
            name: "File names".to_owned(),
            files: vec![],
        }
    }

    pub fn get_name(&self) -> String{
        self.name.clone()
    }

    pub fn get_file(&self, name: &str) -> Option<&InputFile> {
        self.files.iter().find(|file| file.get_name() == name)
    }
    

    pub fn files_length(&self) -> usize{
        self.files.len()
    }

    pub fn add_file_name(&mut self, element: InputFile){
        self.files.push(element);
    }

    pub fn remove_file_name(&mut self, name: &str) {
        self.files.retain(|file| file.get_name() != name);
    }
}

#[derive(Default, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(default, rename = "file", rename_all = "lowercase")]
pub struct InputFile {
    #[serde(rename = "$text")]
    name: String
}

impl InputFile{
    pub fn new() -> Self{
        InputFile{
            name: "".to_owned()
        }
    }

    pub fn get_name(&self) -> String{
        self.name.clone()
    }

    pub fn set_name(&mut self, file_name: String){
        self.name = file_name;
    }
}
use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[serde(default, rename_all = "lowercase", rename ="parameter")]
pub struct FileNames {
    #[serde(rename = "@name")]
    name: String,
    
    #[serde(rename = "file")]
    files: Vec<InputFile>,
}


impl FileNames{
    pub fn new() -> Self {
        FileNames {
            name: "File names".to_owned(),
            files: vec![],
        }
    }

    pub fn generate(files:Vec<String>) -> Self{
        let mut generated_files: Vec<InputFile> = Vec::new();

        for file in files{
            generated_files.push(InputFile::generate(file))
        }

        FileNames {
            name: "File names".to_owned(),
            files: generated_files,
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
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

    pub fn files(&self) -> &[InputFile]{
        &self.files
    }
}

#[derive(Default, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[cfg_attr(feature = "arbitrary", derive(arbitrary::Arbitrary))]
#[serde(default, rename = "file", rename_all = "lowercase")]
pub struct InputFile {
    #[serde(rename = "$text")]
    name: String
}

impl InputFile{
    pub fn new() -> Self{
        InputFile{
            name: "".to_string()
        }
    }

    pub fn generate(file_name:String) -> Self{
        InputFile{
            name: file_name
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn set_name(&mut self, file_name: &str){
        self.name = file_name.to_owned();
    }
}
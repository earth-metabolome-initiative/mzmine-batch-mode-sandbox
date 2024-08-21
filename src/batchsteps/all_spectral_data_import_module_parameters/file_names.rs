use serde::{Serialize, Deserialize};

use crate::xml_serialization::*;

#[derive(Default, Serialize, Deserialize, PartialEq)]
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
    
    pub fn write_element(&self, writer: &mut Writer<Cursor<Vec<u8>>>) -> IoResult<()>{
        let mut last_files = BytesStart::new("parameter");

        last_files.push_attribute(("name", self.get_name()));

        // Write the start tag
        writer.write_event(Event::Start(last_files))
            .map_err(|e| IoError::new(ErrorKind::Other, e.to_string()))?;

        for file in &self.files{
            let current_file = BytesStart::new("file");

            // Write the start tag
            writer.write_event(Event::Start(current_file))
                .map_err(|e| IoError::new(ErrorKind::Other, e.to_string()))?;

            writer.write_event(Event::Text(BytesText::new(file.get_name())))
                .map_err(|e| IoError::new(ErrorKind::Other, e.to_string()))?;

                // Write the end tag
            writer.write_event(Event::End(BytesEnd::new("file")))
                .map_err(|e| IoError::new(ErrorKind::Other, e.to_string()))?;
        }
        
        // Write the end tag
        writer.write_event(Event::End(BytesEnd::new("parameter")))
            .map_err(|e| IoError::new(ErrorKind::Other, e.to_string()))?;

        Ok(())
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

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn set_name(&mut self, file_name: &str){
        self.name = file_name.to_owned();
    }
}
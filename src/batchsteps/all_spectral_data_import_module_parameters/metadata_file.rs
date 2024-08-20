use serde::{Serialize, Deserialize};

use crate::xml_serialization::*;

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

    pub fn get_name(&self) -> &str{
        &self.name
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
    
    pub fn write_element(&self, writer: &mut Writer<Cursor<Vec<u8>>>) -> IoResult<()> {

        let mut metadata = BytesStart::new("parameter");

        metadata.push_attribute(("name", self.get_name()));
        metadata.push_attribute(("selected", self.is_selected().to_string().as_str()));

            // Write the start tag
        writer.write_event(Event::Start(metadata))
            .map_err(|e| IoError::new(ErrorKind::Other, e.to_string()))?;


            //          Writing last file vector

        //<parameter name="Monotonic shape">true</parameter>
        let current_file = BytesStart::new("current_file");

        //element.push_attribute(("name", self.get_name())); 

        // Write the start tag
        writer.write_event(Event::Start(current_file))
            .map_err(|e| IoError::new(ErrorKind::Other, e.to_string()))?;

        writer.write_event(Event::Text(BytesText::new(self.get_current_file().get_name())))
            .map_err(|e| IoError::new(ErrorKind::Other, e.to_string()))?;

        // Write the end tag
        writer.write_event(Event::End(BytesEnd::new("current_file")))
            .map_err(|e| IoError::new(ErrorKind::Other, e.to_string()))?;


            //          Writing last file vector

        for file in &self.last_files{
            let last_files = BytesStart::new("last_file");

            // Write the start tag
            writer.write_event(Event::Start(last_files))
                .map_err(|e| IoError::new(ErrorKind::Other, e.to_string()))?;

            writer.write_event(Event::Text(BytesText::new(file.get_name())))
                .map_err(|e| IoError::new(ErrorKind::Other, e.to_string()))?;

            // Write the end tag
            writer.write_event(Event::End(BytesEnd::new("last_file")))
                .map_err(|e| IoError::new(ErrorKind::Other, e.to_string()))?;
        }


        // Write the end tag
        writer.write_event(Event::End(BytesEnd::new("parameter")))
        .map_err(|e| IoError::new(ErrorKind::Other, e.to_string()))?;
        
        Ok(())
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

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn set_name(&mut self, file_name: &str){
        self.name = file_name.to_owned();
    }
}
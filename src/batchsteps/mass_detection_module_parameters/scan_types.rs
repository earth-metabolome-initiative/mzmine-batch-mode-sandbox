use serde::{Serialize, Deserialize};


use quick_xml::events::{Event, BytesEnd, BytesStart, BytesText};
use quick_xml::writer::Writer;
use std::io::{Cursor, Result as IoResult, Error as IoError, ErrorKind};

#[derive(Default, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(default, rename_all = "lowercase")]
pub struct ScanTypes {
    #[serde(rename = "@name")]
    name: String,
    
    #[serde(rename = "$text")]
    value: String,
}

impl ScanTypes {
    pub fn new() -> Self{
        ScanTypes { 
            name: "Scan types (IMS)".to_owned(), 
            value: "".to_owned(),
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn get_value(&self) -> &str{
        &self.value
    }

    pub fn set_value(&mut self, value:&str){
        self.value = value.to_owned();
    } 

    pub fn write_element(&self, writer: &mut Writer<Cursor<Vec<u8>>>) -> IoResult<()> {
        let mut element = BytesStart::new("parameter");
        
        // Set attributes with proper conversion from String to &str
        element.push_attribute(("name", self.name.as_str()));
        
        // Write the start tag
        writer.write_event(Event::Start(element))
            .map_err(|e| IoError::new(ErrorKind::Other, e.to_string()))?;
        
        // Write the text content
        writer.write_event(Event::Text(BytesText::new(&self.value)))
            .map_err(|e| IoError::new(ErrorKind::Other, e.to_string()))?;
        
        // Write the end tag
        writer.write_event(Event::End(BytesEnd::new("parameter")))
            .map_err(|e| IoError::new(ErrorKind::Other, e.to_string()))?;
        
        Ok(())
    }
}
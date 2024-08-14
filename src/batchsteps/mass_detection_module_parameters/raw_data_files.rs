use serde::{Serialize, Deserialize};

use quick_xml::events::{Event, BytesEnd, BytesStart, BytesText};
use quick_xml::writer::Writer;
use std::io::{Cursor, Result as IoResult, Error as IoError, ErrorKind};

#[derive(Default, Serialize, Deserialize, PartialEq, Clone)]
#[serde(default, rename_all = "lowercase")]
pub struct RawDataFiles {
    #[serde(rename = "@name")]
    name: String,

    //needs the _ because type is a reserved keyword
    #[serde(rename = "@type")]
    _type: String,
    
    value: String,
}

impl RawDataFiles {
    pub fn new() -> Self{
        RawDataFiles { 
            name: "Raw data files".to_owned(), 
            _type: "BATCH_LAST_FILES".to_owned(),
            value: "".to_owned(),
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn get_type(&self) -> &str{
        &self._type
    }

    pub fn set_type(&mut self, _type: String){
        self._type = _type;
    }

    pub fn get_value(&self) -> &str{
        &self.value
    }

    pub fn set_value(&mut self, value: &str){
        self.value = value.to_owned();
    }

    pub fn write_element(&mut self, writer: &mut Writer<Cursor<Vec<u8>>>) -> IoResult<()> {
       // create XML element -> istantiate element name (batch/batchstep/parameter/module)
       let mut element = BytesStart::new("parameter");

       // add the attribute(tag) to the element
       element.push_attribute(("name", self.name.as_str()));

       // add the attribute(tag) to the element
       element.push_attribute(("type", self._type.as_str()));
       
       // add the value as text
       writer.write_event(Event::Start(element))
           .map_err(|e| IoError::new(ErrorKind::Other, e.to_string()))?;

       // close the XML element
       if !self.value.is_empty() {
           writer.write_event(Event::Text(BytesText::new(self.value.as_str())))
               .map_err(|e| IoError::new(ErrorKind::Other, e.to_string()))?;
       }

       // Write the end tag
       writer.write_event(Event::End(BytesEnd::new("parameter")))
           .map_err(|e| IoError::new(ErrorKind::Other, e.to_string()))?;

       Ok(())
    }
}
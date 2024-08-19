use serde::{Serialize, Deserialize};

use crate::xml_serialization::*;

#[derive(Default, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(default, rename_all = "lowercase")]
pub struct MonotonicShape{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "$text")]
    value: bool,
}

impl MonotonicShape{
    pub fn new() -> Self{
        MonotonicShape{
            name: "Monotonic shape".to_owned(),
            value: true,
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn get_value(&self) -> &bool{
        &self.value
    }
    
    pub fn set_value(&mut self, value: bool){
        self.value = value;
    } 

    pub fn write_element(&self, writer: &mut Writer<Cursor<Vec<u8>>>) -> IoResult<()> {
        //<parameter name="Monotonic shape">true</parameter>
        let mut element = BytesStart::new("parameter");

        element.push_attribute(("name", self.get_name())); 

        // Write the start tag
        writer.write_event(Event::Start(element))
            .map_err(|e| IoError::new(ErrorKind::Other, e.to_string()))?;

        writer.write_event(Event::Text(BytesText::new(self.get_value().to_string().as_str())))
            .map_err(|e| IoError::new(ErrorKind::Other, e.to_string()))?;

        // Write the end tag
        writer.write_event(Event::End(BytesEnd::new("parameter")))
            .map_err(|e| IoError::new(ErrorKind::Other, e.to_string()))?;
        
        Ok(())
    } 
}
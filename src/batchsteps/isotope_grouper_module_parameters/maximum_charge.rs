use serde::{Serialize, Deserialize};

use crate::xml_serialization::*;

#[derive(Default, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(default, rename_all = "lowercase")]
pub struct MaximumCharge{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "$text")]
    value: Option<u8>,
}

impl MaximumCharge{
    pub fn new() -> Self{
        MaximumCharge{
            name: "Maximum charge".to_owned(),
            value: None,
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn get_value(&self) -> &Option<u8>{
        &self.value
    }
    
    pub fn set_value(&mut self, value: Option<u8>){
        self.value = value;
    } 

    pub fn write_element(&self, writer: &mut Writer<Cursor<Vec<u8>>>) -> IoResult<()> {
        let mut element = BytesStart::new("parameter");

        element.push_attribute(("name", self.name.as_str())); 

        // Write the start tag
        writer.write_event(Event::Start(element))
            .map_err(|e| IoError::new(ErrorKind::Other, e.to_string()))?;

        // f32 to string
        let value = match *self.get_value() {
            Some(value) => value.to_string(),
            None => "".to_string(),
        };

        writer.write_event(Event::Text(BytesText::new(&value)))
            .map_err(|e| IoError::new(ErrorKind::Other, e.to_string()))?;

            
        // Write the end tag
        writer.write_event(Event::End(BytesEnd::new("parameter")))
            .map_err(|e| IoError::new(ErrorKind::Other, e.to_string()))?;
        
        Ok(())
    } 
}
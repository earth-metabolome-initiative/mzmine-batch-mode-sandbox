use serde::{Serialize, Deserialize};

use crate::xml_serialization::*;

#[derive(Default, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(default, rename_all = "lowercase")]
pub struct NameSuffix{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "$text")]
    value: String,
}

impl NameSuffix{
    pub fn new() -> Self{
        NameSuffix{
            name: "Name suffix".to_owned(),
            value: "deiso".to_owned(),
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn get_value(&self) -> &str{
        &self.value
    }
    
    pub fn set_value(&mut self, value: &str){
        self.value = value.to_owned();
    } 

    pub fn write_element(&self, writer: &mut Writer<Cursor<Vec<u8>>>) -> IoResult<()> {
        //<parameter name="Name suffix">deiso</parameter>
        let mut element = BytesStart::new("parameter");

        element.push_attribute(("name", self.get_name())); 

        // Write the start tag
        writer.write_event(Event::Start(element))
            .map_err(|e| IoError::new(ErrorKind::Other, e.to_string()))?;

        writer.write_event(Event::Text(BytesText::new(self.get_value())))
            .map_err(|e| IoError::new(ErrorKind::Other, e.to_string()))?;

        // Write the end tag
        writer.write_event(Event::End(BytesEnd::new("parameter")))
            .map_err(|e| IoError::new(ErrorKind::Other, e.to_string()))?;
        
        Ok(())
    } 
}
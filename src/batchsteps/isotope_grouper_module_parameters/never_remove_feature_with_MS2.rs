use serde::{Serialize, Deserialize};

use crate::xml_serialization::*;

#[derive(Default, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(default, rename_all = "lowercase")]
pub struct NeverRemoveFeatureWithMs2{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "$text")]
    value: bool,
}

impl NeverRemoveFeatureWithMs2{
    pub fn new() -> Self{
        NeverRemoveFeatureWithMs2{
            name: "Never remove feature with MS2".to_owned(),
            value: false,
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
        //<parameter name="Never remove feature with MS2">true</parameter>
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
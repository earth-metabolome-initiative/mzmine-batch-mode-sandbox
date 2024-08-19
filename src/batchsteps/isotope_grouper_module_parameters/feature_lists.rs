use serde::{Serialize, Deserialize};

use crate::xml_serialization::*;

#[derive(Default, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(default, rename_all = "lowercase")]
pub struct FeatureLists{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "@type")]
    _type: String,
}

impl FeatureLists{
    pub fn new() -> Self{
        FeatureLists{
            name: "Feature lists".to_owned(),
            _type: "BATCH_LAST_FEATURELISTS".to_owned(),
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn get_type(&self) -> &str{
        &self._type
    }
    
    pub fn set_type(&mut self, _type: &str){
        self._type = _type.to_owned();
    }

    pub fn write_element(&self, writer: &mut Writer<Cursor<Vec<u8>>>) -> IoResult<()> {
        let mut element = BytesStart::new("parameter");
        
        // Set attributes with proper conversion from String to &str
        element.push_attribute(("name", self.name.as_str()));

        // Set attributes with proper conversion from String to &str
        element.push_attribute(("type", self.get_type()));
        
        // Write the start tag
        writer.write_event(Event::Start(element))
            .map_err(|e| IoError::new(ErrorKind::Other, e.to_string()))?;
        
        // Write the end tag
        writer.write_event(Event::End(BytesEnd::new("parameter")))
            .map_err(|e| IoError::new(ErrorKind::Other, e.to_string()))?;
        
        Ok(())
    } 
}
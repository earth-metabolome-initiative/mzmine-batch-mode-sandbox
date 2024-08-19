use serde::{Serialize, Deserialize};

use crate::xml_serialization::*;

#[derive(Default, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(default, rename_all = "lowercase")]
pub struct MobilityTolerance{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "@selected")]
    selected: bool,

    #[serde(rename = "$text")]
    value: Option<f32>,
}

impl MobilityTolerance{
    pub fn new() -> Self{
        MobilityTolerance{
            name: "Mobility tolerance".to_owned(),
            selected: false,
            value: None,
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn invert_selected(&mut self){
        self.selected = !self.selected;
    }

    pub fn is_selected(&self) -> &bool{
        &self.selected
    }

    pub fn select(&mut self) {
        self.selected = true;
    }

    pub fn deselect(&mut self){
        self.selected=false;
    }

    pub fn get_value(&self) -> &Option<f32>{
        &self.value
    }
    
    pub fn set_value(&mut self, value: Option<f32>){
        self.value = value;
    } 

    pub fn write_element(&self, writer: &mut Writer<Cursor<Vec<u8>>>) -> IoResult<()> {
        let mut element = BytesStart::new("parameter");

        element.push_attribute(("name", self.get_name())); 

        element.push_attribute(("selected", self.is_selected().to_string().as_str())); 

        // Write the start tag
        writer.write_event(Event::Start(element))
            .map_err(|e| IoError::new(ErrorKind::Other, e.to_string()))?;

        // f32 to string
        let value = match *self.get_value() {
            Some(value) => format!("{:.1}", value),
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
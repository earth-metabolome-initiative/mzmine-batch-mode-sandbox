use serde::{Serialize, Deserialize};

use crate::xml_serialization::*;

#[derive(Default, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct RetentionTimeTolerance{
    #[serde(rename="@name")]
    name: String,

    #[serde(rename="@unit")]
    unit: String,

    #[serde(rename="$text")]
    value: Option<f32>,
}

impl RetentionTimeTolerance{
    pub fn new() -> Self{
        RetentionTimeTolerance{
            name: "Retention time tolerance".to_owned(),
            unit: "MINUTES".to_owned(),
            value: None,
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn get_unit(&self) -> &str{
        &self.unit
    }

    pub fn set_unit(&mut self, unit:&str){
        self.unit = unit.to_owned();
    } 

    pub fn get_value(&self) -> &Option<f32>{
        &self.value
    }

    pub fn set_value(&mut self, value:Option<f32>){
        self.value = value;
    }

    pub fn write_element(&self, writer: &mut Writer<Cursor<Vec<u8>>>) -> IoResult<()> {
        //<parameter name="Retention time tolerance" unit="MINUTES">0.04</parameter>
        let mut element = BytesStart::new("parameter");

        element.push_attribute(("name", self.get_name())); 
        element.push_attribute(("unit", self.get_unit())); 

        // Write the start tag
        writer.write_event(Event::Start(element))
            .map_err(|e| IoError::new(ErrorKind::Other, e.to_string()))?;

        let value = match *self.get_value(){
            Some(value) => value.to_string(),
            None => "".to_string()
        };

        writer.write_event(Event::Text(BytesText::new(&value)))
            .map_err(|e| IoError::new(ErrorKind::Other, e.to_string()))?;

        // Write the end tag
        writer.write_event(Event::End(BytesEnd::new("parameter")))
            .map_err(|e| IoError::new(ErrorKind::Other, e.to_string()))?;
        
        Ok(())
    }
}
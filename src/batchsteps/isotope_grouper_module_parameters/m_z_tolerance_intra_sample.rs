use serde::{Serialize, Deserialize};

use crate::xml_serialization::*;

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(default, rename_all = "lowercase")]
pub struct MzToleranceIntraSample{
    #[serde(rename = "@name")]
    name: String,

    parameter: Vec<MzToleranceIntraSampleParameters>
}

impl MzToleranceIntraSample{
    pub fn new() -> Self{
        MzToleranceIntraSample{
            name: "m/z tolerance (intra-sample)".to_owned(),
            parameter: Vec::new()
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn get_parameter_length(&self) -> usize{
        self.parameter.len()
    }

    pub fn add_parameter(&mut self, parameter:MzToleranceIntraSampleParameters){
        self.parameter.push(parameter);
    }

    pub fn get_parameter(&mut self, target: &str) -> Option<&mut MzToleranceIntraSampleParameters> {
        for parameter in &mut self.parameter {
            match (&mut *parameter, target) {
                (MzToleranceIntraSampleParameters::PpmTolerance(_), "Ppmtolerance") => return Some(parameter),
                (MzToleranceIntraSampleParameters::AbsoluteTolerance(_), "Absolutetolerance") => return Some(parameter),
                _ => panic!("No matching parameter")
            }
        }
        None
    }
    
    pub fn write_element(&mut self, writer: &mut Writer<Cursor<Vec<u8>>>) -> IoResult<()>{
        let mut element = BytesStart::new("parameter");

        // add the attribute(tag) to the element
        element.push_attribute(("name", self.get_name()));
        
        // Write the start tag
        writer.write_event(Event::Start(element))
            .map_err(|e| IoError::new(ErrorKind::Other, e.to_string()))?;

        if !self.parameter.is_empty(){
            for parameter in &mut self.parameter{
                match parameter{
                    MzToleranceIntraSampleParameters::AbsoluteTolerance(_f) => parameter.write_element(writer)?,
                    MzToleranceIntraSampleParameters::PpmTolerance(_f) => parameter.write_element(writer)?,
                    _ => panic!("No matching parameter")
                }
            }
        }

        writer.write_event(Event::End(BytesEnd::new("parameter")))
            .map_err(|e| IoError::new(ErrorKind::Other, e.to_string()))?;
    
        Ok(())
    }

}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(untagged)]
pub enum MzToleranceIntraSampleParameters{
    PpmTolerance(PpmTolerance),
    AbsoluteTolerance(AbsoluteTolerance),
}

impl MzToleranceIntraSampleParameters{
    pub fn get_value(&self) -> &Option<f32>{
        match self{
            MzToleranceIntraSampleParameters::AbsoluteTolerance(_f) => _f.get_value(),
            MzToleranceIntraSampleParameters::PpmTolerance(_f) => _f.get_value(),
            _ => panic!("No matching parameter")
        }
    }

    pub fn set_value(&mut self, value:Option<f32>){
        match self{
            MzToleranceIntraSampleParameters::AbsoluteTolerance(_f) => _f.set_value(value),
            MzToleranceIntraSampleParameters::PpmTolerance(_f) => _f.set_value(value),
            _ => panic!("No matching parameter")
        }
    }

    pub fn write_element(&self, writer:&mut Writer<Cursor<Vec<u8>>>) -> IoResult<()>{
        match self{
            MzToleranceIntraSampleParameters::AbsoluteTolerance(_f) => return _f.write_element(writer),
            MzToleranceIntraSampleParameters::PpmTolerance(_f) => return _f.write_element(writer),
            _ => panic!("No matching parameter")
        }
        Ok(())
    }
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(default, rename_all = "lowercase", rename = "ppmtolerance")]
pub struct PpmTolerance{
    #[serde(rename = "$text")]
    value: Option<f32>,
}

impl PpmTolerance{
    pub fn new() -> Self{
        PpmTolerance{
            value: None
        }
    }

    pub fn get_value(&self) -> &Option<f32>{
        &self.value
    }

    pub fn set_value(&mut self, value: Option<f32>){
        self.value = value;
    }

    pub fn write_element(&self, writer: &mut Writer<Cursor<Vec<u8>>>) -> IoResult<()> {
        let mut element = BytesStart::new("ppmtolerance");
        
        // Write the start tag
        writer.write_event(Event::Start(element))
            .map_err(|e| IoError::new(ErrorKind::Other, e.to_string()))?;

        let value = match *self.get_value(){
            Some(value) => format!("{:.1}", value),
            None => "".to_string(),
        };
        
        writer.write_event(Event::Text(BytesText::new(&value)))
            .map_err(|e| IoError::new(ErrorKind::Other, e.to_string()))?;
            
        // Write the end tag
        writer.write_event(Event::End(BytesEnd::new("ppmtolerance")))
            .map_err(|e| IoError::new(ErrorKind::Other, e.to_string()))?;
        
        Ok(())
    } 

}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(default, rename_all = "lowercase", rename = "absolutetolerance")]
pub struct AbsoluteTolerance{
    #[serde(rename = "$text")]
    value: Option<f32>,
}

impl AbsoluteTolerance{
    pub fn new() -> Self{
        AbsoluteTolerance{
            value: None
        }
    }

    pub fn get_value(&self) -> &Option<f32>{
        &self.value
    }

    pub fn set_value(&mut self, value: Option<f32>){
        self.value = value;
    }

    pub fn write_element(&self, writer: &mut Writer<Cursor<Vec<u8>>>) -> IoResult<()> {
        let mut element = BytesStart::new("absolutetolerance");
        
        // Write the start tag
        writer.write_event(Event::Start(element))
            .map_err(|e| IoError::new(ErrorKind::Other, e.to_string()))?;

        let value = match *self.get_value() {
            Some(value) => value.to_string(),
            None => "".to_string(),
        };
        
        if !value.is_empty() {
            writer.write_event(Event::Text(BytesText::new(&value)))
                .map_err(|e| IoError::new(ErrorKind::Other, e.to_string()))?;
        }
            
        // Write the end tag
        writer.write_event(Event::End(BytesEnd::new("absolutetolerance")))
            .map_err(|e| IoError::new(ErrorKind::Other, e.to_string()))?;
        
        Ok(())
    } 
}
use core::panic;

use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(default, rename_all = "lowercase")]
pub struct Ms1Ms2PrecursorTolerance{
    #[serde(rename = "@name")]
    name: String,

    parameter: Vec<ToleranceParameters>
}

impl Ms1Ms2PrecursorTolerance{
    pub fn new() -> Self{
        Ms1Ms2PrecursorTolerance{
            name: "MS1 to MS2 precursor tolerance (m/z)".to_owned(),
            parameter: Vec::new()
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn get_parameter_length(&self) -> usize{
        self.parameter.len()
    }

    pub fn add_parameter(&mut self, parameter:ToleranceParameters){
        self.parameter.push(parameter);
    }

    pub fn get_parameter(&mut self, target: &str) -> Option<&mut ToleranceParameters> {
        for parameter in &mut self.parameter {
            match (&mut *parameter, target) {
                (ToleranceParameters::PpmTolerance(_), "Ppmtolerance") => return Some(parameter),
                (ToleranceParameters::AbsoluteTolerance(_), "Absolutetolerance") => return Some(parameter),
                _ => (),
            }
        }
        None
    }
    
    

}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(untagged)]
pub enum ToleranceParameters{
    PpmTolerance(PpmTolerance),
    AbsoluteTolerance(AbsoluteTolerance),
}

impl ToleranceParameters{
    pub fn get_value(&self) -> &Option<f32>{
        match self{
            ToleranceParameters::AbsoluteTolerance(_f) => _f.get_value(),
            ToleranceParameters::PpmTolerance(_f) => _f.get_value(),
            _ => panic!("No matching parameter")
        }
    }

    pub fn set_value(&mut self, value:Option<f32>){
        match self{
            ToleranceParameters::AbsoluteTolerance(_f) => _f.set_value(value),
            ToleranceParameters::PpmTolerance(_f) => _f.set_value(value),
            _ => panic!("No matching parameter")
        }
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

}
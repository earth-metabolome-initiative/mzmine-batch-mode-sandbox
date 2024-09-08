use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(default, rename_all = "lowercase", rename = "parameter")]
pub struct MzToleranceIntraSample{
    #[serde(rename = "@name")]
    name: String,

    parameters: Vec<MzToleranceIntraSampleParameters>
}

impl MzToleranceIntraSample{
    pub fn new() -> Self{
        MzToleranceIntraSample{
            name: "m/z tolerance (intra-sample)".to_owned(),
            parameters: Vec::new()
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn get_parameter_length(&self) -> usize{
        self.parameters.len()
    }

    pub fn add_parameter(&mut self, parameter:MzToleranceIntraSampleParameters){
        self.parameters.push(parameter);
    }

    pub fn get_parameter(&mut self, target: &str) -> Option<&mut MzToleranceIntraSampleParameters> {
        for parameter in &mut self.parameters {
            match (&mut *parameter, target) {
                (MzToleranceIntraSampleParameters::PpmTolerance(_), "Ppmtolerance") => return Some(parameter),
                (MzToleranceIntraSampleParameters::AbsoluteTolerance(_), "Absolutetolerance") => return Some(parameter),
                _ => panic!("No matching parameter")
            }
        }
        None
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
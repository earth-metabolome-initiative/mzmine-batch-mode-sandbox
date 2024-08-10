use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(default, rename_all = "lowercase")]
pub struct ScanTypes {
    #[serde(rename = "@name")]
    name: String,
    
    parameters: Vec<ScanTypesParameter>,
}

impl ScanTypes {
    pub fn new() -> Self{
        ScanTypes { 
            name: "Scan types (IMS)".to_owned(), 
            parameters: Vec::new()
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn add_parameter(&mut self, parameter:ScanTypesParameter){
        self.parameters.push(parameter);
    }

    pub fn get_parameters_length(&self) -> usize{
        self.parameters.len()
    }
}

#[derive(Default, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(default, rename = "file", rename_all = "lowercase")]
pub struct ScanTypesParameter{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "$text")]
    parameter: String
}

impl ScanTypesParameter{
    pub fn new() -> Self{
        ScanTypesParameter{
            name: "parameter".to_owned(),
            parameter: "All scan types".to_owned()
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn get_parameter(&self) -> &str{
        &self.parameter
    }

    pub fn set_parameter(&mut self, parameter:&str){
        self.parameter = parameter.to_owned();
    }
}
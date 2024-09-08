use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(default, rename_all = "lowercase", rename = "parameter")]
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
}
use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(default, rename_all = "lowercase", rename = "parameter")]
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
}
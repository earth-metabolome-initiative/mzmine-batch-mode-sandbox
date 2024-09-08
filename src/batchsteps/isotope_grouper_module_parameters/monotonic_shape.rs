use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(default, rename_all = "lowercase", rename = "parameter")]
pub struct MonotonicShape{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "$text")]
    value: bool,
}

impl MonotonicShape{
    pub fn new() -> Self{
        MonotonicShape{
            name: "Monotonic shape".to_owned(),
            value: true,
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
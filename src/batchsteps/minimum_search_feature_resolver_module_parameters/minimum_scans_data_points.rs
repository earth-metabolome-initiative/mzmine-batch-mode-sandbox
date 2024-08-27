use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(default, rename_all = "lowercase", rename = "parameter")]
pub struct MinimumScansDataPoints{
    #[serde(rename ="@name")]
    name: String,
    
    #[serde(rename ="$text")]
    value: Option<u8>,
}

impl MinimumScansDataPoints{
    pub fn new() -> Self{
        MinimumScansDataPoints{
            name: "Minimum scans (data points)".to_owned(),
            value: None,
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn get_value(&self) -> &Option<u8>{
        &self.value
    }

    pub fn set_value(&mut self, value:Option<u8>){
        self.value = value;
    }
}
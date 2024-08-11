use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
pub struct MinimumConsecutiveScans{
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "$text")]
    value: Option<u8>,
}

impl MinimumConsecutiveScans{
    pub fn new() -> Self{
        MinimumConsecutiveScans{
            name: "Minimum consecutive scans".to_owned(),
            value: None,
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn set_value(&mut self, value: Option<u8>){
        self.value = value;
    }

    pub fn get_value(&self) -> &Option<u8>{
        &self.value
    }
}
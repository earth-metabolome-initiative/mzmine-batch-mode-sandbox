use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
pub struct AllowSingleScanChromatograms{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "$text")]
    value: Option<f32>
}

impl AllowSingleScanChromatograms{
    pub fn new() -> Self{
        AllowSingleScanChromatograms{
            name: "Allow single scan chromatograms".to_owned(),
            value: None,
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn get_value(&self) -> &Option<f32>{
        &self.value
    }

    pub fn set_value(&mut self, value:Option<f32>){
        self.value = value;
    }
}
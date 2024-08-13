use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(default, rename_all = "lowercase")]
pub struct MergeMsMsSpectraTIMS{
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "$text")]
    value: bool,
}

impl MergeMsMsSpectraTIMS{
    pub fn new() -> Self{
        MergeMsMsSpectraTIMS{
            name: "Merge MS/MS spectra (TIMS)".to_owned(),
            value: false,
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn get_value(&self) -> &bool{
        &self.value
    }

    pub fn set_value(&mut self, value:bool){
        self.value = value;
    } 

    pub fn invert_value(&mut self){
        self.value = !self.value;
    }
}

use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(default, rename_all = "lowercase")]
pub struct DenormalizeFragmentScanTraps {
    #[serde(rename = "@name")]
    name: String,
    
    value: bool,
}

impl DenormalizeFragmentScanTraps {
    pub fn new() -> Self{
        DenormalizeFragmentScanTraps { 
            name: "Denormalize fragment scans (traps)".to_owned(), 
            value: true,
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn set_value(&mut self, value:bool){
        self.value = value;
    }

    pub fn get_value(&self) -> &bool{
        &self.value
    }

    pub fn switch_value(&mut self){
        self.value = !self.value;
    }
}
use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(default, rename_all = "lowercase")]
pub struct MinimumSignalIntensityRelativeTIMS{
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@selected")]
    selected: bool,
    #[serde(rename = "$text")]
    value: Option<u8>,
}

impl MinimumSignalIntensityRelativeTIMS{
    pub fn new() -> Self{
        MinimumSignalIntensityRelativeTIMS{
            name: "Minimum signal intensity (relative, TIMS)".to_owned(),
            selected: false,
            value: None,
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn invert_selected(&mut self){
        self.selected = !self.selected;
    }

    pub fn is_selected(&self) -> bool{
        self.selected
    }

    pub fn select(&mut self) {
        self.selected = true;
    }

    pub fn deselect(&mut self){
        self.selected=false;
    }

    pub fn get_value(&self) -> &Option<u8>{
        &self.value
    }

    pub fn set_value(&mut self, value:Option<u8>){
        self.value = value;
    } 
}
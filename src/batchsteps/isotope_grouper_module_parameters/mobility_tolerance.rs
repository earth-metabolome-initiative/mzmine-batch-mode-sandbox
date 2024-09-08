use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(default, rename_all = "lowercase", rename = "parameter")]
pub struct MobilityTolerance{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "@selected")]
    selected: bool,

    #[serde(rename = "$text")]
    value: Option<f32>,
}

impl MobilityTolerance{
    pub fn new() -> Self{
        MobilityTolerance{
            name: "Mobility tolerance".to_owned(),
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

    pub fn is_selected(&self) -> &bool{
        &self.selected
    }

    pub fn select(&mut self) {
        self.selected = true;
    }

    pub fn deselect(&mut self){
        self.selected=false;
    }

    pub fn get_value(&self) -> &Option<f32>{
        &self.value
    }
    
    pub fn set_value(&mut self, value: Option<f32>){
        self.value = value;
    } 
}
use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(default, rename_all = "lowercase")]
pub struct MinimumRelativeFeatureHeight{
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@selected")]
    selected: bool,
    #[serde(rename = "$text")]
    value: Option<f32>,
}

impl MinimumRelativeFeatureHeight{
    pub fn new() -> Self{
        MinimumRelativeFeatureHeight{
            name: "Minimum relative feature height".to_owned(),
            selected: true,
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

    pub fn get_value(&self) -> &Option<f32>{
        &self.value
    }

    pub fn set_value(&mut self, value:Option<f32>){
        self.value = value;
    } 
}
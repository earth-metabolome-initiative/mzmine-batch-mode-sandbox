use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all ="lowercase", rename = "parameter")]
pub struct MinimumFeaturesInAnIsotopePattern{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "@selected")]
    selected: bool,

    #[serde(rename = "$text")]
    value: Option<u8>
}

impl MinimumFeaturesInAnIsotopePattern{
    pub fn new() -> Self{
        MinimumFeaturesInAnIsotopePattern{
            name: "Minimum features in an isotope pattern".to_owned(),
            selected: false,
            value: None,
        }
    }
    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn is_selected(&self) -> &bool{
        &self.selected
    }

    pub fn select(&mut self){
        self.selected = true;
    }

    pub fn deselect(&mut self){
        self.selected = false;
    }

    pub fn get_value(&self) -> &Option<u8>{
        &self.value
    }

    pub fn set_value(&mut self, value:Option<u8>){
        self.value = value;
    }
}
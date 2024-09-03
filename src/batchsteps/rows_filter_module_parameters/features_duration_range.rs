use serde::{Serialize, Deserialize};

use super::chromatographic_FWHM::MinMax;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all ="lowercase", rename = "parameter")]
pub struct FeaturesDurationRange{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "@selected")]
    selected: bool,

    //steal the MinMax from ChromatographicFWHM
    #[serde(rename = "min")]
    min: MinMax,

    #[serde(rename = "max")]
    max: MinMax
}

impl Default for FeaturesDurationRange{
    fn default() -> Self {
        FeaturesDurationRange{
            name: "features duration range".to_owned(),
            selected: false,
            min: MinMax::new(),
            max: MinMax::new()
        }
    }
}

impl FeaturesDurationRange{
    pub fn new(selected:bool, min:Option<f32>, max:Option<f32>) -> Self{
        let mut minimum = MinMax::new();
        minimum.set_value(min);
        
        let mut maximum = MinMax::new();
        maximum.set_value(max);

        FeaturesDurationRange{
            name: "features duration range".to_owned(),
            selected: selected,
            min: minimum,
            max: maximum
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

    pub fn get_min_value(&self) -> &Option<f32>{
        self.min.get_value()
    }

    pub fn get_max_value(&self) -> &Option<f32>{
        self.max.get_value()
    }

    pub fn set_min_value(&mut self, value:Option<f32>){
        self.min.set_value(value);
    } 

    pub fn set_max_value(&mut self, value:Option<f32>){
        self.max.set_value(value);
    }
}
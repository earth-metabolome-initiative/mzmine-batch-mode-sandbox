use serde::{Serialize, Deserialize};

use crate::batchsteps::rows_filter_module_parameters::chromatographic_FWHM::MinMax;

#[derive(Default, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all ="lowercase", rename = "parameter")]
pub struct Charge{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "@selected")]
    selected: bool,

    #[serde(rename = "min")]
    min: MinMax,

    #[serde(rename = "max")]
    max: MinMax
}

impl Charge{
    pub fn new() -> Self{
        Charge{
            name: "Charge".to_owned(),
            selected: false,
            min: MinMax::new(),
            max: MinMax::new()
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
        self.selected = false
    }

    pub fn set_min_value(&mut self, value: Option<f32>){
        self.min.set_value(value);
    }

    pub fn set_max_value(&mut self, value: Option<f32>){
        self.max.set_value(value);
    }

    pub fn get_min_value(&self) -> &Option<f32>{
        self.min.get_value()
    }

    pub fn get_max_value(&self) -> &Option<f32>{
        self.max.get_value()
    }
}
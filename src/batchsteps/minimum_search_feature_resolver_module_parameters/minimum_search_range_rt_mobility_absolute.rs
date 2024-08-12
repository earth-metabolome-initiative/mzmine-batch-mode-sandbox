use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
pub struct MinimumSearchRangeRTMobilityAbsolute{
    #[serde(rename="@name")]
    name: String,

    #[serde(rename = "@name")]
    value: Option<f32>,
}

impl MinimumSearchRangeRTMobilityAbsolute{
    pub fn new() -> Self{
        MinimumSearchRangeRTMobilityAbsolute{
            name: "Minimum search range RT/Mobility (absolute)".to_owned(),
            value: None,
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn set_value(&mut self, value:Option<f32>){
        self.value = value;
    }

    pub fn get_value(&self) -> &Option<f32>{
        &self.value
    }
}
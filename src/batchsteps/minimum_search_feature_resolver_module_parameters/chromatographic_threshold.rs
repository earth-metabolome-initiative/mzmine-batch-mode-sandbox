use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(default, rename_all = "lowercase", rename = "parameter")]
pub struct ChromatographicThreshold{
    #[serde(rename="@name")]
    name: String,

    #[serde(rename = "$text")]
    value: Option<f32>,
}

impl ChromatographicThreshold{
    pub fn new() -> Self{
        ChromatographicThreshold{
            name: "Chromatographic threshold".to_owned(),
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
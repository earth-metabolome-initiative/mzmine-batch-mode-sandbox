use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(default, rename_all = "lowercase", rename = "parameter")]
pub struct MinimumRelativeHeight{
    #[serde(rename="@name")]
    name: String,

    #[serde(rename = "$text")]
    value: Option<f32>,
}

impl MinimumRelativeHeight{
    pub fn new() -> Self{
        MinimumRelativeHeight{
            name: "Minimum relative height".to_owned(),
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
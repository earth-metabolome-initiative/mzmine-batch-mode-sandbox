use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
pub struct MinimumAbsoluteHeight{
    #[serde(rename="@name")]
    name: String,

    #[serde(rename="$text")]
    value: Option<f32>
}

impl MinimumAbsoluteHeight{
    pub fn new() -> Self{
        MinimumAbsoluteHeight{
            name: "Minimum absolute height".to_owned(),
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
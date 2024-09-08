use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(default, rename_all = "lowercase", rename = "parameter")]
pub struct OriginalFeatureList{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "$text")]
    value: String,
}

impl OriginalFeatureList{
    pub fn new() -> Self{
        OriginalFeatureList{
            name: "Original feature list".to_owned(),
            value: "".to_owned(),
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn get_value(&self) -> &str{
        &self.value
    }
    
    pub fn set_value(&mut self, value: &str){
        self.value = value.to_owned();
    } 
}
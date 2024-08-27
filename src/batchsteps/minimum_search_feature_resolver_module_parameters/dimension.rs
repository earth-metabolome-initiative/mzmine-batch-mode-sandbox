use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(default, rename_all = "lowercase", rename="parameter")]
pub struct Dimension{
    #[serde(rename="@name")]
    name: String,

    #[serde(rename = "$text")]
    value: String,
}

impl Dimension{
    pub fn new() -> Self{
        Dimension{
            name: "Dimension".to_owned(),
            value: "".to_owned(),
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn set_value(&mut self, value:&str){
        self.value = value.to_owned();
    }

    pub fn get_value(&self) -> &str{
        &self.value
    }
}
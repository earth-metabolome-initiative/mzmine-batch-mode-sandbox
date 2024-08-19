use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(default, rename_all = "lowercase")]
pub struct Suffix{
    #[serde(rename="@name")]
    name: String,

    #[serde(rename = "$text")]
    value: char,
}

impl Suffix{
    pub fn new() -> Self{
        Suffix{
            name: "Suffix".to_owned(),
            value: 'r',
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn set_value(&mut self, value:char){
        self.value = value;
    }

    pub fn get_value(&self) -> &char{
        &self.value
    }
}
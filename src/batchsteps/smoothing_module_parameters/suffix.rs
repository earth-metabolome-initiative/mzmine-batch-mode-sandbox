use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(default, rename_all = "lowercase", rename = "parameter")]
pub struct Suffix{
    #[serde(rename="@name")]
    name: String,

    #[serde(rename="$text")]
    value: String,
}

impl Suffix{
    pub fn new() -> Self{
        Suffix{
            name: "Suffix".to_owned(),
            value: "".to_owned()
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn get_value(&self) -> &str{
        &self.value
    }

    pub fn set_value(&mut self, value:&str){
        self.value = value.to_owned();
    }
}
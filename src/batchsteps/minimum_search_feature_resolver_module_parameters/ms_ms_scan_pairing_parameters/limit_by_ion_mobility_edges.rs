use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(default, rename_all = "lowercase")]
pub struct LimitByIonMobilityEdges{
    #[serde(rename="@name")]
    name: String,

    #[serde(rename="$text")]
    value: bool,
}

impl LimitByIonMobilityEdges{
    pub fn new() -> Self{
        LimitByIonMobilityEdges{
            name: "Limit by ion mobility edges".to_owned(),
            value: true,
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn get_value(&self) -> &bool{
        &self.value
    }

    pub fn set_value(&mut self, value:bool){
        self.value = value;
    }

    pub fn invert_value(&mut self){
        self.value = !self.value;
    }
}
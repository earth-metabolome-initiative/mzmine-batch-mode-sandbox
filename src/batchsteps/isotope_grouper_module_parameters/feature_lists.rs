use serde::{Serialize, Deserialize};

use crate::isotope_grouper_module::*;

#[derive(Default, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(default, rename_all = "lowercase")]
pub struct FeatureLists{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "@type")]
    _type: String,
}

impl FeatureLists{
    pub fn new() -> Self{
        FeatureLists{
            name: "Feature lists".to_owned(),
            _type: "BATCH_LAST_FEATURELISTS".to_owned(),
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn get_type(&self) -> &str{
        &self._type
    }
    
    pub fn set_type(&mut self, _type: &str){
        self._type = _type.to_owned();
    } 
}
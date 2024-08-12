use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize)]
#[serde(default, rename_all = "lowercase")]
pub struct FeatureLists{
    #[serde(rename="@name")]
    name: String,

    #[serde(rename = "@name")]
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

    pub fn set_type(&mut self, _type:&str){
        self._type = _type.to_owned();
    }

    pub fn get_type(&self) -> &str{
        &self._type
    }
}
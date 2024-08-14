use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(default, rename_all = "lowercase")]
pub struct FeatureLists{
    #[serde(rename="@name")]
    name: String,

    #[serde(rename="@type")]
    _type: String,

    #[serde(rename="$text")]
    value: Option<f32>
}

impl FeatureLists{
    pub fn new() -> Self{
        FeatureLists{
            name: "Feature lists".to_owned(),
            _type: "BATCH_LAST_FEATURELISTS".to_owned(),
            value: None,
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
    pub fn set_value(&mut self, value:Option<f32>){
        self.value = value;
    }

    pub fn get_value(&self) -> &Option<f32>{
        &self.value
    }
}
use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(default, rename_all = "lowercase", rename = "parameter")]
pub struct MinRatioOfPeakTopEdge{
    #[serde(rename="@name")]
    name: String,

    #[serde(rename = "$text")]
    value: Option<f32>,
}

impl MinRatioOfPeakTopEdge{
    pub fn new() -> Self{
        MinRatioOfPeakTopEdge{
            name: "Min ratio of peak top/edge".to_owned(),
            value: None,
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn get_value(&self) -> &Option<f32>{
        &self.value
    }

    pub fn set_value(&mut self, value: Option<f32>){
        self.value = value;
    }

    
}
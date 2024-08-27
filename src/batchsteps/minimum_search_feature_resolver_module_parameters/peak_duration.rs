use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(default, rename_all = "lowercase", rename = "parameter")]
pub struct PeakDuration{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "min")]
    min: MinMax,
    
    #[serde(rename = "max")]
    max: MinMax
}

impl PeakDuration{
    pub fn new() -> Self{
        PeakDuration{
            name: "Peak duration range (min/mobility)".to_owned(),
            min: MinMax::new(),
            max: MinMax::new()  
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn get_min_value(&self) -> &str{
        self.min.get_value()
    }

    pub fn get_max_value(&self) -> &str{
        self.max.get_value()
    }

    pub fn set_min_value(&mut self, value:Option<f64>){
        self.min.set_value(value);
    }

    pub fn set_max_value(&mut self, value:Option<f64>){
        self.max.set_value(value);
    }
}


#[derive(Default, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(default, rename_all = "lowercase")]
pub struct MinMax{
    #[serde(rename = "$text")]
    value: String
}

impl MinMax{
    pub fn new() -> Self{
        MinMax{
            value: "".to_owned()
        }
    }

    fn get_value(&self) -> &str{
        &self.value
    }

    pub fn set_value(&mut self, value: Option<f64>) {
        self.value = match value {
            Some(v) => Some(format!("{:.16}", v)).unwrap(), // Format to 16 decimal places
            None => "".to_owned(),
        };
    }
}
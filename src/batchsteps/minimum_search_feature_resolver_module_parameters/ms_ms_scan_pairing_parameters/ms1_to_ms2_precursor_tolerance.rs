use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(default, rename_all = "lowercase", rename = "parameter")]
pub struct Ms1Ms2PrecursorTolerance{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "absolutetolerance")]
    absolute_tolerance: Tolerance,

    #[serde(rename = "ppmtolerance")]  
    ppm_tolerance: Tolerance
}

impl Ms1Ms2PrecursorTolerance{
    pub fn new() -> Self{
        Ms1Ms2PrecursorTolerance{
            name: "MS1 to MS2 precursor tolerance (m/z)".to_owned(),
            absolute_tolerance: Tolerance::new(),
            ppm_tolerance: Tolerance::new()
        }
    }

    pub fn new_isotope_grouper_module() -> Self{
        Ms1Ms2PrecursorTolerance{
            name: "m/z tolerance (intra-sample)".to_string(),
            absolute_tolerance: Tolerance::new(),
            ppm_tolerance: Tolerance::new()
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn set_name(&mut self, name:&str){
        self.name = name.to_owned();
    }

    pub fn get_absolute_value(&self) -> &Option<f32>{
        self.absolute_tolerance.get_value()
    }

    pub fn get_ppm_value(&self) -> &Option<f32>{
        self.ppm_tolerance.get_value()
    }

    pub fn set_absolute_value(&mut self, value: Option<f32>){
        self.absolute_tolerance.set_value(value);
    }

    pub fn set_ppm_value(&mut self, value: Option<f32>){
        self.ppm_tolerance.set_value(value);
    }
}

#[derive(Default, Serialize, Deserialize, Debug, Clone, PartialEq)]
#[serde(default, rename_all = "lowercase")]
struct Tolerance{
    #[serde(rename = "$text")]
    value: Option<f32>
}

impl Tolerance{
    pub fn new() -> Self{
        Tolerance{
            value: None
        }
    }

    fn set_value(&mut self, value: Option<f32>){
        self.value = value;
    }

    fn get_value(&self) -> &Option<f32>{
        &self.value
    }
}
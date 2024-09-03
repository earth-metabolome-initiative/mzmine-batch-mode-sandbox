use serde::{Serialize, Deserialize};

use super::chromatographic_FWHM::MinMax;

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all ="lowercase", rename = "parameter")]
pub struct KendrickMassDefect{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "@selected")]
    selected: bool,

    #[serde(rename = "parameter")]
    parameters: Vec<KendrickMassDefectParameters>
}

impl Default for KendrickMassDefect{
    fn default() -> Self {
        KendrickMassDefect{
            name: "Kendrick mass defect".to_owned(),
            selected: false,
            parameters: Vec::new()
        }
    }
}

impl KendrickMassDefect {
    pub fn new() -> Self{
        KendrickMassDefect{
            name: "Kendrick mass defect".to_owned(),
            selected: false,
            parameters: Vec::new()
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn is_selected(&self) -> &bool{
        &self.selected
    }

    pub fn select(&mut self){
        self.selected = true;
    }
    
    pub fn deselect(&mut self){
        self.selected = false;
    }

    pub fn get_parameters_length(&self) -> usize{
        self.parameters.len()
    }

    pub fn add_parameter(&mut self, parameter:KendrickMassDefectParameters){
        self.parameters.push(parameter)
    }

    pub fn get_parameter(&mut self, target: &str) -> Option<&KendrickMassDefectParameters>{
        self.parameters.iter().find(|param| match param {
            KendrickMassDefectParameters::KendrickMassDefectParameter(_) => "KendrickMassDefectParameter" == target,
            KendrickMassDefectParameters::KendrickMassBase(_) => "KendrickMassBase" == target,
            KendrickMassDefectParameters::Shift(_) => "Shift" == target,
            KendrickMassDefectParameters::ChargeParameter(_) => "Charge" == target,
            KendrickMassDefectParameters::Divisor(_) => "Divisor" == target,
            KendrickMassDefectParameters::UseRemainderOfKendrickMass(_) => "UseRemainderOfKendrickMass" == target,
        })
    }
}


#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum KendrickMassDefectParameters{
    KendrickMassDefectParameter(KendrickMassDefectParameter),
    KendrickMassBase(KendrickMassBase),
    Shift(Shift),
    ChargeParameter(ChargeParameter),
    Divisor(Divisor),
    UseRemainderOfKendrickMass(UseRemainderOfKendrickMass)
}

impl KendrickMassDefectParameters{
    pub fn get_name(&self) -> &str{
        match self{
            KendrickMassDefectParameters::KendrickMassDefectParameter(_f) => return _f.get_name(),
            KendrickMassDefectParameters::KendrickMassBase(_f) => return _f.get_name(),
            KendrickMassDefectParameters::Shift(_f) => return _f.get_name(),
            KendrickMassDefectParameters::ChargeParameter(_f) => return _f.get_name(),
            KendrickMassDefectParameters::Divisor(_f) => return _f.get_name(),
            KendrickMassDefectParameters::UseRemainderOfKendrickMass(_f) => return _f.get_name(),
            _ => panic!("No matching parameter")
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all ="lowercase", rename = "parameter")]
pub struct KendrickMassDefectParameter{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "min")]
    min: MinMax,

    #[serde(rename = "max")]
    max: MinMax,
}

impl Default for KendrickMassDefectParameter{
    fn default() -> Self {
        KendrickMassDefectParameter{
            name: "Kendrick mass defect".to_owned(),
            min: MinMax::new(),
            max: MinMax::new()
        }
    }
}

impl KendrickMassDefectParameter{
    pub fn new(min: Option<f32>, max:Option<f32>) -> Self{
        let mut minimum = MinMax::new();
        minimum.set_value(min);
        
        let mut maximum = MinMax::new();
        maximum.set_value(max);

        KendrickMassDefectParameter{
            name: "Kendrick mass defect".to_owned(),
            min: minimum,
            max: maximum
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn get_min_value(&self) -> &Option<f32>{
        self.min.get_value()
    }

    pub fn get_max_value(&self) -> &Option<f32>{
        self.max.get_value()
    }

    pub fn set_min_value(&mut self, value:Option<f32>){
        self.min.set_value(value);
    }

    pub fn set_max_value(&mut self, value: Option<f32>){
        self.max.set_value(value);
    }
}

#[derive(Default, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all ="lowercase", rename = "parameter")]
pub struct KendrickMassBase{
    #[serde(rename = "@name")]
    name: String,

    #[serde(skip_serializing_if = "Option::is_none", rename = "$text")]
    value: Option<f32>
}

impl KendrickMassBase {
    pub fn new() -> Self{
        KendrickMassBase{
            name: "Kendrick mass base".to_owned(),
            value: None
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn get_value(&self) -> &Option<f32>{
        &self.value
    }

    pub fn set_value(&mut self, value:Option<f32>){
        self.value = value;
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all ="lowercase", rename = "parameter")]
pub struct Shift{
    #[serde(rename = "@name")]
    name: String,

    #[serde(skip_serializing_if = "Option::is_none", rename = "$text")]
    value: Option<f32>
}

impl Default for Shift{
    fn default() -> Self {
        Shift{
            name: "Shift".to_owned(),
            value: None 
        }
    }
}

impl Shift {
    pub fn new(value:Option<f32>) -> Self{
        Shift{
            name: "Shift".to_owned(),
            value: value
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn get_value(&self) -> &Option<f32>{
        &self.value
    }

    pub fn set_value(&mut self, value:Option<f32>){
        self.value = value;
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all ="lowercase", rename = "parameter")]
pub struct ChargeParameter{
    #[serde(rename = "@name")]
    name: String,

    #[serde(skip_serializing_if = "Option::is_none", rename = "$text")]
    value: Option<u8>
}

impl Default for ChargeParameter{
    fn default() -> Self {
        ChargeParameter{
            name: "Charge".to_owned(),
            value: None
        }
    }
}

impl ChargeParameter {
    pub fn new(value:Option<u8>) -> Self{
        ChargeParameter{
            name: "Charge".to_owned(),
            value: value
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn get_value(&self) -> &Option<u8>{
        &self.value
    }

    pub fn set_value(&mut self, value:Option<u8>){
        self.value = value;
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all ="lowercase", rename = "parameter")]
pub struct Divisor{
    #[serde(rename = "@name")]
    name: String,

    #[serde(skip_serializing_if = "Option::is_none", rename = "$text")]
    value: Option<u8>
}

impl Default for Divisor{
    fn default() -> Self{
        Divisor{
            name: "Divisor".to_owned(),
            value: None
        }
    }
}

impl Divisor {
    pub fn new(value:Option<u8>) -> Self{
        Divisor{
            name: "Divisor".to_owned(),
            value: value
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn get_value(&self) -> &Option<u8>{
        &self.value
    }

    pub fn set_value(&mut self, value:Option<u8>){
        self.value = value;
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all ="lowercase", rename = "parameter")]
pub struct UseRemainderOfKendrickMass{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "$text")]
    value: bool
}

impl Default for UseRemainderOfKendrickMass{
    fn default() -> Self {
        UseRemainderOfKendrickMass{
            name: "Use Remainder of Kendrick mass".to_owned(),
            value: false
        }
    }
}

impl UseRemainderOfKendrickMass {
    pub fn new(value:bool) -> Self{
        UseRemainderOfKendrickMass{
            name: "Use Remainder of Kendrick mass".to_owned(),
            value: value
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
}
use serde::{Serialize, Deserialize};

use crate::minimum_search_feature_resolver_module_parameters::Ms1Ms2PrecursorTolerance as MzTolerance;

#[derive(Default, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all ="lowercase", rename = "parameter")]
pub struct Validate13CIsotopeRows{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "@selected")]
    selected: bool,

    #[serde(rename = "parameter")]
    parameters: Vec<Validate13CIsotopeRowsParameter>,
}

impl Validate13CIsotopeRows {
    pub fn new() -> Self{
        Validate13CIsotopeRows{
            name: "Validate 13C isotope pattern".to_owned(),
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

    pub fn add_parameter(&mut self, parameter: Validate13CIsotopeRowsParameter){
        self.parameters.push(parameter);
    }

    pub fn get_parameter(&self, target: &str) -> Option<&Validate13CIsotopeRowsParameter> {
        self.parameters.iter().find(|param| match param {
            Validate13CIsotopeRowsParameter::MzTolerance(_) => "MzTolerance" == target,
            Validate13CIsotopeRowsParameter::MaxCharge(_) => "MaxCharge" == target,
            Validate13CIsotopeRowsParameter::EstimateMinimumCarbon(_) => "EstimateMinimumCarbon" == target,
            Validate13CIsotopeRowsParameter::Remove13C(_) => "Remove13C" == target,
            Validate13CIsotopeRowsParameter::ExcludeIsotopes(_) => "ExcludeIsotopes" == target,
        })
    }

    pub fn get_parameters_length(&self) -> usize{
        self.parameters.len()
    }

}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(untagged)]
pub enum Validate13CIsotopeRowsParameter{
    MzTolerance(MzTolerance),
    MaxCharge(MaxCharge),
    EstimateMinimumCarbon(EstimateMinimumCarbon),
    Remove13C(Remove13C),
    ExcludeIsotopes(ExcludeIsotopes),
}

impl Validate13CIsotopeRowsParameter{
    pub fn get_name(&self) -> &str{
        match self{
            Validate13CIsotopeRowsParameter::MzTolerance(_f) => _f.get_name(),
            Validate13CIsotopeRowsParameter::MaxCharge(_f) => _f.get_name(),
            Validate13CIsotopeRowsParameter::EstimateMinimumCarbon(_f) => _f.get_name(),
            Validate13CIsotopeRowsParameter::Remove13C(_f) => _f.get_name(),
            Validate13CIsotopeRowsParameter::ExcludeIsotopes(_f) => _f.get_name(),
            _ => panic!("No matching parameter")
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all ="lowercase", rename = "parameter")]
pub struct MaxCharge{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "$text")]
    value: Option<u8>
}

impl Default for MaxCharge{
    fn default() -> Self {
        MaxCharge{
            name: "Max charge".to_owned(),
            value: None,
        }
    }
}
 
impl MaxCharge{
    pub fn new(value: Option<u8>) -> Self{
        MaxCharge{
            name: "Max charge".to_owned(),
            value: value,
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
pub struct EstimateMinimumCarbon{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "$text")]
    value: bool
}

impl Default for EstimateMinimumCarbon{
    fn default() -> Self {
        EstimateMinimumCarbon{
            name: "Estimate minimum carbon".to_owned(),
            value: false,
        }
    }
}
 
impl EstimateMinimumCarbon{
    pub fn new(value: bool) -> Self{
        EstimateMinimumCarbon{
            name: "Estimate minimum carbon".to_owned(),
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
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all ="lowercase", rename = "parameter")]
pub struct Remove13C{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "$text")]
    value: bool
}

impl Default for Remove13C{
    fn default() -> Self {
        Remove13C{
            name: "Remove if 13C".to_owned(),
            value: false,
        }
    }
}
 
impl Remove13C{
    pub fn new(value: bool) -> Self{
        Remove13C{
            name: "Remove if 13C".to_owned(),
            value: value,
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

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all ="lowercase", rename = "parameter")]
pub struct ExcludeIsotopes{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "$text")]
    value: String,
}

impl Default for ExcludeIsotopes{
    fn default() -> Self{
        ExcludeIsotopes{
            name: "Exclude isotopes".to_owned(),
            value: "H,C,N,O,S".to_owned(),
        }
    }
}
 
impl ExcludeIsotopes{
    pub fn new() -> Self{
        ExcludeIsotopes{
            name: "Exclude isotopes".to_owned(),
            value: "".to_owned(),
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
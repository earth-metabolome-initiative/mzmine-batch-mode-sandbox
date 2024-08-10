use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(default, rename_all = "lowercase")]
pub struct ScanFilters {
    #[serde(rename = "@name")]
    name: String,
    
    parameters: Vec<Parameter>,
}

impl ScanFilters {
    pub fn new() -> Self{
        ScanFilters { 
            name: "Scan filters".to_owned(), 
            parameters: Vec::new()
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn add_parameter(&mut self, parameter: Parameter){
        self.parameters.push(parameter);
    }

    pub fn get_parameters_length(&self) -> usize{
        self.parameters.len()
    }

    pub fn get_parameter_value(&self, parameter:Parameter){
        // TODO
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(untagged)]
pub enum Parameter{
    ScanNumber(ScanNumber),
    BaseFilteringInteger(BaseFilteringInteger),
    RetentionTime(RetentionTime),
    Mobility(Mobility),
    MSLevelFiler(MSLevelFilter),
    ScanDefinition(ScanDefinition),
    Polarity(Polarity),
    SpectrumType(SpectrumType)
}

#[derive(Default, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(default, rename_all = "lowercase")]
pub struct ScanNumber{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "$text")]
    value: Option<u16>,
}

impl ScanNumber{
    pub fn new() -> Self{
        ScanNumber { 
            name: "Scan number".to_owned(),
            value: None 
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn set_value(&mut self, value: Option<u16>){
        self.value = value;
    }

    pub fn get_value(&self) -> Option<u16>{
        self.value
    }
}


#[derive(Default, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(default, rename_all = "lowercase")]
pub struct BaseFilteringInteger{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "$text")]
    value: Option<u16>,
}

impl BaseFilteringInteger{
    pub fn new() -> Self{
        BaseFilteringInteger { 
            name: "Base Filtering Integer".to_owned(),
            value: None 
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }
    pub fn set_value(&mut self, value: Option<u16>){
        self.value = value;
    }

    pub fn get_value(&self) -> Option<u16>{
        self.value
    }
}

#[derive(Default, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(default, rename_all = "lowercase")]
pub struct RetentionTime{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "$text")]
    value: Option<u16>,
}

impl RetentionTime{
    pub fn new() -> Self{
        RetentionTime { 
            name: "Retention time".to_owned(),
            value: None 
        }
    }
    
    pub fn get_name(&self) -> &str{
        &self.name
    }
    pub fn set_value(&mut self, value: Option<u16>){
        self.value = value;
    }

    pub fn get_value(&self) -> Option<u16>{
        self.value
    }
}


#[derive(Default, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(default, rename_all = "lowercase")]
pub struct Mobility{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "$text")]
    value: Option<u16>,
}

impl Mobility{
    pub fn new() -> Self{
        Mobility { 
            name: "Mobility".to_owned(),
            value: None 
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn set_value(&mut self, value: Option<u16>){
        self.value = value;
    }

    pub fn get_value(&self) -> Option<u16>{
        self.value
    }
}

#[derive(Default, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(default, rename_all = "lowercase")]
pub struct MSLevelFilter{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "@selected")]
    selected: bool,

    #[serde(rename = "@level")]
    level: String,

    #[serde(rename = "$text")]
    value: Option<u16>,
}

impl MSLevelFilter{
    pub fn new() -> Self{
        MSLevelFilter { 
            name: "MS level filter".to_owned(),
            selected: true,
            level: "None selected".to_owned(),
            value: None 
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn set_level(&mut self, level:String){
        self.level = level;
    }

    pub fn get_level(&self) -> &str {
        &self.level
    }

    pub fn set_value(&mut self, value: Option<u16>){
        self.value = value;
    }

    pub fn get_value(&self) -> Option<u16>{
        self.value
    }

    pub fn set_ms1(&mut self, value:Option<u16>){
        self.set_level("MS1, level = 1".to_owned());
        self.set_value(value);
    }

    pub fn set_ms2(&mut self, value:Option<u16>){
        self.set_level("MSn, level ≥ 2".to_owned());
        self.set_value(value);
    }
}


#[derive(Default, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(default, rename_all = "lowercase")]
pub struct ScanDefinition{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "$text")]
    value: Option<u16>,
}

impl ScanDefinition{
    pub fn new() -> Self{
        ScanDefinition { 
            name: "Scan definition".to_owned(),
            value: None 
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn set_value(&mut self, value: Option<u16>){
        self.value = value;
    }

    pub fn get_value(&self) -> Option<u16>{
        self.value
    }
}

#[derive(Default, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(default, rename_all = "lowercase")]
pub struct Polarity{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "$text")]
    value: String,
}

impl Polarity{
    pub fn new() -> Self{
        Polarity { 
            name: "Polarity".to_owned(),
            value: "Any".to_owned(), 
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn set_value(&mut self, value: String){
        self.value = value;
    }

    pub fn get_value(&self) -> String{
        self.value.clone()
    }
}

#[derive(Default, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(default, rename_all = "lowercase")]
pub struct SpectrumType{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "$text")]
    value: String,
}

impl SpectrumType{
    pub fn new() -> Self{
        SpectrumType { 
            name: "Spectrum type".to_owned(),
            value: "ANY".to_owned(), 
        }
    }
    
    pub fn get_name(&self) -> &str{
        &self.name
    }
    pub fn set_value(&mut self, value: String){
        self.value = value;
    }

    pub fn get_value(&self) -> String{
        self.value.clone()
    }
}
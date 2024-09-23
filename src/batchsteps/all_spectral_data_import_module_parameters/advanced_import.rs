use core::panic;
use std::collections::HashMap;

use serde::{Serialize, Deserialize};

pub enum Value {
    Single(Option<f32>),
    Vector(Vec<Option<f32>>),
}


#[derive(Default, Serialize, Deserialize, PartialEq)]
 #[serde(default, rename_all = "lowercase", rename ="parameter")]
pub struct AdvancedImport{
    #[serde(rename = "@name")]
    name: String,

     #[serde(rename = "@selected")]
    selected: bool,
 
    #[serde(rename = "parameter")]
    parameters: Vec<AdvancedImportParameters>
 }

 impl AdvancedImport {
    pub fn new() -> Self {
        AdvancedImport {
            name: "Advanced import".to_owned(),
            selected: false,
            parameters: Vec::new(),
        }
    }

    /// Generate the predifined XML template with the new values inputted
    /// The order is: \n \t - ScanFilters \n->\t CropMS1mz \n->\t MSDetectorAdvanced \n->\t DenormalizeFragmentScansTraps
    /// 
    pub fn generate(values:Vec<String>) -> Self{
        let mut generated_parameters: Vec<AdvancedImportParameters> = Vec::new();

        // Vec to associate parameter's modules/parameters to positions in the array
        let mut parameter_indexes: Vec<(String, usize)> = Vec::new();

        //parameters that are present in advanced parameter
        let strings_to_match = vec![
            "ScanFilters".to_string(),
            "CropMS1mz".to_string(),
            "MSDetectorAdvanced".to_string(),
            "DenormalizeFragmentScansTraps".to_string(),
        ];

        // Search for the modules
        for (index, string) in values.iter().enumerate() {
            if strings_to_match.contains(string) {
                parameter_indexes.push((string.clone(), index));
            }
        }

        // Create a mapping of strings to their order
            // Order has to be specified in strings_to_match -> equivalent to order in the XML [predifined]
            // Order is maintained with insertion (from Rust 1.63 and on) -> we exploit it
        let order_map: std::collections::HashMap<_, _> = strings_to_match
                        .iter()
                        .enumerate()
                        .map(|(index, string)| (string.clone(), index))
                        .collect();

        // Iterate through the keys to retrieve which parameters are selected
        for key in order_map.keys() {
            // if match, pass a sub-set of values (aka pass the values associated to said parameter)
            match key.as_str() {
                "ScanFilters" => {
                    generated_parameters.push(AdvancedImportParameters::ScanFilters(
                        ScanFilters::generate(&values[1..=*order_map.get("CropMS1mz").unwrap()]) // Adjust slice based on crop_index
                    ));
                }
                "CropMS1mz" => {
                    generated_parameters.push(AdvancedImportParameters::CropMS1mz(
                        CropMS1mz::generate(&values[*order_map.get("CropMS1mz").unwrap()+1..=*order_map.get("MSDetectorAdvanced").unwrap()])
                    ));
                }
                "MSDetectorAdvanced" => {
                    generated_parameters.push(AdvancedImportParameters::MSDetectorAdvanced(
                        MSDetectorAdvanced::generate(&values[*order_map.get("MSDetectorAdvanced").unwrap()+1..=*order_map.get("DenormalizeFragmentScansTraps").unwrap()])
                    ));
                }
                "DenormalizeFragmentScansTraps" => {
                    generated_parameters.push(AdvancedImportParameters::MSDetectorAdvanced(
                        MSDetectorAdvanced::generate(&values[*order_map.get("DenormalizeFragmentScansTraps").unwrap()+1..])
                    ));
                }
                _ => panic!("No matching parameter {}", values[0]),
            }
        }

        AdvancedImport {
            name: "Advanced import".to_owned(),
            selected: false,
            parameters: generated_parameters,
        }
    }

    pub fn get_name(&self) -> String{
        self.name.clone()
    }

    pub fn get_parameters_length(&self) -> usize{
        self.parameters.len()
    }

    pub fn add_parameter(&mut self, parameter:AdvancedImportParameters){
        self.parameters.push(parameter);
    }

    pub fn invert_selected(&mut self){
        self.selected = !self.selected;
    }

    pub fn is_selected(&self) -> bool{
        self.selected
    }

    pub fn select(&mut self) {
        self.selected = true;
    }

    pub fn deselect(&mut self){
        self.selected=false;
    }
}


 
 #[derive(Serialize, Deserialize, PartialEq)]
 #[serde(untagged)]
 pub enum AdvancedImportParameters{
    ScanFilters(ScanFilters),
    CropMS1mz(CropMS1mz),
    MSDetectorAdvanced(MSDetectorAdvanced),
    DenormalizeFragmentScansTraps(DenormalizeFragmentScansTraps),
 }


// ### ### ### ### ### ### ###     Scan Filter     ### ### ### ### ### ### ### ### ### ###

 #[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
 #[serde(default, rename_all = "lowercase", rename ="parameter")]
pub struct ScanFilters{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "@selected")]
    selected: bool,

    #[serde(rename = "parameter")]
    parameters: Vec<ScanFiltersParameters>,
 }

 impl Default for ScanFilters{
    fn default() -> Self{
        ScanFilters {
            name: "Scan filters".to_owned(),
            selected: true,
            parameters: vec![
                ScanFiltersParameters::ScanNumber(ScanNumber::new()),
                ScanFiltersParameters::BaseFilteringInteger(BaseFilteringInteger::new()),
                ScanFiltersParameters::RetentionTime(RetentionTime::new()),
                ScanFiltersParameters::Mobility(Mobility::new()),
                ScanFiltersParameters::MSLevelFilter(MSLevelFilter::default()),
                ScanFiltersParameters::ScanDefinition(ScanDefinition::new()),
                ScanFiltersParameters::Polarity(Polarity::default()),
                ScanFiltersParameters::SpectrumType(SpectrumType::default())
            ]
        }
    }
 }

impl ScanFilters {
    pub fn new() -> Self {
        ScanFilters {
            name: "Scan filters".to_owned(),
            selected: true,
            parameters: Vec::new(),
        }
    }

    /// Generate the sub parameters automatically from the parameters present passed to the method
    /// Order is: \n \t - ScanNumber \n->\t BaseFilteringInteger \n->\t RetentionTime \n->\t Mobility \n->\t MSLevelFilter \n->\t ScanDefinition \n->\t Polarity\n->\t SpectrumType
    pub fn generate(values: &[String]) -> Self{
        let mut generated_parameters = Vec::new();

        // Vec to associate parameter's modules/parameters to positions in the array
        let mut parameter_indexes: Vec<(String, usize)> = Vec::new();

        //parameters that are present in advanced parameter
        let strings_to_match = vec![
            "ScanNumber".to_string(),
            "BaseFilteringInteger".to_string(),
            "RetentionTime".to_string(),
            "Mobility".to_string(),
            "MSLevelFilter".to_string(),
            "ScanDefinition".to_string(),
            "Polarity".to_string(),
            "SpectrumType".to_string(),
        ];

        // Search for the modules
        for (index, string) in values.iter().enumerate() {
            if strings_to_match.contains(string) {
                parameter_indexes.push((string.clone(), index));
            }
        }

        // Create a mapping of strings to their order
            // Order has to be specified in strings_to_match -> equivalent to order in the XML [predifined]
            // Order is maintained with insertion (from Rust 1.63 and on) -> we exploit it
        let order_map: std::collections::HashMap<_, _> = strings_to_match
                        .iter()
                        .enumerate()
                        .map(|(index, string)| (string.clone(), index))
                        .collect();

        // Iterate through the keys to retrieve which parameters are selected
        for key in order_map.keys() {
            // if match, pass a sub-set of values (aka pass the values associated to said parameter)
            match key.as_str() {
                "ScanNumber" => {
                    generated_parameters.push(ScanFiltersParameters::ScanNumber(
                        ScanNumber::generate(&values[1..=*order_map.get("BaseFilteringInteger").unwrap()]) // Adjust slice based on crop_index
                    ));
                }
                "BaseFilteringInteger" => {
                    generated_parameters.push(ScanFiltersParameters::BaseFilteringInteger(
                        BaseFilteringInteger::generate(&values[*order_map.get("BaseFilteringInteger").unwrap()+1..=*order_map.get("RetentionTime").unwrap()])
                    ));
                }
                "RetentionTime" => {
                    generated_parameters.push(ScanFiltersParameters::RetentionTime(
                        RetentionTime::generate(&values[*order_map.get("RetentionTime").unwrap()+1..=*order_map.get("Mobility").unwrap()])
                    ));
                }
                "Mobility" => {
                    generated_parameters.push(ScanFiltersParameters::Mobility(
                        Mobility::generate(&values[*order_map.get("Mobility").unwrap()..=*order_map.get("MSLevelFilter").unwrap()])
                    ));
                }
                "MSLevelFilter" => {
                    generated_parameters.push(ScanFiltersParameters::MSLevelFilter(
                        MSLevelFilter::generate(&values[*order_map.get("MSLevelFilter").unwrap()..=*order_map.get("ScanDefinition").unwrap()])
                    ));
                }
                "ScanDefinition" => {
                    generated_parameters.push(ScanFiltersParameters::ScanDefinition(
                        ScanDefinition::generate(&values[*order_map.get("ScanDefinition").unwrap()+1..=*order_map.get("Polarity").unwrap()])
                    ));
                }
                "Polarity" => {
                    generated_parameters.push(ScanFiltersParameters::Polarity(
                        Polarity::generate(&values[*order_map.get("Polarity").unwrap()+1..=*order_map.get("SpectrumType").unwrap()])
                    ));
                }
                "SpectrumType" => {
                    generated_parameters.push(ScanFiltersParameters::SpectrumType(
                        SpectrumType::generate(&values[*order_map.get("SpectrumType").unwrap()+1..])
                    ));
                }
                _ => panic!("No matching parameter {}", values[0]),
            }
        }

        ScanFilters {
            name: "Scan filters".to_owned(),
            selected: true,
            parameters: generated_parameters,
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn get_parameters_length(&self) -> usize{
        self.parameters.len()
    }

    pub fn add_parameter(&mut self, parameter:ScanFiltersParameters){
        self.parameters.push(parameter)
    }

    pub fn invert_selected(&mut self){
        self.selected = !self.selected;
    }

    pub fn is_selected(&self) -> bool{
        self.selected
    }

    pub fn select(&mut self) {
        self.selected = true;
    }

    pub fn deselect(&mut self){
        self.selected=false;
    }

    pub fn get_parameter(&mut self, target: &str) -> &mut ScanFiltersParameters{
        for parameter in &mut self.parameters {
            match parameter {
                ScanFiltersParameters::ScanNumber(_) if target == "Scan number" => return parameter,
                ScanFiltersParameters::BaseFilteringInteger(_) if target == "Base Filtering Integer" => return parameter,
                ScanFiltersParameters::RetentionTime(_) if target == "Retention time" => return parameter,
                ScanFiltersParameters::Mobility(_) if target == "Mobility" => return parameter,
                ScanFiltersParameters::MSLevelFilter(_) if target == "MS Level Filter" => return parameter,
                ScanFiltersParameters::ScanDefinition(_) if target == "Scan definition" => return parameter,
                ScanFiltersParameters::Polarity(_) if target == "Polarity" => return parameter,
                ScanFiltersParameters::SpectrumType(_) if target == "Spectrum type" => return parameter,
                _ => continue
            }
        }
        panic!("No matching parameter {}", target)
    }
}

 #[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
 #[serde(untagged)]
pub enum ScanFiltersParameters{
    ScanNumber(ScanNumber),
    BaseFilteringInteger(BaseFilteringInteger),
    RetentionTime(RetentionTime),
    Mobility(Mobility),
    MSLevelFilter(MSLevelFilter),
    ScanDefinition(ScanDefinition),
    Polarity(Polarity),
    SpectrumType(SpectrumType),
 }

impl ScanFiltersParameters{
    pub fn new(&self) -> Self{
        match self {
            ScanFiltersParameters::ScanNumber(_f) => ScanFiltersParameters::ScanNumber(ScanNumber::new()),
            ScanFiltersParameters::BaseFilteringInteger(_f) => ScanFiltersParameters::BaseFilteringInteger(BaseFilteringInteger::new()),
            ScanFiltersParameters::RetentionTime(_f) => ScanFiltersParameters::RetentionTime(RetentionTime::new()),
            ScanFiltersParameters::Mobility(_f) => ScanFiltersParameters::Mobility(Mobility::new()),
            ScanFiltersParameters::MSLevelFilter(_f) => ScanFiltersParameters::MSLevelFilter(MSLevelFilter::new()),
            ScanFiltersParameters::ScanDefinition(_f) => ScanFiltersParameters::ScanDefinition(ScanDefinition::new()),
            ScanFiltersParameters::Polarity(_f) => ScanFiltersParameters::Polarity(Polarity::new()),
            ScanFiltersParameters::SpectrumType(_f) => ScanFiltersParameters::SpectrumType(SpectrumType::new())
        }
    }

    /// Generate the sub parameters automatically from the parameters present passed to the method
        pub fn generate(&self, values: &[String]) -> Self{
        match self {
            ScanFiltersParameters::ScanNumber(_f) => ScanFiltersParameters::ScanNumber(ScanNumber::generate(values)),
            ScanFiltersParameters::BaseFilteringInteger(_f) => ScanFiltersParameters::BaseFilteringInteger(BaseFilteringInteger::generate(values)),
            ScanFiltersParameters::RetentionTime(_f) => ScanFiltersParameters::RetentionTime(RetentionTime::generate(values)),
            ScanFiltersParameters::Mobility(_f) => ScanFiltersParameters::Mobility(Mobility::generate(values)),
            ScanFiltersParameters::MSLevelFilter(_f) => ScanFiltersParameters::MSLevelFilter(MSLevelFilter::generate(values)),
            ScanFiltersParameters::ScanDefinition(_f) => ScanFiltersParameters::ScanDefinition(ScanDefinition::generate(values)),
            ScanFiltersParameters::Polarity(_f) => ScanFiltersParameters::Polarity(Polarity::generate(values)),
            ScanFiltersParameters::SpectrumType(_f) => ScanFiltersParameters::SpectrumType(SpectrumType::generate(values))
        }
    }

    pub fn get_name(&self) -> &str{
        match self{
            ScanFiltersParameters::ScanNumber(_f) => _f.get_name(),
            ScanFiltersParameters::BaseFilteringInteger(_f) => _f.get_name(),
            ScanFiltersParameters::RetentionTime(_f) => _f.get_name(),
            ScanFiltersParameters::Mobility(_f) => _f.get_name(),
            ScanFiltersParameters::MSLevelFilter(_f) => _f.get_name(),
            ScanFiltersParameters::ScanDefinition(_f) => _f.get_name(),
            ScanFiltersParameters::Polarity(_f) => _f.get_name(),
            ScanFiltersParameters::SpectrumType(_f) => _f.get_name(),
            _ => panic!("No matching parameter")
        }
    }

    pub fn set_name(&mut self, value:&str){
        match self{
            ScanFiltersParameters::MSLevelFilter(_f) => _f.set_name(value),
            _ => panic!("No matching parameter")
        }
    }

    pub fn set_value(&mut self, value: Option<u8>){
        match self{
            ScanFiltersParameters::MSLevelFilter(_f) => _f.set_value(value),
            _ => panic!("No matching parameter")
        }
    }

    pub fn set_selected(&mut self, value:&str){
        match self{
            ScanFiltersParameters::MSLevelFilter(_f) => _f.set_selected(value),
            _ => panic!("No matching parameter")
        }
    }
}

 #[derive(Default, Serialize, Deserialize,  Clone, Debug, PartialEq)]
 #[serde(default, rename_all = "lowercase", rename = "parameter")]
pub struct ScanNumber{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "$text")]
    value: Option<u8>,
 }

 impl ScanNumber {
    pub fn new() -> Self {
        ScanNumber {
            name: "Scan number".to_owned(),
            value: None,
        }
    }

    pub fn generate(value: &[String]) -> Self{
        ScanNumber {
            name: "Scan number".to_owned(),
            value: value[0].parse::<u8>().ok(), // Parse the string to u8 and handle the result
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn set_value(&mut self, value:Option<u8>){
        self.value = value;
    }

    pub fn get_value(&self) -> &Option<u8>{
        &self.value
    }
}

 #[derive(Default, Serialize, Deserialize,  Clone, Debug, PartialEq)]
 #[serde(default, rename_all = "lowercase", rename = "parameter")]
pub struct BaseFilteringInteger{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "$text")]
    value: Option<u8>,
 }

 impl BaseFilteringInteger {
    pub fn new() -> Self {
        BaseFilteringInteger {
            name: "Base Filtering Integer".to_owned(),
            value: None,
        }
    }

    pub fn generate(value: &[String]) -> Self{
        BaseFilteringInteger {
            name: "Base Filtering Integer".to_owned(),
            value: value[0].parse::<u8>().ok(),
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn set_value(&mut self, value:Option<u8>){
        self.value = value;
    }

    pub fn get_value(&self) -> &Option<u8>{
        &self.value
    }
}

 #[derive(Default, Serialize, Deserialize,  Clone, Debug, PartialEq)]
 #[serde(default, rename_all = "lowercase", rename = "parameter")]
pub struct RetentionTime{
    #[serde(rename = "@name")]
    name: String,

    #[serde(skip_serializing_if = "Option::is_none")]
    values: Option<Vec<RetentionTimeMinMax>>,
 }

impl RetentionTime {
    pub fn new() -> Self {
        RetentionTime {
            name: "Retention time".to_owned(),
            values: None,
        }
    }

    pub fn generate(values: &[String]) -> Self{
        RetentionTime {
            name: "Retention time".to_owned(),
            values: Some(vec![
                //take the second and third element of string and convert it to u8 to create Min/Max values
                RetentionTimeMinMax::RetentionTimeMin(RetentionTimeMin::new(values[0].parse::<u8>().unwrap())),
                RetentionTimeMinMax::RetentionTimeMax(RetentionTimeMax::new(values[1].parse::<u8>().unwrap())),
            ])
        }
    }

    pub fn m_adap_default(min:u8, max:u8) -> Self{
        RetentionTime{
            name: "Retention time".to_owned(),
            values: Some(vec![
                RetentionTimeMinMax::RetentionTimeMin(RetentionTimeMin::new(min)),
                RetentionTimeMinMax::RetentionTimeMax(RetentionTimeMax::new(max)),
            ])
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

}

#[derive(Serialize, Deserialize,  Clone, Debug, PartialEq)]
#[serde(untagged)]
enum RetentionTimeMinMax{
    RetentionTimeMin(RetentionTimeMin),
    RetentionTimeMax(RetentionTimeMax)
}

#[derive(Default, Serialize, Deserialize,  Clone, Debug, PartialEq)]
#[serde(default, rename_all = "lowercase", rename = "min")]
pub struct RetentionTimeMin{
    #[serde(rename ="$text")]
    value: u8,
}

impl RetentionTimeMin{
    pub fn new(value:u8) -> Self{
        RetentionTimeMin{
            value: value,
        }
    }
}

#[derive(Default, Serialize, Deserialize,  Clone, Debug, PartialEq)]
#[serde(default, rename_all = "lowercase", rename = "max")]
pub struct RetentionTimeMax{
    #[serde(rename ="$text")]
    value: u8,
}

impl RetentionTimeMax{
    pub fn new(value:u8) -> Self{
        RetentionTimeMax{
            value: value,
        }
    }
}

 #[derive(Default, Serialize, Deserialize,  Clone, Debug, PartialEq)]
 #[serde(default, rename_all = "lowercase", rename = "parameter")]
pub struct Mobility{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "$text")]
    value: Option<u8>,
 }

 impl Mobility {
    pub fn new() -> Self {
        Mobility {
            name: "Mobility".to_owned(),
            value: None,
        }
    }

    pub fn generate(value: &[String]) -> Self{
        Mobility {
            name: "Mobility".to_owned(),
            value: value[0].parse::<u8>().ok(),
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn set_value(&mut self, value:Option<u8>){
        self.value = value;
    }

    pub fn get_value(&self) -> &Option<u8>{
        &self.value
    }
}

 #[derive(Serialize, Deserialize,  Clone, Debug, PartialEq)]
 #[serde(default, rename_all = "lowercase", rename = "parameter")]
pub struct MSLevelFilter{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "@selected")]
    selected: String,

    #[serde(rename = "$text")]
    value: Option<u8>,
 }

 impl Default for MSLevelFilter{
    fn default() -> Self{
        MSLevelFilter{
            name: "MS level filter".to_owned(),
            selected: "All MS levels".to_owned(),
            value: Some(1),
        }
    }
 }

 impl MSLevelFilter {
    pub fn new() -> Self {
        MSLevelFilter {
            name: "MS level filter".to_owned(),
            selected: "".to_owned(),
            value: None,
        }
    }

    pub fn generate(value: &[String]) -> Self{
        MSLevelFilter {
            name: "MS level filter".to_owned(),
            selected: "".to_owned(),
            value: value[0].parse::<u8>().ok(),
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn set_name(&mut self, value:&str){
        self.name = value.to_owned();
    }

    pub fn get_selected(&self) -> &str{
        &self.selected
    }

    pub fn set_selected(&mut self, selection:&str){
        self.selected = selection.to_owned();
    }

    pub fn set_value(&mut self, value: Option<u8>){
        self.value = value;
    }

    pub fn get_value(&self) -> &Option<u8>{
        &self.value
    }
}


 #[derive(Default, Serialize, Deserialize,  Clone, Debug, PartialEq)]
 #[serde(default, rename_all = "lowercase", rename = "parameter")]
 pub struct ScanDefinition{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "$text")]
    value: Option<u8>,
 }

 impl ScanDefinition {
    pub fn new() -> Self {
        ScanDefinition {
            name: "Scan definition".to_owned(),
            value: None,
        }
    }

    pub fn generate(value: &[String]) -> Self{
        ScanDefinition {
            name: "Scan definition".to_owned(),
            value: value[0].parse::<u8>().ok(),
        }
    }

    pub fn get_name(&self) -> &str{
       &self.name
    }

    pub fn set_value(&mut self, value: Option<u8>){
        self.value = value;
    }

    pub fn get_value(&self) -> &Option<u8>{
        &self.value
    }
}

 #[derive(Serialize, Deserialize,  Clone, Debug, PartialEq)]
 #[serde(default, rename_all = "lowercase", rename = "parameter")]
 
pub struct Polarity{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "$text")]
    value: String,
 }

 impl Default for Polarity{
    fn default() -> Self{
        Polarity { name: "Polarity".to_owned(), value: "Any".to_owned() }
    }
 }

 impl Polarity {
    pub fn new() -> Self {
        Polarity {
            name: "Polarity".to_owned(),
            value: "".to_owned(),
        }
    }

    pub fn generate(value:&[String]) -> Self{
        Polarity {
            name: "Polarity".to_owned(),
            value: value[0].clone(),
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn set_value(&mut self, value: &str){
        self.value = value.to_owned();
    }

    pub fn get_value(&self) -> &str{
        &self.value
    }
}

 #[derive(Serialize, Deserialize,  Clone, Debug, PartialEq)]
 #[serde(default, rename_all = "lowercase", rename = "parameter")]
pub struct SpectrumType{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "$text")]
    value: String,
 }

impl Default for SpectrumType{
    fn default() -> Self{
        SpectrumType { name: "Spectrum type".to_owned(), value: "ANY".to_owned() }
    }
}

impl SpectrumType {
    pub fn new() -> Self {
        SpectrumType {
            name: "Spectrum type".to_owned(),
            value: "".to_owned(),
        }
    }

    pub fn generate(value: &[String]) -> Self{
        SpectrumType {
            name: "Spectrum type".to_owned(),
            value: value[0].clone(),
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn set_value(&mut self, value: &str){
        self.value = value.to_owned();
    }

    pub fn get_value(&self) -> &str{
        &self.value
    }
}


// Package of Crop MS1 m/z and its subparameters

#[derive(Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase", rename = "parameter")]
pub struct CropMS1mz{
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@selected")]
    selected: bool,

    #[serde(rename = "$text")]
    value: Option<u8>
}

impl CropMS1mz {
    pub fn new() -> Self {
        CropMS1mz {
            name: "Crop MS1 m/z".to_owned(),
            selected: false,
            value: None
        }
    }

    pub fn generate(values: &[String]) -> Self{
        CropMS1mz {
            name: "Crop MS1 m/z".to_owned(),
            selected: values[0].parse::<bool>().unwrap(),
            value: values[1].parse::<u8>().ok()
        }
    }

    pub fn get_name(&self) -> String{
        self.name.clone()
    }

    pub fn invert_selected(&mut self){
        self.selected = !self.selected;
    }

    pub fn is_selected(&self) -> bool{
        self.selected
    }

    pub fn select(&mut self) {
        self.selected = true;
    }

    pub fn deselect(&mut self){
        self.selected=false;
    }

    pub fn get_value(&self) -> Option<u8>{
        self.value
    }

    pub fn set_value(&mut self, value:Option<u8>){
        self.value = value;
    }
}



#[derive(Serialize, Deserialize, Clone, Debug,PartialEq)]
#[serde(default, rename_all = "lowercase", rename ="parameter")]
pub struct MSDetectorAdvanced{
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@selected", skip_serializing_if = "Option::is_none")]
    selected: Option<bool>,
    #[serde(rename = "@selected_item")]
    selected_item: String,

    #[serde(rename = "module")]
    modules: Vec<MSDetectorAdvancedModules>,
}

impl Default for MSDetectorAdvanced{
    fn default() -> Self{
        MSDetectorAdvanced{
            name: "".to_owned(),
            selected: Some(true),
            selected_item: "Factor of lowest signal".to_owned(),
            modules: vec![
                MSDetectorAdvancedModules::FactorOfLowestSignal(FactorOfLowestSignal::new()),
                MSDetectorAdvancedModules::Auto(Auto::default()),
                MSDetectorAdvancedModules::Centroid(Centroid::new()),
                MSDetectorAdvancedModules::ExactMass(ExactMass::new()),
                MSDetectorAdvancedModules::LocalMaxima(LocalMaxima::new()),
                MSDetectorAdvancedModules::RecursiveThreshold(RecursiveThreshold::default()),
                MSDetectorAdvancedModules::WaveletTransform(WaveletTransform::default())
            ]
        }
    }
}

impl MSDetectorAdvanced {
    pub fn new() -> Self {
        MSDetectorAdvanced {
            name: "".to_owned(),
            selected: Some(true),
            selected_item: "Factor of lowest signal".to_owned(),
            modules: Vec::new(),
        }
    }

    pub fn generate(values: &[String]) -> Self{
        let mut generated_modules: Vec<MSDetectorAdvancedModules> = Vec::new();

        // Vec to associate parameter's modules/parameters to positions in the array
        let mut parameter_indexes: Vec<(String, usize)> = Vec::new();

        //parameters that are present in advanced parameter
        let strings_to_match = vec![
            "ScanNumber".to_string(),
            "BaseFilteringInteger".to_string(),
            "RetentionTime".to_string(),
            "Mobility".to_string(),
            "MSLevelFilter".to_string(),
            "ScanDefinition".to_string(),
            "Polarity".to_string(),
            "SpectrumType".to_string(),
        ];

        // Search for the modules
        for (index, string) in values.iter().enumerate() {
            if strings_to_match.contains(string) {
                parameter_indexes.push((string.clone(), index));
            }
        }

        // Create a mapping of strings to their order
            // Order has to be specified in strings_to_match -> equivalent to order in the XML [predifined]
            // Order is maintained with insertion (from Rust 1.63 and on) -> we exploit it
        let order_map: std::collections::HashMap<_, _> = strings_to_match
                        .iter()
                        .enumerate()
                        .map(|(index, string)| (string.clone(), index))
                        .collect();

        // Iterate through the keys to retrieve which parameters are selected
        for key in order_map.keys() {
            // if match, pass a sub-set of values (aka pass the values associated to said parameter)
            match key.as_str() {
                "FactorOfLowestSignal" => {
                    generated_modules.push(MSDetectorAdvancedModules::FactorOfLowestSignal(
                        FactorOfLowestSignal::generate(&values[1..=*order_map.get("Auto").unwrap()]) // Adjust slice based on crop_index
                    ));
                }
                "Auto" => {
                    generated_modules.push(MSDetectorAdvancedModules::Auto(
                        Auto::generate(&values[*order_map.get("Auto").unwrap()+1..=*order_map.get("Centroid").unwrap()])
                    ));
                }
                "Centroid" => {
                    generated_modules.push(MSDetectorAdvancedModules::Centroid(
                        Centroid::generate(&values[*order_map.get("Centroid").unwrap()+1..=*order_map.get("ExactMass").unwrap()])
                    ));
                }
                "ExactMass" => {
                    generated_modules.push(MSDetectorAdvancedModules::ExactMass(
                        ExactMass::generate(&values[*order_map.get("ExactMass").unwrap()..=*order_map.get("LocalMaxima").unwrap()])
                    ));
                }
                "LocalMaxima" => {
                    generated_modules.push(MSDetectorAdvancedModules::LocalMaxima(
                        LocalMaxima::generate(&values[*order_map.get("LocalMaxima").unwrap()..=*order_map.get("RecursiveThreshold").unwrap()])
                    ));
                }
                "RecursiveThreshold" => {
                    generated_modules.push(MSDetectorAdvancedModules::RecursiveThreshold(
                        RecursiveThreshold::generate(&values[*order_map.get("RecursiveThreshold").unwrap()+1..=*order_map.get("WaveletTransform").unwrap()])
                    ));
                }
                "WaveletTransform" => {
                    generated_modules.push(MSDetectorAdvancedModules::WaveletTransform(
                        WaveletTransform::generate(&values[*order_map.get("WaveletTransform").unwrap()+1..])
                    ));
                }
                _ => panic!("No matching parameter {}", values[0]),
            }
        }

        MSDetectorAdvanced {
            name: "".to_owned(),
            selected: values[0].parse::<bool>().ok(),
            selected_item: "Factor of lowest signal".to_owned(),
            modules: Vec::new(),
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn set_name(&mut self, name:&str){
        self.name = name.to_owned();
    } 

    pub fn invert_selected(&mut self){
        self.selected = Some(!self.selected.unwrap());
    }

    pub fn is_selected(&self) -> bool{
        self.selected.unwrap()
    }

    pub fn select(&mut self) {
        self.selected = Some(true);
    }

    pub fn deselect(&mut self){
        self.selected = Some(false);
    }

    pub fn not_selected(&mut self){
        self.selected = None;
    }

    pub fn get_selected_item(&self) -> &str{
        &self.selected_item
    }

    pub fn set_selected_item(&mut self, item:String){
        self.selected_item = item;
    }

    pub fn set_ms1(&mut self, value: Option<f32>) {
        self.name = "MS1 detector (Advanced)".to_owned();
        for module in &mut self.modules {
            match module {
                MSDetectorAdvancedModules::FactorOfLowestSignal(this) => {
                    this.set_value(value);
                }
                _ => {}
            }
        }
    }

    pub fn set_ms2(&mut self, value: Option<f32>) {
        self.name = "MS2 detector (Advanced)".to_owned();
        for module in &mut self.modules {
            match module {
                MSDetectorAdvancedModules::FactorOfLowestSignal(this) => {
                    this.set_value(value);
                }
                _ => {}
            }
        }
    }

    pub fn add_module(&mut self, module:MSDetectorAdvancedModules){
        self.modules.push(module);
    }

    pub fn get_module_length(&self) -> usize{
        self.modules.len()
    }

    pub fn get_module(&mut self, target:&str) -> &mut MSDetectorAdvancedModules{
        for module in &mut self.modules{
            match module{
                MSDetectorAdvancedModules::FactorOfLowestSignal(_f) if target == "FactorOfLowestSignal" => return module,
                MSDetectorAdvancedModules::Auto(_f) if target == "Auto" => return module,
                MSDetectorAdvancedModules::Centroid(_f) if target == "Centroid" => return module,
                MSDetectorAdvancedModules::ExactMass(_f) if target == "ExactMass" => return module,
                MSDetectorAdvancedModules::LocalMaxima(_f) if target == "LocalMaxima" => return module,
                MSDetectorAdvancedModules::RecursiveThreshold(_f) if target == "RecursiveThreshold" => return module,
                MSDetectorAdvancedModules::WaveletTransform(_f) if target == "WaveletTransform" => return module,
                _ => continue,
            }
        } 
        panic!("No matching parameter {}", target)
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(untagged)]
pub enum MSDetectorAdvancedModules {
    FactorOfLowestSignal(FactorOfLowestSignal),
    Auto(Auto),
    Centroid(Centroid),
    ExactMass(ExactMass),
    LocalMaxima(LocalMaxima),
    RecursiveThreshold(RecursiveThreshold),
    WaveletTransform(WaveletTransform)
}

impl MSDetectorAdvancedModules {
    pub fn get_name(&self) -> &str{
        match self {
            MSDetectorAdvancedModules::FactorOfLowestSignal(f) => f.get_name(),
            MSDetectorAdvancedModules::Auto(f) => f.get_name(),
            MSDetectorAdvancedModules::Centroid(f) => f.get_name(),
            MSDetectorAdvancedModules::ExactMass(f) => f.get_name(),
            MSDetectorAdvancedModules::LocalMaxima(f) => f.get_name(),
            _ => panic!("No matching parameter found"),
        }
    }

    pub fn get_value(&self) -> Result<&Option<f32>, &'static str>{
        match self {
            MSDetectorAdvancedModules::FactorOfLowestSignal(f) => Ok(f.get_value()),
            MSDetectorAdvancedModules::Auto(f) => Ok(f.get_value()),
            MSDetectorAdvancedModules::Centroid(f) => Ok(f.get_value()),
            MSDetectorAdvancedModules::ExactMass(f) => Ok(f.get_value()),
            MSDetectorAdvancedModules::LocalMaxima(f) => Ok(f.get_value()),
            _ => Err("Module not found"),
        }
    }

    pub fn set_value(&mut self, value:Option<f32>){
        match self {
            MSDetectorAdvancedModules::FactorOfLowestSignal(f) => f.set_value(value),
            MSDetectorAdvancedModules::Auto(f) => f.set_value(value),
            MSDetectorAdvancedModules::Centroid(f) => f.set_value(value),
            MSDetectorAdvancedModules::ExactMass(f) => f.set_value(value),
            MSDetectorAdvancedModules::LocalMaxima(f) => f.set_value(value),
            _ => panic!("No matching parameter found"),
        }
    }
}


#[derive(Default, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(default, rename_all = "lowercase", rename ="module")]
pub struct FactorOfLowestSignal{
    #[serde(rename = "@name")]
    name: String,

    parameter: ParameterFactorOfLowestSignal,
}

impl FactorOfLowestSignal{
    pub fn new() -> Self{
        FactorOfLowestSignal{
            name: "Factor of lowest signal".to_owned(),
            parameter: ParameterFactorOfLowestSignal::new()
        }
    }

    pub fn generate(value: &[String]) -> Self{
        FactorOfLowestSignal{
            name: "Factor of lowest signal".to_owned(),
            parameter: ParameterFactorOfLowestSignal::generate(value.get(0).and_then(|v| v.parse::<f32>().ok()))
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn set_value(&mut self, value:Option<f32>){
        self.parameter.set_value(value);
    }

    pub fn get_value(& self) -> &Option<f32>{
        self.parameter.get_value()
    }

    pub fn set_parameter(&mut self, parameter:ParameterFactorOfLowestSignal){
        self.parameter = parameter;
    }
}

#[derive(Default, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(default, rename_all = "lowercase", rename ="parameter")]
pub struct ParameterFactorOfLowestSignal{
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "$text")]
    value: Option<f32>,
}

impl ParameterFactorOfLowestSignal {
    pub fn new() -> Self{
        ParameterFactorOfLowestSignal{
            name: "Noise factor".to_owned(),
            value: None,
        }
    }

    pub fn generate(value: Option<f32>) -> Self{
        ParameterFactorOfLowestSignal{
            name: "Noise factor".to_owned(),
            value: value,
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn set_value(&mut self, value: Option<f32>){
        self.value = value;
    }

    pub fn get_value(& self) -> &Option<f32>{
        &self.value
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(default, rename_all = "lowercase", rename ="module")]
pub struct Auto{
    #[serde(rename = "@name")]
    name: String,

    parameter: ParameterAuto,
}

impl Default for Auto{
    fn default() -> Self{
        Auto { 
            name: "Auto".to_owned(), 
            parameter: ParameterAuto::default() 
        }
    }
}

impl Auto{
    pub fn new() -> Self{
        Auto { 
            name: "Auto".to_owned(), 
            parameter: ParameterAuto::new() 
        }
    }

    pub fn generate(value:&[String]) -> Self{
        Auto { 
            name: "Auto".to_owned(), 
            parameter: ParameterAuto::generate(value.get(0).and_then(|v| v.parse::<f32>().ok()))
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn get_value(&self) -> &Option<f32>{
        & self.parameter.get_value()
    }

    pub fn set_value(&mut self, value:Option<f32>){
        self.parameter.set_value(value);
    }

    pub fn set_parameter(&mut self, parameter:ParameterAuto){
        self.parameter = parameter;
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(default, rename_all = "lowercase", rename ="parameter")]
pub struct ParameterAuto{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "$text")]
    value: Option<f32>,
}

impl Default for ParameterAuto{
    fn default() -> Self{
        ParameterAuto{
            name: "Noise level".to_owned(),
            value: Some(1000.0),
        }
    }
}


impl ParameterAuto{
    pub fn new() -> Self{
        ParameterAuto{
            name: "Noise level".to_owned(),
            value: None,
        }
    }

    pub fn generate(value: Option<f32>) -> Self{
        ParameterAuto{
            name: "Noise level".to_owned(),
            value: value,
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    // just set it pub for testing but imo shouldn't
    pub fn get_value(& self) -> &Option<f32>{
        &self.value
    }
    // just set it pub for testing but imo shouldn't 
    pub fn set_value(&mut self, value:Option<f32>){
        self.value = value;
    }
}

#[derive(Default, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(default, rename_all = "lowercase", rename ="module")]
pub struct Centroid{
    #[serde(rename = "@name")]
    name: String,

    parameter: ParameterCentroid,
}

impl Centroid{
    pub fn new() -> Self{
        Centroid{
            name: "Centroid".to_owned(),
            parameter: ParameterCentroid::new()
        }
    }

    pub fn generate(value: &[String]) -> Self{
        Centroid{
            name: "Centroid".to_owned(),
            parameter: ParameterCentroid::generate(value.get(0).and_then(|v| v.parse::<f32>().ok()))
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn set_value(&mut self, value:Option<f32>){
        self.parameter.set_value(value);
    }

    pub fn get_value(&self) -> &Option<f32>{
        self.parameter.get_value()
    }

    pub fn set_parameter(&mut self, parameter: ParameterCentroid){
        self.parameter = parameter;
    }
}

#[derive(Default, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(default, rename_all = "lowercase", rename ="parameter")]
pub struct ParameterCentroid{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "$text")]
    value: Option<f32>,
}

impl ParameterCentroid{
    pub fn new() -> Self{
        ParameterCentroid{
            name: "Noise level".to_owned(),
            value: None,
        }
    }

    pub fn generate(value: Option<f32>) -> Self{
        ParameterCentroid{
            name: "Noise level".to_owned(),
            value: value,
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

#[derive(Default, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(default, rename_all = "lowercase", rename ="module")]
pub struct ExactMass{
    #[serde(rename = "@name")]
    name: String,

    parameter: ParameterExactMass,
}

impl ExactMass{
    pub fn new() -> Self{
        ExactMass{
            name: "Exact mass".to_owned(),
            parameter: ParameterExactMass::new(),
        }
    }

    pub fn generate(value: &[String]) -> Self{
        ExactMass{
            name: "Exact mass".to_owned(),
            parameter: ParameterExactMass::generate(value.get(0).and_then(|v| v.parse::<f32>().ok())),
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn get_value(&self) -> &Option<f32>{
        self.parameter.get_value()
    }

    pub fn set_value(&mut self, value:Option<f32>){
        self.parameter.set_value(value);
    }

    pub fn set_parameter(&mut self, parameter:ParameterExactMass){
        self.parameter = parameter;
    }
}

#[derive(Default, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(default, rename_all = "lowercase", rename ="parameter")]
pub struct ParameterExactMass{
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "$text")]
    value: Option<f32>, 
}

impl ParameterExactMass{
    pub fn new() -> Self{
        ParameterExactMass{
            name: "Noise level".to_owned(),
            value: None,
        }
    }

    pub fn generate(value: Option<f32>) -> Self{
        ParameterExactMass{
            name: "Noise level".to_owned(),
            value: value,
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

#[derive(Default, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(default, rename_all = "lowercase", rename ="module")]
pub struct LocalMaxima{
    #[serde(rename = "@name")]
    name: String,

    parameter: ParameterLocalMaxima,
}

impl LocalMaxima{
    pub fn new() -> Self{
        LocalMaxima{
            name: "Local maxima".to_owned(),
            parameter: ParameterLocalMaxima::new(),
        }
    }

    pub fn generate(value:&[String]) -> Self{
        LocalMaxima{
            name: "Local maxima".to_owned(),
            parameter: ParameterLocalMaxima::generate(value.get(0).and_then(|v| v.parse::<f32>().ok())),
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn get_value(&self) -> &Option<f32>{
        self.parameter.get_value()
    }

    pub fn set_value(&mut self, value:Option<f32>){
        self.parameter.set_value(value);
    }

    pub fn set_parameter(&mut self, parameter:ParameterLocalMaxima){
        self.parameter = parameter;
    }
}

#[derive(Default, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(default, rename_all = "lowercase", rename ="parameter")]
pub struct ParameterLocalMaxima{
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "$text")]
    value: Option<f32>, 
}
impl ParameterLocalMaxima{
    pub fn new() -> Self{
        ParameterLocalMaxima{
            name: "Noise level".to_owned(),
            value: None
        }
    }

    pub fn generate(value: Option<f32>) -> Self{
        ParameterLocalMaxima{
            name: "Noise level".to_owned(),
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


#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(default, rename_all = "lowercase", rename = "module")]
pub struct RecursiveThreshold{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename ="parameter")]
    parameters: Vec<RecursiveThresholdParameters>,
}

impl Default for RecursiveThreshold{
    fn default() -> Self{
        RecursiveThreshold{
            name: "Recursive threshold".to_owned(),
            parameters: vec![
                RecursiveThresholdParameters::RTNoiseLevel(RTNoiseLevel::new()),
                RecursiveThresholdParameters::MinMZPeakWidth(MinMZPeakWidth::new()),
                RecursiveThresholdParameters::MaxMZPeakWidth(MaxMZPeakWidth::new())
            ],
        }
    }
}

impl RecursiveThreshold{
    pub fn new() -> Self{
        RecursiveThreshold{
            name: "Recursive threshold".to_owned(),
            parameters: Vec::new(),
        }
    }

    /// Generate the object following the order of the elements listed
    /// Order is: \n->\t RTNoiseLevel \n->\t MinMZPeakWidth \n->\t MaxMZPeakWidth
    pub fn generate(values: &[String]) -> Self{
        RecursiveThreshold{
            name: "Recursive threshold".to_owned(),
            parameters: vec![
                RecursiveThresholdParameters::RTNoiseLevel(RTNoiseLevel::generate(values.get(0).and_then(|v| v.parse::<f32>().ok()))),
                RecursiveThresholdParameters::MinMZPeakWidth(MinMZPeakWidth::generate(values.get(1).and_then(|v| v.parse::<f32>().ok()))),
                RecursiveThresholdParameters::MaxMZPeakWidth(MaxMZPeakWidth::generate(values.get(2).and_then(|v| v.parse::<f32>().ok())))
            ],
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn add_parameter(&mut self, parameter: RecursiveThresholdParameters){
        self.parameters.push(parameter);
    }   

    pub fn paramater_length(&self) -> usize{
        self.parameters.len()
    }

    pub fn set_parameter_value(&mut self, target: &str, value: Option<f32>){
        for param in &mut self.parameters {
            match param {
                RecursiveThresholdParameters::RTNoiseLevel(rt) if target == "RTNoiseLevel" => return rt.set_value(value),
                RecursiveThresholdParameters::MinMZPeakWidth(min_mz) if target == "MinMZPeakWidth" => return min_mz.set_value(value),
                RecursiveThresholdParameters::MaxMZPeakWidth(max_mz) if target == "MaxMZPeakWidth" => return max_mz.set_value(value),
                _ => continue,
            }
        }
    }

    pub fn get_parameter_value(&self, target: &str) -> &Option<f32>{
        for param in &self.parameters {
            match param {
                RecursiveThresholdParameters::RTNoiseLevel(rt) if target == "RTNoiseLevel" => return rt.get_value(),
                RecursiveThresholdParameters::MinMZPeakWidth(min_mz) if target == "MinMZPeakWidth" => return min_mz.get_value(),
                RecursiveThresholdParameters::MaxMZPeakWidth(max_mz) if target == "MaxMZPeakWidth" => return max_mz.get_value(),
                _ => continue,
            }
        }
        &None
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(untagged)]
pub enum RecursiveThresholdParameters{
    RTNoiseLevel(RTNoiseLevel),
    MinMZPeakWidth(MinMZPeakWidth),
    MaxMZPeakWidth(MaxMZPeakWidth),
}

impl RecursiveThresholdParameters{
    pub fn new(&self) -> RecursiveThresholdParameters{
        match self {
            RecursiveThresholdParameters::RTNoiseLevel(_f) => RecursiveThresholdParameters::RTNoiseLevel(RTNoiseLevel::new()),
            RecursiveThresholdParameters::MinMZPeakWidth(_f) => RecursiveThresholdParameters::MinMZPeakWidth(MinMZPeakWidth::new()),
            RecursiveThresholdParameters::MaxMZPeakWidth(_f) => RecursiveThresholdParameters::MaxMZPeakWidth(MaxMZPeakWidth::new())
        }
    }

    pub fn generate(&self, value: Option<f32>) -> RecursiveThresholdParameters{
        match self {
            RecursiveThresholdParameters::RTNoiseLevel(_f) => RecursiveThresholdParameters::RTNoiseLevel(RTNoiseLevel::generate(value)),
            RecursiveThresholdParameters::MinMZPeakWidth(_f) => RecursiveThresholdParameters::MinMZPeakWidth(MinMZPeakWidth::generate(value)),
            RecursiveThresholdParameters::MaxMZPeakWidth(_f) => RecursiveThresholdParameters::MaxMZPeakWidth(MaxMZPeakWidth::generate(value))
        }
    }
    
    pub fn get_value(&self) -> &Option<f32>{
        match self{
            RecursiveThresholdParameters::RTNoiseLevel(f) => f.get_value(),
            RecursiveThresholdParameters::MinMZPeakWidth(f) => f.get_value(),
            RecursiveThresholdParameters::MaxMZPeakWidth(f) => f.get_value(),
        }
    }

    pub fn set_value(&mut self, value: Option<f32>){
        match self{
            RecursiveThresholdParameters::RTNoiseLevel(f) => f.set_value(value),
            RecursiveThresholdParameters::MinMZPeakWidth(f) => f.set_value(value),
            RecursiveThresholdParameters::MaxMZPeakWidth(f) => f.set_value(value),
        }
    }
}

#[derive(Default, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(default, rename_all = "lowercase", rename = "parameter")]
pub struct RTNoiseLevel{
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "$text")]
    value: Option<f32>
}

impl RTNoiseLevel{
    pub fn new() -> Self{
        RTNoiseLevel{
            name: "Noise level".to_owned(),
            value: None,
        }
    }

    pub fn generate(value: Option<f32>) -> Self{
        RTNoiseLevel{
            name: "Noise level".to_owned(),
            value: value,
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn set_value(&mut self, value: Option<f32>){
        self.value = value;
    }

    pub fn get_value(&self) -> &Option<f32>{
        &self.value
    }
}

#[derive(Default, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(default, rename_all = "lowercase", rename = "parameter")]
pub struct MinMZPeakWidth{
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "$text")]
    value: Option<f32>
}

impl MinMZPeakWidth{
    pub fn new() -> Self {
        MinMZPeakWidth{
            name: "Min m/z peak width".to_owned(),
            value: None,
        }
    }

    pub fn generate(value: Option<f32>) -> Self{
        MinMZPeakWidth{
            name: "Min m/z peak width".to_owned(),
            value: value,
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn set_value(&mut self, value: Option<f32>){
        self.value = value;
    }

    pub fn get_value(&self) -> &Option<f32>{
        &self.value
    }
}

#[derive(Default, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(default, rename_all = "lowercase", rename = "parameter")]
pub struct MaxMZPeakWidth{
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "$text")]
    value: Option<f32>
}

impl MaxMZPeakWidth {
    pub fn new() -> Self {
        MaxMZPeakWidth {
            name: "Max m/z peak width".to_owned(),
            value: None,
        }
    }

    pub fn generate(value: Option<f32>) -> Self{
        MaxMZPeakWidth {
            name: "Max m/z peak width".to_owned(),
            value: value,
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn set_value(&mut self, value:Option<f32>){
        self.value = value;
    }
    
    pub fn get_value(&self) -> &Option<f32>{
        &self.value
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(default, rename_all = "lowercase", rename="module")]
pub struct WaveletTransform{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename ="parameter")]
    parameters: Vec<WaveletTransformParameters>,
}

impl Default for WaveletTransform{
    fn default() -> Self {
        WaveletTransform {
            name: "Wavelet transform".to_owned(),
            parameters: vec![
                WaveletTransformParameters::WTNoiseLevel(WTNoiseLevel::new()),
                WaveletTransformParameters::ScaleLevel(ScaleLevel::new()),
                WaveletTransformParameters::WaveletWindowSize(WaveletWindowSize::new())
            ]
        }
    }
}

impl WaveletTransform {
    pub fn new() -> Self {
        WaveletTransform {
            name: "Wavelet transform".to_owned(),
            parameters: Vec::new(),
        }
    }

    pub fn generate(values: &[String]) -> Self {
        let mut generated_parameters: Vec<WaveletTransformParameters> = Vec::new();

        // Vec to associate parameter's modules/parameters to positions in the array
        let mut parameter_indexes: Vec<(String, usize)> = Vec::new();

        // Parameters that are present in advanced parameter
        let strings_to_match = vec![
            "WTNoiseLevel".to_string(),
            "ScaleLevel".to_string(),
            "WaveletWindowSize".to_string(),
        ];

        // Search for the modules
        for (index, string) in values.iter().enumerate() {
            if strings_to_match.contains(string) {
                parameter_indexes.push((string.clone(), index));
            }
        }

        // Create a mapping of strings to their order
        // Order has to be specified in strings_to_match -> equivalent to order in the XML [predefined]
        // Order is maintained with insertion (from Rust 1.63 and on) -> we exploit it
        let order_map: std::collections::HashMap<_, _> = strings_to_match
            .iter()
            .enumerate()
            .map(|(index, string)| (string.clone(), index))
            .collect();

        // Iterate through the keys to retrieve which parameters are selected
        for key in order_map.keys() {
            // If match, pass a sub-set of values (aka pass the values associated to said parameter)
            match key.as_str() {
                "WTNoiseLevel" => {
                    generated_parameters.push(WaveletTransformParameters::WTNoiseLevel(
                        WTNoiseLevel::generate(values.get(0).and_then(|v| v.parse::<f32>().ok()))
                    ));
                }
                "ScaleLevel" => {
                    generated_parameters.push(WaveletTransformParameters::ScaleLevel(
                        ScaleLevel::generate(values.get(1).and_then(|v| v.parse::<f32>().ok()))
                    ));
                }
                "WaveletWindowSize" => {
                    generated_parameters.push(WaveletTransformParameters::WaveletWindowSize(
                        WaveletWindowSize::generate(values.get(2).and_then(|v| v.parse::<f32>().ok()))
                    ));
                }
                _ => panic!("No matching key {} found", key),
            }
        }
        

        WaveletTransform {
            name: "Wavelet transform".to_owned(),
            parameters: generated_parameters,
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn add_parameter(&mut self, parameter: WaveletTransformParameters) {
        self.parameters.push(parameter);
    }

    pub fn parameters_length(&self) -> usize {
        self.parameters.len()
    }

    pub fn set_parameter_value(&mut self, target: &str, value: Option<f32>) {
        for param in &mut self.parameters {
            match param {
                WaveletTransformParameters::WTNoiseLevel(f) if target == "WTNoiseLevel" => {
                    return f.set_value(value);
                }
                WaveletTransformParameters::ScaleLevel(f) if target == "ScaleLevel" => {
                    return f.set_value(value);
                }
                WaveletTransformParameters::WaveletWindowSize(f) if target == "WaveletWindowSize" => {
                    return f.set_value(value);
                }
                _ => continue,
            }
        }
    }

    pub fn get_parameter_value(&self, target: &str) -> &Option<f32> {
        for param in &self.parameters {
            match param {
                WaveletTransformParameters::WTNoiseLevel(f) if target == "WTNoiseLevel" => {
                    return f.get_value();
                }
                WaveletTransformParameters::ScaleLevel(f) if target == "ScaleLevel" => {
                    return f.get_value();
                }
                WaveletTransformParameters::WaveletWindowSize(f) if target == "WaveletWindowSize" => {
                    return f.get_value();
                }
                _ => continue,
            }
        }
        &None
    }
}


#[derive(Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(untagged)]
pub enum WaveletTransformParameters{
    WTNoiseLevel(WTNoiseLevel),
    ScaleLevel(ScaleLevel),
    WaveletWindowSize(WaveletWindowSize),
}

impl WaveletTransformParameters{
    /// Generate the object but caution, it needs all of the three to set their parameters
    pub fn generate(&self, values: &[String]) -> Self{
        assert_eq!(values.len(), 3, "Not all parameters are inputted");
        match self{
            WaveletTransformParameters::WTNoiseLevel(_f) => WaveletTransformParameters::WTNoiseLevel(WTNoiseLevel::generate(values.get(0).and_then(|v| v.parse::<f32>().ok()))),
            WaveletTransformParameters::ScaleLevel(_f) => WaveletTransformParameters::ScaleLevel(ScaleLevel::generate(values.get(1).and_then(|v| v.parse::<f32>().ok()))),
            WaveletTransformParameters::WaveletWindowSize(_f) => WaveletTransformParameters::WaveletWindowSize(WaveletWindowSize::generate(values.get(2).and_then(|v| v.parse::<f32>().ok()))),
        }
    }

    pub fn get_value(&self) -> &Option<f32> {
        match self{
            WaveletTransformParameters::WTNoiseLevel(f) => f.get_value(),
            WaveletTransformParameters::ScaleLevel(f) => f.get_value(),
            WaveletTransformParameters::WaveletWindowSize(f) => f.get_value(),
        }
    }

    pub fn set_value(&mut self, value: Option<f32>){
        match self{
            WaveletTransformParameters::WTNoiseLevel(f) => f.set_value(value),
            WaveletTransformParameters::ScaleLevel(f) => f.set_value(value),
            WaveletTransformParameters::WaveletWindowSize(f) => f.set_value(value),
        }
    }
}

#[derive(Default, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(default, rename_all = "lowercase", rename="parameter")]
pub struct WTNoiseLevel{
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "$text")]
    value: Option<f32>
}

impl WTNoiseLevel {
    pub fn new() -> Self {
        WTNoiseLevel {
            name: "Noise level".to_owned(),
            value: None,
        }
    }

    pub fn generate(value: Option<f32>) -> Self{
        WTNoiseLevel {
            name: "Noise level".to_owned(),
            value: value,
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

#[derive(Default, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(default, rename_all = "lowercase", rename="parameter")]
pub struct ScaleLevel{
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "$text")]
    value: Option<f32>
}

impl ScaleLevel {
    pub fn new() -> Self {
        ScaleLevel {
            name: "Scale level".to_owned(),
            value: None,
        }
    }

    pub fn generate(value: Option<f32>) -> Self{
        ScaleLevel {
            name: "Scale level".to_owned(),
            value: value,
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

#[derive(Default, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(default, rename_all = "lowercase", rename="parameter")]
pub struct WaveletWindowSize{
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "$text")]
    value: Option<f32>
}

impl WaveletWindowSize {
    pub fn new() -> Self {
        WaveletWindowSize {
            name: "Wavelet window size (%)".to_owned(),
            value: None,
        }
    }

    pub fn generate(value: Option<f32>) -> Self{
        WaveletWindowSize {
            name: "Wavelet window size (%)".to_owned(),
            value: value,
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

#[derive(Default, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(default, rename_all = "lowercase", rename="parameter")]
pub struct DenormalizeFragmentScansTraps{
    #[serde(rename = "@name")]
    name: String,
    
    #[serde(rename = "$text")]
    value: bool
}

impl DenormalizeFragmentScansTraps{
    pub fn new() -> Self{
        DenormalizeFragmentScansTraps{
            name: "Denormalize fragment scans (traps)".to_owned(),
            value: true
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn get_value(&self) -> &bool{
        &self.value
    }

    pub fn set_value(&mut self, value: bool){
        self.value = value;
    }

    pub fn invert_value(&mut self){
        self.value = !self.value;
    }

    pub fn set_true(&mut self) {
        self.value = true;
    }

    pub fn set_false(&mut self){
        self.value = false;
    }
}
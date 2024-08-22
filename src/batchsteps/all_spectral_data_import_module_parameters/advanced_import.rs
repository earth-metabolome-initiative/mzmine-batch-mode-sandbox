use core::panic;

use serde::{Serialize, Deserialize};
use crate::xml_serialization::*;

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

    pub fn get_parameter(&mut self, target: &str) -> Option<&mut ScanFiltersParameters> {
        for parameter in &mut self.parameters {
            match parameter {
                ScanFiltersParameters::ScanNumber(_) if target == "Scan number" => return Some(parameter),
                ScanFiltersParameters::BaseFilteringInteger(_) if target == "Base Filtering Integer" => return Some(parameter),
                ScanFiltersParameters::RetentionTime(_) if target == "Retention time" => return Some(parameter),
                ScanFiltersParameters::Mobility(_) if target == "Mobility" => return Some(parameter),
                ScanFiltersParameters::MSLevelFilter(_) if target == "MS Level Filter" => return Some(parameter),
                ScanFiltersParameters::ScanDefinition(_) if target == "Scan definition" => return Some(parameter),
                ScanFiltersParameters::Polarity(_) if target == "Polarity" => return Some(parameter),
                ScanFiltersParameters::SpectrumType(_) if target == "Spectrum type" => return Some(parameter),
                _ => {}
            }
        }
        None
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

    #[serde(rename = "$text")]
    value: Option<u8>,
 }

impl RetentionTime {
    pub fn new() -> Self {
        RetentionTime {
            name: "Retention time".to_owned(),
            value: None,
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

    // pub fn get_parameter(&mut self, target: &str) -> &mut Parameter {
    //     for param in &mut self.parameters {
    //         match param {
    //             Parameter::RawDataFiles(_) if target == "RawDataFiles" => return param,
    //             Parameter::ScanFilters(_) if target == "ScanFilters" => return param,
    //             Parameter::ScanTypes(_) if target == "ScanTypes" => return param,
    //             Parameter::MSDetectorAdvanced(_) if target == "MSDetectorAdvanced" => return param,
    //             Parameter::DenormalizeFragmentScanTraps(_) if target == "DenormalizeFragmentScanTraps" => return param,
    //             _ => continue,
    //         }
    //     }
    //     panic!("Parameter '{}' not found", target)
    // }
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

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn add_parameter(&mut self, parameter:WaveletTransformParameters){
        self.parameters.push(parameter);
    }

    pub fn parameters_length(&self) -> usize{
        self.parameters.len()
    }

    pub fn set_parameter_value(&mut self, target: &str, value: Option<f32>){
        for param in &mut self.parameters {
            match param {
                WaveletTransformParameters::WTNoiseLevel(_f) if target == "WTNoiseLevel" => return _f.set_value(value),
                WaveletTransformParameters::ScaleLevel(_f) if target == "ScaleLevel" => return _f.set_value(value),
                WaveletTransformParameters::WaveletWindowSize(_f) if target == "WaveletWindowSize" => return _f.set_value(value),
                _ => continue,
            }
        }
    }

    pub fn get_parameter_value(&self, target: &str) -> &Option<f32>{
        for param in &self.parameters {
            match param {
                WaveletTransformParameters::WTNoiseLevel(_f) if target == "WTNoiseLevel" => return _f.get_value(),
                WaveletTransformParameters::ScaleLevel(_f) if target == "ScaleLevel" => return _f.get_value(),
                WaveletTransformParameters::WaveletWindowSize(_f) if target == "WaveletWindowSize" => return _f.get_value(),
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
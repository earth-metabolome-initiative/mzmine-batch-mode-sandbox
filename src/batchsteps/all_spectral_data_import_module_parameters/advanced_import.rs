use serde::{Serialize, Deserialize};

pub enum Value {
    Single(Option<f32>),
    Vector(Vec<Option<f32>>),
}


#[derive(Default, Serialize, Deserialize, PartialEq)]
 #[serde(default, rename_all = "lowercase")]
pub struct AdvancedImport{
    #[serde(rename = "@name")]
    name: String,

     #[serde(rename = "@selected")]
    selected: bool,
 
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
    ScanFilter(ScanFilter),
    CropMS1mz(CropMS1mz),
    MSDetectorAdvanced(MSDetectorAdvanced),
    DenormalizeFragmentScansTraps(DenormalizeFragmentScansTraps),
 }

// ### ### ### ### ### ### ###     Scan Filter     ### ### ### ### ### ### ### ### ### ###

 #[derive(Default, Serialize, Deserialize, PartialEq)]
 #[serde(default, rename_all = "lowercase")]
pub struct ScanFilter{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "@selected")]
    selected: bool,

    parameters: Vec<ScanFilterParameters>,
 }

impl ScanFilter {
    pub fn new() -> Self {
        ScanFilter {
            name: "Scan filters".to_owned(),
            selected: true,
            parameters: Vec::new(),
        }
    }

    pub fn get_name(&self) -> String{
        self.name.clone()
    }

    pub fn get_parameters_length(&self) -> usize{
        self.parameters.len()
    }

    pub fn add_parameter(&mut self, parameter:ScanFilterParameters){
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
}

 #[derive(Serialize, Deserialize, PartialEq)]
 #[serde(untagged)]
pub enum ScanFilterParameters{
    ScanNumber(ScanNumber),
    BaseFilteringInteger(BaseFilteringInteger),
    RetentionTime(RetentionTime),
    Mobility(Mobility),
    MSLevelFilter(MSLevelFilter),
    ScanDefinition(ScanDefinition),
    Polarity(Polarity),
    SpectrumType(SpectrumType),
 }

impl ScanFilterParameters{
    pub fn new(&self) -> Self{
        match self {
            ScanFilterParameters::ScanNumber(_f) => ScanFilterParameters::ScanNumber(ScanNumber::new()),
            ScanFilterParameters::BaseFilteringInteger(_f) => ScanFilterParameters::BaseFilteringInteger(BaseFilteringInteger::new()),
            ScanFilterParameters::RetentionTime(_f) => ScanFilterParameters::RetentionTime(RetentionTime::new()),
            ScanFilterParameters::Mobility(_f) => ScanFilterParameters::Mobility(Mobility::new()),
            ScanFilterParameters::MSLevelFilter(_f) => ScanFilterParameters::MSLevelFilter(MSLevelFilter::new()),
            ScanFilterParameters::ScanDefinition(_f) => ScanFilterParameters::ScanDefinition(ScanDefinition::new()),
            ScanFilterParameters::Polarity(_f) => ScanFilterParameters::Polarity(Polarity::new()),
            ScanFilterParameters::SpectrumType(_f) => ScanFilterParameters::SpectrumType(SpectrumType::new())
        }
    }
}

 #[derive(Default, Serialize, Deserialize, PartialEq)]
 #[serde(default, rename_all = "lowercase")]
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

    pub fn get_name(&self) -> String{
        self.name.clone()
    }

    pub fn set_value(&mut self, value:Option<u8>){
        self.value = value;
    }

    pub fn get_value(&self) -> Option<u8>{
        self.value
    }
}

 #[derive(Default, Serialize, Deserialize, PartialEq)]
 #[serde(default, rename_all = "lowercase")]
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

    pub fn get_name(&self) -> String{
        self.name.clone()
    }

    pub fn set_value(&mut self, value:Option<u8>){
        self.value = value;
    }

    pub fn get_value(&self) -> Option<u8>{
        self.value
    }
}

 #[derive(Default, Serialize, Deserialize, PartialEq)]
 #[serde(default, rename_all = "lowercase")]
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

    pub fn get_name(&self) -> String{
        self.name.clone()
    }

    pub fn set_value(&mut self, value:Option<u8>){
        self.value = value;
    }

    pub fn get_value(&self) -> Option<u8>{
        self.value
    }
}

 #[derive(Default, Serialize, Deserialize, PartialEq)]
 #[serde(default, rename_all = "lowercase")]
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

    pub fn get_name(&self) -> String{
        self.name.clone()
    }

    pub fn set_value(&mut self, value:Option<u8>){
        self.value = value;
    }

    pub fn get_value(&self) -> Option<u8>{
        self.value
    }
}

 #[derive(Default, Serialize, Deserialize, PartialEq)]
 #[serde(default, rename_all = "lowercase")]
pub struct MSLevelFilter{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "@selected")]
    selected: String,

    #[serde(rename = "$text")]
    value: Option<u8>,
 }

 impl MSLevelFilter {
    pub fn new() -> Self {
        MSLevelFilter {
            name: "MS level filter".to_owned(),
            selected: "All MS levels".to_owned(),
            value: Some(1),
        }
    }

    pub fn get_name(&self) -> String{
        self.name.clone()
    }

    pub fn get_selected(&self) -> String{
        self.selected.clone()
    }

    pub fn set_selected(&mut self, selection:String){
        self.selected = selection;
    }

    pub fn set_value(&mut self, value: Option<u8>){
        self.value = value;
    }

    pub fn get_value(&self) -> Option<u8>{
        self.value
    }
}


 #[derive(Default, Serialize, Deserialize, PartialEq)]
 #[serde(default, rename_all = "lowercase")]
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

    pub fn get_name(&self) -> String{
        self.name.clone()
    }

    pub fn set_value(&mut self, value: Option<u8>){
        self.value = value;
    }

    pub fn get_value(&self) -> Option<u8>{
        self.value
    }
}

 #[derive(Default, Serialize, Deserialize, PartialEq)]
 #[serde(default, rename_all = "lowercase")]
 
pub struct Polarity{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "$text")]
    value: String,
 }

 impl Polarity {
    pub fn new() -> Self {
        Polarity {
            name: "Polarity".to_owned(),
            value: "Any".to_owned(),
        }
    }

    pub fn get_name(&self) -> String{
        self.name.clone()
    }

    pub fn set_value(&mut self, value: String){
        self.value = value;
    }

    pub fn get_value(&self) -> String{
        self.value.clone()
    }
}

 #[derive(Default, Serialize, Deserialize, PartialEq)]
 #[serde(default, rename_all = "lowercase")]
pub struct SpectrumType{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "$text")]
    value: String,
 }

impl SpectrumType {
    pub fn new() -> Self {
        SpectrumType {
            name: "Spectrum type".to_owned(),
            value: "ANY".to_owned(),
        }
    }

    pub fn get_name(&self) -> String{
        self.name.clone()
    }

    pub fn set_value(&mut self, value: String){
        self.value = value;
    }

    pub fn get_value(&self) -> String{
        self.value.clone()
    }
}


// Package of Crop MS1 m/z and its subparameters

#[derive(Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
pub struct CropMS1mz{
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@selected")]
    selected: bool,

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



#[derive(Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
pub struct MSDetectorAdvanced{
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@selected")]
    selected: bool,
    #[serde(rename = "@selected_item")]
    selected_item: String,

    modules: Vec<MSDetectorAdvancedModules>,
}

impl MSDetectorAdvanced {
    pub fn new() -> Self {
        MSDetectorAdvanced {
            name: "".to_owned(),
            selected: true,
            selected_item: "Factor of lowest signal".to_owned(),
            modules: Vec::new(),
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

    pub fn get_selected_item(&self) -> String{
        self.selected_item.clone()
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

    pub fn module_length(&self) -> usize{
        self.modules.len()
    }

    pub fn get_module(&self, index:usize) -> MSDetectorAdvancedModules{
        self.modules[index].clone()
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
            _ => (),
        }
    }
}


#[derive(Default, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(default, rename_all = "lowercase")]
pub struct FactorOfLowestSignal{
    #[serde(rename = "@name")]
    name: String,

    parameter: ParameterFactorOfLowestSignal,
}

impl FactorOfLowestSignal{
    pub fn new(value:Option<f32>) -> Self{
        FactorOfLowestSignal{
            name: "Factor of lowest signal".to_owned(),
            parameter: ParameterFactorOfLowestSignal::new(value)
        }
    }

    pub fn set_value(&mut self, value:Option<f32>){
        self.parameter.set_value(value);
    }

    pub fn get_value(& self) -> &Option<f32>{
        self.parameter.get_value()
    }
}

#[derive(Default, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(default, rename_all = "lowercase")]
pub struct ParameterFactorOfLowestSignal{
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "$text")]
    value: Option<f32>,
}

impl ParameterFactorOfLowestSignal {
    fn new(value: Option<f32>) -> Self{
        ParameterFactorOfLowestSignal{
            name: "Factor of lowest signal".to_owned(),
            value: Some(0.0),
        }
    }

    fn set_value(&mut self, value: Option<f32>){
        self.value = value;
    }

    fn get_value(& self) -> &Option<f32>{
        &self.value
    }
}

#[derive(Default, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(default, rename_all = "lowercase")]
pub struct Auto{
    #[serde(rename = "@name")]
    name: String,

    parameter: ParameterAuto,
}

impl Auto{
    pub fn new() -> Self{
        Auto { 
            name: "Auto".to_owned(), 
            parameter: ParameterAuto::new() }
    }

    pub fn get_name(&self) -> String{
        self.name.clone()
    }

    pub fn get_value(&self) -> &Option<f32>{
        & self.parameter.get_value()
    }

    pub fn set_value(&mut self, value:Option<f32>){
        self.parameter.set_value(value);
    }
}

#[derive(Default, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(default, rename_all = "lowercase")]
pub struct ParameterAuto{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "$text")]
    value: Option<f32>,
}


impl ParameterAuto{
    pub fn new() -> Self{
        ParameterAuto{
            name: "Noise level".to_owned(),
            value: Some(1000.0),
        }
    }

    pub fn get_name(&self) -> String{
        self.name.clone()
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
#[serde(default, rename_all = "lowercase")]
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

    pub fn get_name(&self) -> String{
        self.name.clone()
    }

    pub fn set_value(&mut self, value:Option<f32>){
        self.parameter.set_value(value);
    }

    pub fn get_value(&self) -> &Option<f32>{
        self.parameter.get_value()
    }
}

#[derive(Default, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(default, rename_all = "lowercase")]
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

    pub fn get_name(&self) -> String{
        self.name.clone()
    }

    pub fn get_value(&self) -> &Option<f32>{
        &self.value
    }

    pub fn set_value(&mut self, value:Option<f32>){
        self.value = value;
    }
}

#[derive(Default, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(default, rename_all = "lowercase")]
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

    pub fn get_name(&self) -> String{
        self.name.clone()
    }

    pub fn get_value(&self) -> &Option<f32>{
        self.parameter.get_value()
    }

    pub fn set_value(&mut self, value:Option<f32>){
        self.parameter.set_value(value);
    }
}

#[derive(Default, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(default, rename_all = "lowercase")]
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

    pub fn get_name(&self) -> String{
        self.name.clone()
    }

    pub fn get_value(&self) -> &Option<f32>{
        &self.value
    }

    pub fn set_value(&mut self, value: Option<f32>){
        self.value = value;
    }
}

#[derive(Default, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(default, rename_all = "lowercase")]
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

    pub fn get_name(&self) -> String{
        self.name.clone()
    }

    pub fn get_value(&self) -> &Option<f32>{
        self.parameter.get_value()
    }

    pub fn set_value(&mut self, value:Option<f32>){
        self.parameter.set_value(value);
    }
}

#[derive(Default, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(default, rename_all = "lowercase")]
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

    pub fn get_name(&self) -> String{
        self.name.clone()
    }

    pub fn get_value(&self) -> &Option<f32>{
        &self.value
    }

    pub fn set_value(&mut self, value:Option<f32>){
        self.value = value;
    }
}


#[derive(Default, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(default, rename_all = "lowercase")]
pub struct RecursiveThreshold{
    #[serde(rename = "@name")]
    name: String,

    parameters: Vec<RecursiveThresholdParameters>,
}

impl RecursiveThreshold{
    pub fn new() -> Self{
        RecursiveThreshold{
            name: "Recursive threshold".to_owned(),
            parameters: Vec::new(),
        }
    }

    pub fn get_name(&self) -> String{
        self.name.clone()
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

    pub fn get_parameter_value(&self, target: &str) -> Option<f32>{
        for param in &self.parameters {
            match param {
                RecursiveThresholdParameters::RTNoiseLevel(rt) if target == "RTNoiseLevel" => return rt.get_value(),
                RecursiveThresholdParameters::MinMZPeakWidth(min_mz) if target == "MinMZPeakWidth" => return min_mz.get_value(),
                RecursiveThresholdParameters::MaxMZPeakWidth(max_mz) if target == "MaxMZPeakWidth" => return max_mz.get_value(),
                _ => continue,
            }
        }
        None
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
    
    pub fn get_value(&self) -> Option<f32>{
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
#[serde(default, rename_all = "lowercase")]
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

    pub fn get_name(&self) -> String{
        self.name.clone()
    }

    pub fn set_value(&mut self, value: Option<f32>){
        self.value = value;
    }

    pub fn get_value(&self) -> Option<f32>{
        self.value
    }
}

#[derive(Default, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(default, rename_all = "lowercase")]
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

    pub fn get_name(&self) -> String{
        self.name.clone()
    }

    pub fn set_value(&mut self, value: Option<f32>){
        self.value = value;
    }

    pub fn get_value(&self) -> Option<f32>{
        self.value
    }
}

#[derive(Default, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(default, rename_all = "lowercase")]
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

    pub fn get_name(&self) -> String{
        self.name.clone()
    }

    pub fn set_value(&mut self, value:Option<f32>){
        self.value = value;
    }
    
    pub fn get_value(&self) -> Option<f32>{
        self.value
    }
}

#[derive(Default, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(default, rename_all = "lowercase")]
pub struct WaveletTransform{
    #[serde(rename = "@name")]
    name: String,

    parameters: Vec<WaveletTransformParameters>,
}

impl WaveletTransform {
    pub fn new() -> Self {
        WaveletTransform {
            name: "Wavelet transform".to_owned(),
            parameters: Vec::new(),
        }
    }

    pub fn get_name(&self) -> String{
        self.name.clone()
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

    pub fn get_parameter_value(&self, target: &str) -> Option<f32>{
        for param in &self.parameters {
            match param {
                WaveletTransformParameters::WTNoiseLevel(_f) if target == "WTNoiseLevel" => return _f.get_value(),
                WaveletTransformParameters::ScaleLevel(_f) if target == "ScaleLevel" => return _f.get_value(),
                WaveletTransformParameters::WaveletWindowSize(_f) if target == "WaveletWindowSize" => return _f.get_value(),
                _ => continue,
            }
        }
        None
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
    pub fn get_value(&self) -> Option<f32> {
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
#[serde(default, rename_all = "lowercase")]
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

    pub fn get_name(&self) -> String{
        self.name.clone()
    }

    pub fn get_value(&self) -> Option<f32>{
        self.value
    }

    pub fn set_value(&mut self, value:Option<f32>){
        self.value = value;
    }
}

#[derive(Default, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(default, rename_all = "lowercase")]
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

    pub fn get_name(&self) -> String{
        self.name.clone()
    }

    pub fn get_value(&self) -> Option<f32>{
        self.value
    }

    pub fn set_value(&mut self, value: Option<f32>){
        self.value = value;
    }
}

#[derive(Default, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(default, rename_all = "lowercase")]
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

    pub fn get_name(&self) -> String{
        self.name.clone()
    }

    pub fn get_value(&self) -> Option<f32>{
        self.value
    }

    pub fn set_value(&mut self, value: Option<f32>){
        self.value = value;
    }
}

#[derive(Default, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(default, rename_all = "lowercase")]
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

    pub fn get_name(&self) -> String{
        self.name.clone()
    }

    pub fn get_value(&self) -> bool{
        self.value
    }

    pub fn set_true(&mut self) {
        self.value = true;
    }

    pub fn set_false(&mut self){
        self.value = false;
    }
}
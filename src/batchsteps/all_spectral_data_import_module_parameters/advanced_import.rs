use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq)]
 #[serde(default, rename_all = "lowercase")]
pub struct AdvancedImport{
    #[serde(rename = "@name")]
    name: String,

     #[serde(rename = "@selected")]
     selected: bool,
 
     parameter: Vec<AdvancedImportParameters>
 }

 impl Default for AdvancedImport {
    fn default() -> Self {
        AdvancedImport {
            name: "Advanced import".to_owned(),
            selected: false,
            parameter: vec![
                AdvancedImportParameters::ScanFilter(ScanFilter::default()),
                AdvancedImportParameters::CropMS1mz(CropMS1mz::default()),
                AdvancedImportParameters::MS1DetectorAdvanced(MSDetectorAdvanced::default()) //MS1
                //AdvancedImportParameters::MS2DetectorAdvanced(MS2DetectorAdvanced::default())  //MS2
            ],
        }
    }
}


 
 #[derive(Serialize, Deserialize, PartialEq)]
 #[serde(untagged)]
 enum AdvancedImportParameters{
    ScanFilter(ScanFilter),
    CropMS1mz(CropMS1mz),
    MS1DetectorAdvanced(MSDetectorAdvanced),
    // MS2DetectorAdvanced(MS2DetectorAdvanced),
    // DenormalizeFragmentScansTraps(DenormalizeFragmentScansTraps),
 }


// ### ### ### ### ### ### ###     Scan Filter     ### ### ### ### ### ### ### ### ### ###

 #[derive(Serialize, Deserialize, PartialEq)]
 #[serde(default, rename_all = "lowercase")]
 struct ScanFilter{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "@selected")]
    selected: bool,

    parameter: Vec<ScanFilterParameters>,
 }

impl Default for ScanFilter {
    fn default() -> Self {
        ScanFilter {
            name: "Scan filters".to_owned(),
            selected: true,
            parameter: vec![
                ScanFilterParameters::ScanNumber(ScanNumber::default()),
                ScanFilterParameters::BaseFilteringInteger(BaseFilteringInteger::default()),
                ScanFilterParameters::RetentionTime(RetentionTime::default()),
                ScanFilterParameters::Mobility(Mobility::default()),
                ScanFilterParameters::MSLevelFilter(MSLevelFilter::default()),
                ScanFilterParameters::ScanDefinition(ScanDefinition::default()),
                ScanFilterParameters::Polarity(Polarity::default()),
                ScanFilterParameters::SpectrumType(SpectrumType::default())
            ],
        }
    }
}

 #[derive(Serialize, Deserialize, PartialEq)]
 #[serde(untagged)]
 enum ScanFilterParameters{
    ScanNumber(ScanNumber),
    BaseFilteringInteger(BaseFilteringInteger),
    RetentionTime(RetentionTime),
    Mobility(Mobility),
    MSLevelFilter(MSLevelFilter),
    ScanDefinition(ScanDefinition),
    Polarity(Polarity),
    SpectrumType(SpectrumType),
 }

 #[derive(Serialize, Deserialize, PartialEq)]
 #[serde(default, rename_all = "lowercase")]
 struct ScanNumber{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "$text")]
    value: Option<u8>,
 }

 impl Default for ScanNumber {
    fn default() -> Self {
        ScanNumber {
            name: "Scan number".to_owned(),
            value: None,
        }
    }
}

 #[derive(Serialize, Deserialize, PartialEq)]
 #[serde(default, rename_all = "lowercase")]
 struct BaseFilteringInteger{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "$text")]
    value: Option<u8>,
 }

 impl Default for BaseFilteringInteger {
    fn default() -> Self {
        BaseFilteringInteger {
            name: "Base Filtering Integer".to_owned(),
            value: None,
        }
    }
}

 #[derive(Serialize, Deserialize, PartialEq)]
 #[serde(default, rename_all = "lowercase")]
 struct RetentionTime{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "$text")]
    value: Option<u8>,
 }

impl Default for RetentionTime {
    fn default() -> Self {
        RetentionTime {
            name: "Retention time".to_owned(),
            value: None,
        }
    }
}

 #[derive(Serialize, Deserialize, PartialEq)]
 #[serde(default, rename_all = "lowercase")]
 struct Mobility{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "$text")]
    value: Option<u8>,
 }

 impl Default for Mobility {
    fn default() -> Self {
        Mobility {
            name: "Mobility".to_owned(),
            value: None,
        }
    }
}

 #[derive(Serialize, Deserialize, PartialEq)]
 #[serde(default, rename_all = "lowercase")]
 struct MSLevelFilter{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "@selected")]
    selected: String,

    #[serde(rename = "$text")]
    value: u8,
 }

 impl Default for MSLevelFilter {
    fn default() -> Self {
        MSLevelFilter {
            name: "MS level filter".to_owned(),
            selected: "All MS levels".to_owned(),
            value: 1,
        }
    }
}


 #[derive(Serialize, Deserialize, PartialEq)]
 #[serde(default, rename_all = "lowercase")]
 struct ScanDefinition{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "$text")]
    value: Option<u8>,
 }

 impl Default for ScanDefinition {
    fn default() -> Self {
        ScanDefinition {
            name: "Scan definition".to_owned(),
            value: None,
        }
    }
}

 #[derive(Serialize, Deserialize, PartialEq)]
 #[serde(default, rename_all = "lowercase")]
 struct Polarity{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "$text")]
    value: String,
 }

 impl Default for Polarity {
    fn default() -> Self {
        Polarity {
            name: "Polarity".to_owned(),
            value: "Any".to_owned(),
        }
    }
}

 #[derive(Serialize, Deserialize, PartialEq)]
 #[serde(default, rename_all = "lowercase")]
 struct SpectrumType{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "$text")]
    value: String,
 }

 impl Default for SpectrumType {
    fn default() -> Self {
        SpectrumType {
            name: "Spectrum type".to_owned(),
            value: "ANY".to_owned(),
        }
    }
}


// Package of Crop MS1 m/z and its subparameters

#[derive(Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
struct CropMS1mz{
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@selected")]
    selected: bool,
}

impl Default for CropMS1mz {
    fn default() -> Self {
        CropMS1mz {
            name: "Crop MS1 m/z".to_owned(),
            selected: false,
        }
    }
}



// ### ### ### ### ### ### ###     MS Advanced     ### ### ### ### ### ### ### ### ### ###



#[derive(Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
struct MSDetectorAdvanced{
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@selected")]
    selected: bool,
    #[serde(rename = "@selected_item")]
    selected_item: String,

    modules: Vec<MSDetectorAdvancedModules>,
}

impl MSDetectorAdvanced {
    fn new() -> Self {
        MSDetectorAdvanced {
            name: "".to_owned(),
            selected: true,
            selected_item: "Factor of lowest signal".to_owned(),
            modules: Vec::new(),
        }
    }

    fn set_ms1(&mut self, value: f32) {
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

    fn set_ms2(&mut self, value: f32) {
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
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(untagged)]
enum MSDetectorAdvancedModules {
    FactorOfLowestSignal(FactorOfLowestSignal),
    Auto(Auto),
    Centroid(Centroid),
    ExactMass(ExactMass),
    LocalMaxima(LocalMaxima),
    RecursiveThreshold(RecursiveThreshold),
    WaveletTransform(WaveletTransform)
}

impl MSDetectorAdvancedModules {
    // Returns an Option<f32> since not all variants may have a value
    fn get_value(&self) -> Option<f32> {
        match self {
            MSDetectorAdvancedModules::FactorOfLowestSignal(f) => Some(f.get_value()),
            // Handle other variants if needed, or return None
            _ => None,
        }
    }
}


#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(default, rename_all = "lowercase")]
struct FactorOfLowestSignal{
    #[serde(rename = "@name")]
    name: String,

    parameter: ParameterFactorOfLowestSignal,
}

impl Default for FactorOfLowestSignal {
    fn default() -> Self {
        FactorOfLowestSignal {
            name: "Factor of lowest signal".to_owned(),
            parameter: ParameterFactorOfLowestSignal::default(),
        }
    }
}

impl FactorOfLowestSignal{
    pub fn new(value:f32) -> Self{
        FactorOfLowestSignal{
            name: "Factor of lowest signal".to_owned(),
            parameter: ParameterFactorOfLowestSignal::new(value)
        }
    }

    fn set_value(&mut self, value:f32){
        self.parameter.set_value(value);
    }

    fn get_value(& self) -> f32{
        self.parameter.get_value()
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(default, rename_all = "lowercase")]
struct ParameterFactorOfLowestSignal{
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "$text")]
    value: f32,
}
impl Default for ParameterFactorOfLowestSignal {
    fn default() -> Self {
        ParameterFactorOfLowestSignal{
            name: "Factor of lowest signal".to_owned(),
            value: 5.0,
        }
    }
}

impl ParameterFactorOfLowestSignal {
    fn new(value: f32) -> Self{
        ParameterFactorOfLowestSignal{
            name: "Factor of lowest signal".to_owned(),
            value: value,
        }
    }

    fn set_value(&mut self, value: f32){
        self.value = value;
    }

    fn get_value(& self) -> f32{
        self.value
    }
}

#[derive(Default, Serialize, Deserialize, PartialEq, Debug)]
#[serde(default, rename_all = "lowercase")]
struct Auto{
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

    pub fn get_value(&self) -> f32{
        self.parameter.get_value()
    }

    pub fn set_value(&mut self, value:f32){
        self.parameter.set_value(value);
    }
}

#[derive(Default, Serialize, Deserialize, PartialEq, Debug)]
#[serde(default, rename_all = "lowercase")]
struct ParameterAuto{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "$text")]
    value: f32,
}


impl ParameterAuto{
    pub fn new() -> Self{
        ParameterAuto{
            name: "Noise level".to_owned(),
            value: 1000.0,
        }
    }

    fn get_value(& self)->f32{
        self.value
    }

    fn set_value(&mut self, value:f32){
        self.value = value;
    }
}

#[derive(Default, Serialize, Deserialize, PartialEq, Debug)]
#[serde(default, rename_all = "lowercase")]
struct Centroid{
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

    pub fn set_value(&mut self, value:Option<f32>){
        self.parameter.set_value(value);
    }

    pub fn get_value(&self) -> Option<f32>{
        self.parameter.get_value()
    }
}

#[derive(Default, Serialize, Deserialize, PartialEq, Debug)]
#[serde(default, rename_all = "lowercase")]
struct ParameterCentroid{
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

    fn get_value(&self) -> Option<f32>{
        self.value
    }

    fn set_value(&mut self, value:Option<f32>){
        self.value = value;
    }
}

#[derive(Default, Serialize, Deserialize, PartialEq, Debug)]
#[serde(default, rename_all = "lowercase")]
struct ExactMass{
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

    pub fn get_value(&self) -> Option<f32>{
        self.parameter.get_value()
    }

    pub fn set_value(&mut self, value:Option<f32>){
        self.parameter.set_value(value);
    }
}

#[derive(Default, Serialize, Deserialize, PartialEq, Debug)]
#[serde(default, rename_all = "lowercase")]
struct ParameterExactMass{
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

    pub fn get_value(&self) -> Option<f32>{
        self.value
    }

    pub fn set_value(&mut self, value: Option<f32>){
        self.value = value;
    }
}

#[derive(Default, Serialize, Deserialize, PartialEq, Debug)]
#[serde(default, rename_all = "lowercase")]
struct LocalMaxima{
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

    pub fn get_value(&self) -> Option<f32>{
        self.parameter.get_value()
    }

    pub fn set_value(&mut self, value:Option<f32>){
        self.parameter.set_value(value);
    }
}

#[derive(Default, Serialize, Deserialize, PartialEq, Debug)]
#[serde(default, rename_all = "lowercase")]
struct ParameterLocalMaxima{
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

    pub fn get_value(&self) -> Option<f32>{
        self.value
    }

    pub fn set_value(&mut self, value:Option<f32>){
        self.value = value;
    }
}


#[derive(Default, Serialize, Deserialize, PartialEq, Debug)]
#[serde(default, rename_all = "lowercase")]
struct RecursiveThreshold{
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

    pub fn add_parameter(&mut self, parameter: RecursiveThresholdParameters){
        self.parameters.push(parameter);
    }   

    pub fn set_parameter_value(&mut self, index: usize, value: Option<f32>){
        self.parameters[index].set_value(value);
    }

    pub fn get_parameter_value(&self, index: usize) -> Option<f32>{
        self.parameters[index].get_value()
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(untagged)]
enum RecursiveThresholdParameters{
    RTNoiseLevel(RTNoiseLevel),
    MinMZPeakWidth(MinMZPeakWidth),
    MaxMZPeakWidth(MaxMZPeakWidth),
}

impl RecursiveThresholdParameters{
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

#[derive(Default, Serialize, Deserialize, PartialEq, Debug)]
#[serde(default, rename_all = "lowercase")]
struct RTNoiseLevel{
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

    pub fn set_value(&mut self, value: Option<f32>){
        self.value = value;
    }

    pub fn get_value(&self) -> Option<f32>{
        self.value
    }
}

#[derive(Default, Serialize, Deserialize, PartialEq, Debug)]
#[serde(default, rename_all = "lowercase")]
struct MinMZPeakWidth{
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

    pub fn set_value(&mut self, value: Option<f32>){
        self.value = value;
    }

    pub fn get_value(&self) -> Option<f32>{
        self.value
    }
}

#[derive(Default, Serialize, Deserialize, PartialEq, Debug)]
#[serde(default, rename_all = "lowercase")]
struct MaxMZPeakWidth{
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

    pub fn set_value(&mut self, value:Option<f32>){
        self.value = value;
    }
    
    pub fn get_value(&self) -> Option<f32>{
        self.value
    }
}

#[derive(Default, Serialize, Deserialize, PartialEq, Debug)]
#[serde(default, rename_all = "lowercase")]
struct WaveletTransform{
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

    pub fn get_value(&self, index: usize) -> Option<f32>{
        self.parameters[index].get_value()
    }

    pub fn set_value(&mut self, index:usize, value:Option<f32>){
        self.parameters[index].set_value(value);
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(untagged)]
enum WaveletTransformParameters{
    WTNoiseLevel(RTNoiseLevel),
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

#[derive(Default, Serialize, Deserialize, PartialEq, Debug)]
#[serde(default, rename_all = "lowercase")]
struct WTNoiseLevel{
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

    pub fn get_value(&self) -> Option<f32>{
        self.value
    }

    pub fn set_value(&mut self, value:Option<f32>){
        self.value = value;
    }
}

#[derive(Default, Serialize, Deserialize, PartialEq, Debug)]
#[serde(default, rename_all = "lowercase")]
struct ScaleLevel{
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

    pub fn get_value(&self) -> Option<f32>{
        self.value
    }

    pub fn set_value(&mut self, value: Option<f32>){
        self.value = value;
    }
}

#[derive(Default, Serialize, Deserialize, PartialEq, Debug)]
#[serde(default, rename_all = "lowercase")]
struct WaveletWindowSize{
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

    pub fn get_value(&self) -> Option<f32>{
        self.value
    }

    pub fn set_value(&mut self, value: Option<f32>){
        self.value = value;
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    //Still need to test serialization

    #[test]
    fn test_ms1_content(){
        let mut ms_detector = MSDetectorAdvanced::new();
        assert_eq!(ms_detector.name, "");                                                                                    //test it has been initialized correctly
        ms_detector.set_ms1(0.0);
        assert_eq!(ms_detector.name, "MS1 detector (Advanced)");                                                             //test it has the correct name
        ms_detector.add_module(MSDetectorAdvancedModules::FactorOfLowestSignal(FactorOfLowestSignal::new(0.0))); 
        assert_eq!(ms_detector.modules.len(), 1);                                                                            //test something has been inserted
        assert_eq!(ms_detector.modules[0], MSDetectorAdvancedModules::FactorOfLowestSignal(FactorOfLowestSignal::new(0.0))); //test that it is in fact this type of object

        ms_detector.set_ms1(7.0);
        assert_eq!(ms_detector.modules[0].get_value(), Some(7.0));
    }

    #[test]
    fn test_ms2_content(){
        let mut ms_detector = MSDetectorAdvanced::new();
        assert_eq!(ms_detector.name, "", "NOT empty");                                                                                    //test it has been initialized correctly
        ms_detector.set_ms2(0.0);
        assert_eq!(ms_detector.name, "MS2 detector (Advanced)", "NOT same name");                                                             //test it has the correct name
        ms_detector.add_module(MSDetectorAdvancedModules::FactorOfLowestSignal(FactorOfLowestSignal::new(0.0))); 
        assert_eq!(ms_detector.modules.len(), 1, "NOT 1 element pushed");                                                                            //test something has been inserted
        assert_eq!(ms_detector.modules[0], MSDetectorAdvancedModules::FactorOfLowestSignal(FactorOfLowestSignal::new(0.0)), "NOT good type inserted"); //test that it is in fact this type of object

        ms_detector.set_ms2(1000.0);
        assert_eq!(ms_detector.modules[0].get_value(), Some(1000.0), "NOT matching value");
    }

    #[test]
    fn test_auto_initialization(){
        let auto_obj = Auto::new();
        assert_eq!(auto_obj.name, "Auto");
    }

    #[test]
    fn test_auto_set_value(){
        let mut auto_obj = Auto::new();
        auto_obj.set_value(17.0);
        assert_eq!(auto_obj.parameter.get_value(), 17.0);
    }

    #[test]
    fn test_parameter_auto_initialization(){
        let parameter_auto = ParameterAuto::new();
        assert_eq!(parameter_auto.name, "Noise level");
    }

    #[test]
    fn test_parameter_auto_set_value(){
        let mut parameter_auto_obj = ParameterAuto::new();
        assert_eq!(parameter_auto_obj.value, 1000.0, "NOT initialized correctely");
        parameter_auto_obj.set_value(28.4);
        assert_eq!(parameter_auto_obj.value, 28.4, "NOT setted correctely");
    }

    #[test]
    fn test_parameter_auto_get_value(){
        let mut parameter_auto_obj = ParameterAuto::new();
        assert_eq!(parameter_auto_obj.get_value(), 1000.0, "NOT the right value");
    }

    #[test]
    fn test_centroid_initialization(){
        let centroid_obj = Centroid::new();
        assert_eq!(centroid_obj.name, "Centroid");
        assert_eq!(centroid_obj.parameter.value, None);
    }


    #[test]
    fn test_centroid_set_value(){
        let mut centroid_obj = Centroid::new();
        assert_eq!(centroid_obj.parameter.value, None);
        centroid_obj.set_value(Some(34.8));
        assert_eq!(centroid_obj.parameter.value, Some(34.8));
    }

    #[test]
    fn test_centroid_get_value(){
        let mut centroid_obj = Centroid::new();
        assert_eq!(centroid_obj.parameter.value, None);
        centroid_obj.set_value(Some(46.9));
        assert_eq!(centroid_obj.get_value(), Some(46.9));
    }

    #[test]
    fn test_centroid_parameter_initialization(){
        let par_centroid_obj = ParameterCentroid::new();
        assert_eq!(par_centroid_obj.name, "Noise level");
    }

    #[test]
    fn test_centroid_parameter_set_value(){
        let mut par_centroid_obj = ParameterCentroid::new();
        assert_eq!(par_centroid_obj.value, None);
        par_centroid_obj.set_value(Some(12.35));
        assert_eq!(par_centroid_obj.value, Some(12.35));
    }

    #[test]
    fn test_centroid_parameter_get_value(){
        let mut par_centroid_obj = ParameterCentroid::new();
        par_centroid_obj.set_value(Some(45.6));
        assert_eq!(par_centroid_obj.get_value(), Some(45.6));
    }

    #[test]
    fn test_ExactMass_initialization(){
        let centroid_obj = ExactMass::new();
        assert_eq!(centroid_obj.name, "Exact mass");
        assert_eq!(centroid_obj.parameter.value, None);
    }


    #[test]
    fn test_ExactMass_set_value(){
        let mut exact_mass_obj = ExactMass::new();
        assert_eq!(exact_mass_obj.parameter.value, None);
        exact_mass_obj.set_value(Some(34.8));
        assert_eq!(exact_mass_obj.parameter.value, Some(34.8));
    }

    #[test]
    fn test_ExactMass_get_value(){
        let mut exact_mass_obj = ExactMass::new();
        assert_eq!(exact_mass_obj.parameter.value, None);
        exact_mass_obj.set_value(Some(46.9));
        assert_eq!(exact_mass_obj.get_value(), Some(46.9));
    }

    #[test]
    fn test_ExactMass_parameter_initialization(){
        let par_exact_mass_obj = ParameterExactMass::new();
        assert_eq!(par_exact_mass_obj.name, "Noise level");
    }

    #[test]
    fn test_ExactMass_parameter_set_value(){
        let mut par_centroid_obj = ParameterExactMass::new();
        assert_eq!(par_centroid_obj.value, None);
        par_centroid_obj.set_value(Some(12.35));
        assert_eq!(par_centroid_obj.value, Some(12.35));
    }

    #[test]
    fn test_ExactMass_parameter_get_value(){
        let mut par_centroid_obj = ParameterCentroid::new();
        par_centroid_obj.set_value(Some(45.6));
        assert_eq!(par_centroid_obj.get_value(), Some(45.6));
    }
    
    #[test]
    fn test_LocalMaxima_initialization(){
        let centroid_obj = LocalMaxima::new();
        assert_eq!(centroid_obj.name, "Local maxima");
        assert_eq!(centroid_obj.parameter.value, None);
    }


    #[test]
    fn test_LocalMaxima_set_value(){
        let mut local_maxima_obj = LocalMaxima::new();
        assert_eq!(local_maxima_obj.parameter.value, None);
        local_maxima_obj.set_value(Some(34.8));
        assert_eq!(local_maxima_obj.parameter.value, Some(34.8));
    }

    #[test]
    fn test_LocalMaxima_get_value(){
        let mut local_maxima_obj = LocalMaxima::new();
        assert_eq!(local_maxima_obj.parameter.value, None);
        local_maxima_obj.set_value(Some(46.9));
        assert_eq!(local_maxima_obj.get_value(), Some(46.9));
    }

    #[test]
    fn test_LocalMaxima_parameter_initialization(){
        let par_local_maxima_obj = ParameterLocalMaxima::new();
        assert_eq!(par_local_maxima_obj.name, "Noise level");
    }

    #[test]
    fn test_LocalMaxima_parameter_set_value(){
        let mut par_local_maxima_obj = ParameterLocalMaxima::new();
        assert_eq!(par_local_maxima_obj.value, None);
        par_local_maxima_obj.set_value(Some(12.35));
        assert_eq!(par_local_maxima_obj.value, Some(12.35));
    }

    #[test]
    fn test_LocalMaxima_parameter_get_value(){
        let mut par_local_maxima_obj = ParameterLocalMaxima::new();
        par_local_maxima_obj.set_value(Some(45.6));
        assert_eq!(par_local_maxima_obj.get_value(), Some(45.6));
    }

    // // //

    #[test]
    fn test_Recursive_Threshold_initialization(){
        let recursive_thr_obj = RecursiveThreshold::new();
        assert_eq!(recursive_thr_obj.name, "Recursive threshold");
        assert_eq!(recursive_thr_obj.parameters.len(), 0);
    }

    #[test]
    fn test_RTNoiseLevel_initialization(){
        let rt_noise_level_obj = RTNoiseLevel::new();
        assert_eq!(rt_noise_level_obj.name, "Noise level");
        assert_eq!(rt_noise_level_obj.value, None);
    }

}


//
//  FactorOfLowestSignal(FactorOfLowestSignal),
//  Auto(Auto),
//  Centroid(Centroid),
//  ExactMass(ExactMass),
//  LocalMaxima(LocalMaxima),
//  RecursiveThreshold(RecursiveThreshold),
//  WaveletTransform(WaveletTransform)
//


// enum RecursiveThresholdParameters{
//     RTNoiseLevel(RTNoiseLevel),
//     MinMZPeakWidth(MinMZPeakWidth),
//     MaxMZPeakWidth(MaxMZPeakWidth),
// }
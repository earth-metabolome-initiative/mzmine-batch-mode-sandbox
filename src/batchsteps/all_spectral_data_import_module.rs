use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")] // rinominare oggetto come lo vogliamo stampato
pub struct AllSpectralDataImportModule {
    #[serde(rename = "@method")]
    method: String,

    #[serde(rename = "@parameter_version")]
    parameter_version: u8,

    parameter: Vec<Parameter>,
}

impl Default for AllSpectralDataImportModule {
    fn default() -> Self {
        AllSpectralDataImportModule {
            method: "io.github.mzmine.modules.io.import_rawdata_all.AllSpectralDataImportModule".to_owned(),
            parameter_version: 1,
            parameter: vec![
                Parameter::FileNames(FileNames::default()),
                Parameter::AdvancedImport(AdvancedImport::default())
                ],
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
enum Parameter {
    FileNames(FileNames),
    AdvancedImport(AdvancedImport),
    // MetadataFile(MetadataFile),
    // SpectralLibraryFiles(SpectralLibraryFiles),
}// 

#[derive(Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
pub struct FileNames {
    #[serde(rename = "@name")]
    name: String,
    //#[serde(rename = "@file")]
    file: Vec<File>,
}

impl Default for FileNames {
    fn default() -> Self {
        FileNames {
            name: "File names".to_owned(),
            file: vec![File::default(),File::default()],
        }
    }
}


#[derive(Serialize, Deserialize, PartialEq)]
#[serde(default, rename = "file", rename_all = "lowercase")]
pub struct File {
    #[serde(rename = "$text")]
    name: String
}

impl Default for File {
    fn default() -> Self {
        File {
            name: "File_name".to_owned(),
        }
    }
}

 #[derive(Serialize, Deserialize, PartialEq)]
 #[serde(default, rename_all = "lowercase")]
 struct AdvancedImport{
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
                AdvancedImportParameters::MS1DetectorAdvanced(MS1DetectorAdvanced::default()) //MS1
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
    MS1DetectorAdvanced(MS1DetectorAdvanced),
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



// ### ### ### ### ### ### ###     MS1     ### ### ### ### ### ### ### ### ### ###



#[derive(Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
struct MS1DetectorAdvanced{
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "@selected")]
    selected: bool,
    #[serde(rename = "@selected_item")]
    selected_item: String,

    module: Vec<MSDetectorAdvancedModules>,
}

impl Default for MS1DetectorAdvanced {
    fn default() -> Self {
        MS1DetectorAdvanced {
            name: "MS1 detector (Advanced)".to_owned(),
            selected: true,
            selected_item: "Factor of lowest signal".to_owned(),
            module: vec![
                MSDetectorAdvancedModules::FactorOfLowestSignal(FactorOfLowestSignal::default()),
                MSDetectorAdvancedModules::Auto(Auto::default()),
                MSDetectorAdvancedModules::Centroid(Centroid::default()),
                MSDetectorAdvancedModules::ExactMass(ExactMass::default()),
                MSDetectorAdvancedModules::LocalMaxima(LocalMaxima::default()),
                MSDetectorAdvancedModules::RecursiveThreshold(RecursiveThreshold::default())
            ],
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq)]
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

#[derive(Serialize, Deserialize, PartialEq)]
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

#[derive(Serialize, Deserialize, PartialEq)]
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
    // Method to get a default MS2 value
    fn ms2_default() -> Self {
        ParameterFactorOfLowestSignal {
            name: "Factor of lowest signal".to_owned(),
            value: 0.0,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
struct Auto{
    #[serde(rename = "@name")]
    name: String,

    parameter: ParameterAuto,
}

impl Default for Auto {
    fn default() -> Self {
        Auto {
            name: "Auto".to_owned(),
            parameter: ParameterAuto::default(),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
struct ParameterAuto{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "$text")]
    value: f32,
}
impl Default for ParameterAuto {
    fn default() -> Self {
        ParameterAuto {
            name: "Noise level".to_owned(),
            value: 1000.0,
        }
    } 
}

#[derive(Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
struct Centroid{
    #[serde(rename = "@name")]
    name: String,

    parameter: ParameterCentroid,
}

impl Default for Centroid {
    fn default() -> Self {
        Centroid {
            name: "Centroid".to_owned(),
            parameter: ParameterCentroid::default(),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
struct ParameterCentroid{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "$text")]
    value: Option<f32>,
}
impl Default for ParameterCentroid {
    fn default() -> Self {
        ParameterCentroid {
            name: "Noise level".to_owned(),
            value: None,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
struct ExactMass{
    #[serde(rename = "@name")]
    name: String,

    parameter: ParameterExactMass,
}
impl Default for ExactMass {
    fn default() -> Self {
        ExactMass {
            name: "Exact mass".to_owned(),
            parameter: ParameterExactMass::default(),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
struct ParameterExactMass{
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "$text")]
    value: Option<f32>, 
}
impl Default for ParameterExactMass {
    fn default() -> Self {
        ParameterExactMass {
            name: "Noise level".to_owned(),
            value: None,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
struct LocalMaxima{
    #[serde(rename = "@name")]
    name: String,

    parameter: ParameterLocalMaxima,
}
impl Default for LocalMaxima {
    fn default() -> Self {
        LocalMaxima {
            name: "Local maxima".to_owned(),
            parameter: ParameterLocalMaxima::default(),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
struct ParameterLocalMaxima{
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "$text")]
    value: Option<f32>, 
}
impl Default for ParameterLocalMaxima {
    fn default() -> Self {
        ParameterLocalMaxima {
            name: "Noise level".to_owned(),
            value: None,
        }
    }
}


#[derive(Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
struct RecursiveThreshold{
    #[serde(rename = "@name")]
    name: String,

    parameter: Vec<RecursiveThresholdParameters>,
}
impl Default for RecursiveThreshold {
    fn default() -> Self {
        RecursiveThreshold {
            name: "Local maxima".to_owned(),
            parameter: vec![
                RecursiveThresholdParameters::RTNoiseLevel(RTNoiseLevel::default()),
                RecursiveThresholdParameters::MinMZPeakWidth(MinMZPeakWidth::default()),
                RecursiveThresholdParameters::MaxMZPeakWidth(MaxMZPeakWidth::default())
            ],
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
enum RecursiveThresholdParameters{
    RTNoiseLevel(RTNoiseLevel),
    MinMZPeakWidth(MinMZPeakWidth),
    MaxMZPeakWidth(MaxMZPeakWidth),
}

#[derive(Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
struct RTNoiseLevel{
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "$text")]
    value: Option<u8>
}

impl Default for RTNoiseLevel {
    fn default() -> Self {
        RTNoiseLevel {
            name: "Noise level".to_owned(),
            value: None,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
struct MinMZPeakWidth{
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "$text")]
    value: Option<u8>
}

impl Default for MinMZPeakWidth {
    fn default() -> Self {
        MinMZPeakWidth {
            name: "Min m/z peak width".to_owned(),
            value: None,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
struct MaxMZPeakWidth{
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "$text")]
    value: Option<u8>
}

impl Default for MaxMZPeakWidth {
    fn default() -> Self {
        MaxMZPeakWidth {
            name: "Max m/z peak width".to_owned(),
            value: None,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
struct WaveletTransform{
    #[serde(rename = "@name")]
    name: String,

    parameter: Vec<WaveletTransformParameters>,
}
impl Default for WaveletTransform {
    fn default() -> Self {
        WaveletTransform {
            name: "Local maxima".to_owned(),
            parameter: vec![
                WaveletTransformParameters::WTNoiseLevel(RTNoiseLevel::default()),
                WaveletTransformParameters::ScaleLevel(ScaleLevel::default()),
                WaveletTransformParameters::WaveletWindowSize(WaveletWindowSize::default())
            ],
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
enum WaveletTransformParameters{
    WTNoiseLevel(RTNoiseLevel),
    ScaleLevel(ScaleLevel),
    WaveletWindowSize(WaveletWindowSize),
}

#[derive(Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
struct WTNoiseLevel{
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "$text")]
    value: Option<u8>
}

impl Default for WTNoiseLevel {
    fn default() -> Self {
        WTNoiseLevel {
            name: "Noise level".to_owned(),
            value: None,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
struct ScaleLevel{
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "$text")]
    value: Option<u8>
}

impl Default for ScaleLevel {
    fn default() -> Self {
        ScaleLevel {
            name: "Scale level".to_owned(),
            value: None,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
struct WaveletWindowSize{
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "$text")]
    value: Option<u8>
}

impl Default for WaveletWindowSize {
    fn default() -> Self {
        WaveletWindowSize {
            name: "Wavelet window size (%)".to_owned(),
            value: None,
        }
    }
}
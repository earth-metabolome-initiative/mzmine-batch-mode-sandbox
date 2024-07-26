use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize, PartialEq)]
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

    pub fn add_parameter(&mut self, parameter: Parameter){
        self.parameters.push(parameter);
    }

    pub fn get_parameter_value(&self, parameter:Parameter){

    }
}

#[derive(Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
enum Parameter{
    ScanNumber(ScanNumber),
    BaseFilteringInteger(BaseFilteringInteger),
    RetentionTime(RetentionTime),
    Mobility(Mobility),
    MSLevelFiler(MSLevelFilter),
    ScanDefinition(ScanDefinition),
    Polarity(Polarity),
    SpectrumType(SpectrumType)
}

#[derive(Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
struct ScanNumber{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "$text")]
    value: Option<u16>,
}

impl ScanNumber{
    fn new() -> Self{
        ScanNumber { 
            name: "Scan number".to_owned(),
            value: None 
        }
    }

    fn set_value(&mut self, value: Option<u16>){
        self.value = value;
    }

    fn get_value(&self) -> Option<u16>{
        self.value
    }
}


#[derive(Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
struct BaseFilteringInteger{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "$text")]
    value: Option<u16>,
}

impl BaseFilteringInteger{
    fn new() -> Self{
        BaseFilteringInteger { 
            name: "Base Filtering Integer".to_owned(),
            value: None 
        }
    }

    fn set_value(&mut self, value: Option<u16>){
        self.value = value;
    }

    fn get_value(&self) -> Option<u16>{
        self.value
    }
}

#[derive(Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
struct RetentionTime{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "$text")]
    value: Option<u16>,
}

impl RetentionTime{
    fn new() -> Self{
        RetentionTime { 
            name: "Retention time".to_owned(),
            value: None 
        }
    }

    fn set_value(&mut self, value: Option<u16>){
        self.value = value;
    }

    fn get_value(&self) -> Option<u16>{
        self.value
    }
}


#[derive(Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
struct Mobility{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "$text")]
    value: Option<u16>,
}

impl Mobility{
    fn new() -> Self{
        Mobility { 
            name: "Mobility".to_owned(),
            value: None 
        }
    }

    fn set_value(&mut self, value: Option<u16>){
        self.value = value;
    }

    fn get_value(&self) -> Option<u16>{
        self.value
    }
}

#[derive(Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
struct MSLevelFilter{
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
    fn new() -> Self{
        MSLevelFilter { 
            name: "MS level filter".to_owned(),
            selected: true,
            level: "None selected".to_owned(),
            value: None 
        }
    }

    fn set_level(&mut self, level:String){
        self.level = level;
    }

    fn get_level(&self) -> String {
        self.level.clone()
    }

    fn set_value(&mut self, value: Option<u16>){
        self.value = value;
    }

    fn get_value(&self) -> Option<u16>{
        self.value
    }

    fn set_ms1(&mut self, value:Option<u16>){
        self.set_level("MS1, level = 1".to_owned());
        self.set_value(value);
    }

    fn set_ms2(&mut self, value:Option<u16>){
        self.set_level("MSn, level ≥ 2".to_owned());
        self.set_value(value);
    }
}


#[derive(Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
struct ScanDefinition{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "$text")]
    value: Option<u16>,
}

impl ScanDefinition{
    fn new() -> Self{
        ScanDefinition { 
            name: "Scan definition".to_owned(),
            value: None 
        }
    }

    fn set_value(&mut self, value: Option<u16>){
        self.value = value;
    }

    fn get_value(&self) -> Option<u16>{
        self.value
    }
}

#[derive(Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
struct Polarity{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "$text")]
    value: String,
}

impl Polarity{
    fn new() -> Self{
        Polarity { 
            name: "Polarity".to_owned(),
            value: "Any".to_owned(), 
        }
    }

    fn set_value(&mut self, value: String){
        self.value = value;
    }

    fn get_value(&self) -> String{
        self.value.clone()
    }
}

#[derive(Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
struct SpectrumType{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "$text")]
    value: String,
}

impl SpectrumType{
    fn new() -> Self{
        SpectrumType { 
            name: "Spectrum type".to_owned(),
            value: "ANY".to_owned(), 
        }
    }

    fn set_value(&mut self, value: String){
        self.value = value;
    }

    fn get_value(&self) -> String{
        self.value.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_filters_initialization(){
        let scan_filters_obj = ScanFilters::new();
        assert_eq!(scan_filters_obj.name, "Scan filters");
        assert_eq!(scan_filters_obj.parameters.len(), 0);
    }

    #[test]
    fn test_scan_filters_add_parameter(){
        let mut scan_filters_obj = ScanFilters::new();
        assert_eq!(scan_filters_obj.parameters.len(), 0);
        scan_filters_obj.add_parameter(Parameter::Mobility(Mobility::new()));
        assert_eq!(scan_filters_obj.parameters.len(), 1);
    }

    #[test]
    fn test_scan_number_initialization(){
        let scan_number_obj = ScanNumber::new();
        assert_eq!(scan_number_obj.name, "Scan number");
        assert_eq!(scan_number_obj.value, None);
    }

    #[test]
    fn test_scan_number_set_value(){
        let mut scan_number_obj = ScanNumber::new();
        assert_eq!(scan_number_obj.value, None);
        scan_number_obj.set_value(Some(2));
        assert_eq!(scan_number_obj.value, Some(2));
    }

    #[test]
    fn test_get_value(){
        let mut scan_number_obj = ScanNumber::new();
        assert_eq!(scan_number_obj.value, None);
        scan_number_obj.set_value(Some(3));
        assert_eq!(scan_number_obj.get_value(), Some(3)); 
    }

    // // // 

    #[test]
    fn test_base_filtering_integer_initialization(){
        let base_filtering_integer_obj = BaseFilteringInteger::new();
        assert_eq!(base_filtering_integer_obj.name, "Base Filtering Integer");
        assert_eq!(base_filtering_integer_obj.value, None);
    }

    #[test]
    fn test_base_filtering_integer_set_value(){
        let mut base_filtering_integer_obj = BaseFilteringInteger::new();
        assert_eq!(base_filtering_integer_obj.value, None);
        base_filtering_integer_obj.set_value(Some(2));
        assert_eq!(base_filtering_integer_obj.value, Some(2));
    }
    
    #[test]
    fn test_base_filtering_integer_get_value(){
        let mut base_filtering_integer_obj = BaseFilteringInteger::new();
        assert_eq!(base_filtering_integer_obj.value, None);
        base_filtering_integer_obj.set_value(Some(3));
        assert_eq!(base_filtering_integer_obj.get_value(), Some(3)); 
    }

    // // //

    #[test]
    fn test_retention_time_initialization(){
        let retention_time_obj = RetentionTime::new();
        assert_eq!(retention_time_obj.name, "Retention time");
        assert_eq!(retention_time_obj.value, None);
    }

    #[test]
    fn test_retention_time_set_value(){
        let mut retention_time_obj = RetentionTime::new();
        assert_eq!(retention_time_obj.value, None);
        retention_time_obj.set_value(Some(2));
        assert_eq!(retention_time_obj.value, Some(2));
    }
    
    #[test]
    fn test_retention_time_get_value(){
        let mut retention_time_obj = RetentionTime::new();
        assert_eq!(retention_time_obj.value, None);
        retention_time_obj.set_value(Some(3));
        assert_eq!(retention_time_obj.get_value(), Some(3)); 
    }

    // // //

    #[test]
    fn test_mobility_initialization(){
        let mobility_obj = Mobility::new();
        assert_eq!(mobility_obj.name, "Mobility");
        assert_eq!(mobility_obj.value, None);
    }

    #[test]
    fn test_mobility_set_value(){
        let mut mobility_obj = Mobility::new();
        assert_eq!(mobility_obj.value, None);
        mobility_obj.set_value(Some(2));
        assert_eq!(mobility_obj.value, Some(2));
    }
    
    #[test]
    fn test_mobility_get_value(){
        let mut mobility_obj = Mobility::new();
        assert_eq!(mobility_obj.value, None);
        mobility_obj.set_value(Some(3));
        assert_eq!(mobility_obj.get_value(), Some(3)); 
    }

    // // //

    #[test]
    fn test_ms_level_initialization(){
        let ms_level_obj = MSLevelFilter::new();
        assert_eq!(ms_level_obj.name, "MS level filter");
        assert_eq!(ms_level_obj.value, None);
    }

    #[test]
    fn test_ms_level_set_value(){
        let mut ms_level_obj = MSLevelFilter::new();
        assert_eq!(ms_level_obj.value, None);
        ms_level_obj.set_value(Some(2));
        assert_eq!(ms_level_obj.value, Some(2));
    }
    
    #[test]
    fn test_ms_level_get_value(){
        let mut ms_level_obj = MSLevelFilter::new();
        assert_eq!(ms_level_obj.value, None);
        ms_level_obj.set_value(Some(3));
        assert_eq!(ms_level_obj.get_value(), Some(3)); 
    }

    #[test]
    fn test_ms_level_set_ms1(){
        let mut ms_level_obj = MSLevelFilter::new();
        assert_eq!(ms_level_obj.level, "None selected");
        ms_level_obj.set_ms1(Some(4));
        assert_eq!(ms_level_obj.level, "MS1, level = 1");
        assert_eq!(ms_level_obj.value, Some(4));
    }

    #[test]
    fn test_ms_level_set_ms2(){
        let mut ms_level_obj = MSLevelFilter::new();
        assert_eq!(ms_level_obj.level, "None selected");
        ms_level_obj.set_ms2(Some(3));
        assert_eq!(ms_level_obj.level, "MSn, level ≥ 2");
        assert_eq!(ms_level_obj.value, Some(3));
    }

    #[test]
    fn test_ms_level_get_level(){
        let mut ms_level_obj = MSLevelFilter::new();
        assert_eq!(ms_level_obj.level, "None selected");
        ms_level_obj.set_ms1(Some(4));
        assert_eq!(ms_level_obj.get_level(), "MS1, level = 1");
    }

    // // //

    #[test]
    fn test_scan_definition_initialization(){
        let scan_definition_obj = ScanDefinition::new();
        assert_eq!(scan_definition_obj.name, "Scan definition");
        assert_eq!(scan_definition_obj.value, None);
    }

    #[test]
    fn test_scan_definition_set_value(){
        let mut scan_definition_obj = ScanDefinition::new();
        assert_eq!(scan_definition_obj.value, None);
        scan_definition_obj.set_value(Some(2));
        assert_eq!(scan_definition_obj.value, Some(2));
    }
    
    #[test]
    fn test_scan_definition_get_value(){
        let mut scan_definition_obj = ScanDefinition::new();
        assert_eq!(scan_definition_obj.value, None);
        scan_definition_obj.set_value(Some(3));
        assert_eq!(scan_definition_obj.get_value(), Some(3)); 
    }

    // // // 

    #[test]
    fn test_polarity_initalization(){
        let polarity_obj = Polarity::new();
        assert_eq!(polarity_obj.name, "Polarity");
        assert_eq!(polarity_obj.value, "Any");
    }

    #[test]
    fn test_polarity_get_value(){
        let polarity_obj = Polarity::new();
        assert_eq!(polarity_obj.get_value(), polarity_obj.value);
    }

    #[test]
    fn test_polarity_set_value(){
        let mut polarity_obj = Polarity::new();
        assert_eq!(polarity_obj.value, "Any");
        polarity_obj.set_value("Another".to_owned());
        assert_eq!(polarity_obj.value, "Another");
    }

    // // // 

    #[test]
    fn test_spectrum_type_initalization(){
        let spectrum_type_obj = SpectrumType::new();
        assert_eq!(spectrum_type_obj.name, "Spectrum type");
        assert_eq!(spectrum_type_obj.value, "ANY");
    }

    #[test]
    fn test_spectrum_type_get_value(){
        let spectrum_type_obj = SpectrumType::new();
        assert_eq!(spectrum_type_obj.get_value(), spectrum_type_obj.value);
    }

    #[test]
    fn test_spectrum_type_set_value(){
        let mut spectrum_type_obj = SpectrumType::new();
        assert_eq!(spectrum_type_obj.value, "ANY");
        spectrum_type_obj.set_value("Another".to_owned());
        assert_eq!(spectrum_type_obj.value, "Another");
    }
}
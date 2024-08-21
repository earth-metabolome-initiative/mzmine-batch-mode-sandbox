use serde::{Serialize, Deserialize};

use quick_xml::events::{Event, BytesEnd, BytesStart, BytesText};
use quick_xml::writer::Writer;
use std::io::{Cursor, Result as IoResult, Error as IoError, ErrorKind};

#[derive(Default, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(default, rename_all = "lowercase")]
pub struct ScanFilters {
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "@selected")]
    selected: bool,
    
    parameters: Vec<Parameter>,
}

impl ScanFilters {
    pub fn new() -> Self{
        ScanFilters { 
            name: "Scan filters".to_owned(), 
            selected: true,
            parameters: Vec::new()
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn get_selected(&self) -> &bool{
        &self.selected
    }

    pub fn set_selected(&mut self, selected: bool){
        self.selected = selected;
    }

    pub fn get_selected_as_str(&self) -> &str {
        if self.selected {
            "true"
        } else {
            "false"
        }
    }

    pub fn add_parameter(&mut self, parameter: Parameter){
        self.parameters.push(parameter);
    }

    pub fn get_parameters_length(&self) -> usize{
        self.parameters.len()
    }

    pub fn get_parameter(&mut self, target:&str) -> Option<&Parameter>{
        for parameter in &mut self.parameters{
            match parameter{
                Parameter::ScanNumber(_f) if target == "Scan number" => return Some(parameter),
                Parameter::BaseFilteringInteger(_f) if target == "Base filtering integer" => return Some(parameter),
                Parameter::RetentionTime(_f) if target == "Retention time" => return Some(parameter),
                Parameter::Mobility(_f) if target == "Mobility" => return Some(parameter),
                Parameter::MSLevelFiler(_f) if target == "MS level filter" => return Some(parameter),
                Parameter::ScanDefinition(_f) if target == "Scan definition" => return Some(parameter),
                Parameter::Polarity(_f) if target == "Polarity" => return Some(parameter),
                Parameter::SpectrumType(_f) if target == "Spectrum type" => return Some(parameter),
                _ => panic!("No matching parameter")
            }
        }
        None
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

impl Parameter{
    // getters / setters
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

    // Has to return a String since &str is a reference to a temporary value, therefore it would be a reference to nothing but implied to store our value
    pub fn value_to_string(&self) -> String {
        match self.get_value() {
            Some(v) => v.to_string(),
            None => String::from("None"),
        }
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

    // Has to return a String since &str is a reference to a temporary value, therefore it would be a reference to nothing but implied to store our value
    pub fn value_to_string(&self) -> String {
        match self.get_value() {
            Some(v) => v.to_string(),
            None => String::from("None"),
        }
    }

    pub fn write_element(&self, writer: &mut Writer<Cursor<Vec<u8>>>) -> IoResult<()> {
        // create XML element -> istantiate element name (batch/batchstep/parameter/module)
        let mut element = BytesStart::new("parameter");

        // add the attribute(tag) to the element
        element.push_attribute(("name", self.name.as_str()));
        
        // add the value as text
        writer.write_event(Event::Start(element))
            .map_err(|e| IoError::new(ErrorKind::Other, e.to_string()))?;

        // close the XML element
        if let Some(ref value) = self.value {
            writer.write_event(Event::Text(BytesText::new(&self.value_to_string().as_str())))
                .map_err(|e| IoError::new(ErrorKind::Other, e.to_string()))?;
        }

        // Write the end tag
        writer.write_event(Event::End(BytesEnd::new("parameter")))
            .map_err(|e| IoError::new(ErrorKind::Other, e.to_string()))?;

        Ok(())
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

    // Has to return a String since &str is a reference to a temporary value, therefore it would be a reference to nothing but implied to store our value
    pub fn value_to_string(&self) -> String {
        match self.get_value() {
            Some(v) => v.to_string(),
            None => String::from("None"),
        }
    }

    pub fn write_element(&self, writer: &mut Writer<Cursor<Vec<u8>>>) -> IoResult<()> {
        // create XML element -> istantiate element name (batch/batchstep/parameter/module)
        let mut element = BytesStart::new("parameter");

        // add the attribute(tag) to the element
        element.push_attribute(("name", self.name.as_str()));
        
        // add the value as text
        writer.write_event(Event::Start(element))
            .map_err(|e| IoError::new(ErrorKind::Other, e.to_string()))?;

        // close the XML element
        if let Some(ref value) = self.value {
            writer.write_event(Event::Text(BytesText::new(&self.value_to_string().as_str())))
                .map_err(|e| IoError::new(ErrorKind::Other, e.to_string()))?;
        }

        // Write the end tag
        writer.write_event(Event::End(BytesEnd::new("parameter")))
            .map_err(|e| IoError::new(ErrorKind::Other, e.to_string()))?;

        Ok(())
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

    // Has to return a String since &str is a reference to a temporary value, therefore it would be a reference to nothing but implied to store our value
    pub fn value_to_string(&self) -> String {
        match self.get_value() {
            Some(v) => v.to_string(),
            None => String::from("None"),
        }
    }

    pub fn write_element(&self, writer: &mut Writer<Cursor<Vec<u8>>>) -> IoResult<()> {
        // create XML element -> istantiate element name (batch/batchstep/parameter/module)
        let mut element = BytesStart::new("parameter");

        // add the attribute(tag) to the element
        element.push_attribute(("name", self.name.as_str()));
        
        // add the value as text
        writer.write_event(Event::Start(element))
            .map_err(|e| IoError::new(ErrorKind::Other, e.to_string()))?;

        // close the XML element
        if let Some(ref value) = self.value {
            writer.write_event(Event::Text(BytesText::new(&self.value_to_string().as_str())))
                .map_err(|e| IoError::new(ErrorKind::Other, e.to_string()))?;
        }

        // Write the end tag
        writer.write_event(Event::End(BytesEnd::new("parameter")))
            .map_err(|e| IoError::new(ErrorKind::Other, e.to_string()))?;

        Ok(())
    }
}

#[derive(Default, Serialize, Deserialize, PartialEq, Debug, Clone)]
#[serde(default, rename_all = "lowercase")]
pub struct MSLevelFilter{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "@level")]
    selected: String,

    #[serde(rename = "$text")]
    value: Option<u16>,
}

impl MSLevelFilter{
    pub fn new() -> Self{
        MSLevelFilter { 
            name: "MS level filter".to_owned(),
            selected: "None selected".to_owned(),
            value: None 
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn set_selected(&mut self, selected:&str){
        self.selected = selected.to_owned();
    }

    pub fn get_selected(&self) -> &str {
        &self.selected
    }

    pub fn set_value(&mut self, value: Option<u16>){
        self.value = value;
    }

    pub fn get_value(&self) -> Option<u16>{
        self.value
    }

    pub fn set_ms1(&mut self, value:Option<u16>){
        self.set_selected("MS1, level = 1");
        self.set_value(value);
    }

    pub fn set_ms2(&mut self, value:Option<u16>){
        self.set_selected("MSn, level ≥ 2");
        self.set_value(value);
    }

    // Has to return a String since &str is a reference to a temporary value, therefore it would be a reference to nothing but implied to store our value
    pub fn value_to_string(&self) -> String {
        match self.get_value() {
            Some(v) => v.to_string(),
            None => String::from("None"),
        }
    }

    pub fn write_element(&self, writer: &mut Writer<Cursor<Vec<u8>>>) -> IoResult<()> {
        // create XML element -> istantiate element name (batch/batchstep/parameter/module)
        let mut element = BytesStart::new("parameter");

        // add the attribute(tag) to the element
        element.push_attribute(("name", self.name.as_str()));

        // add the attribute(tag) to the element
        element.push_attribute(("selected", self.get_selected()));
        
        // add the value as text
        writer.write_event(Event::Start(element))
            .map_err(|e| IoError::new(ErrorKind::Other, e.to_string()))?;

        // close the XML element
        if let Some(ref value) = self.value {
            writer.write_event(Event::Text(BytesText::new(&self.value_to_string().as_str())))
                .map_err(|e| IoError::new(ErrorKind::Other, e.to_string()))?;
        }

        // Write the end tag
        writer.write_event(Event::End(BytesEnd::new("parameter")))
            .map_err(|e| IoError::new(ErrorKind::Other, e.to_string()))?;

        Ok(())
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

    // Has to return a String since &str is a reference to a temporary value, therefore it would be a reference to nothing but implied to store our value
    pub fn value_to_string(&self) -> String {
        match self.get_value() {
            Some(v) => v.to_string(),
            None => String::from("None"),
        }
    }

    pub fn write_element(&self, writer: &mut Writer<Cursor<Vec<u8>>>) -> IoResult<()> {
        // create XML element -> istantiate element name (batch/batchstep/parameter/module)
        let mut element = BytesStart::new("parameter");

        // add the attribute(tag) to the element
        element.push_attribute(("name", self.name.as_str()));
        
        // add the value as text
        writer.write_event(Event::Start(element))
            .map_err(|e| IoError::new(ErrorKind::Other, e.to_string()))?;

        // close the XML element
        if let Some(ref value) = self.value {
            writer.write_event(Event::Text(BytesText::new(&self.value_to_string().as_str())))
                .map_err(|e| IoError::new(ErrorKind::Other, e.to_string()))?;
        }

        // Write the end tag
        writer.write_event(Event::End(BytesEnd::new("parameter")))
            .map_err(|e| IoError::new(ErrorKind::Other, e.to_string()))?;

        Ok(())
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

    pub fn set_value(&mut self, value: &str){
        self.value = value.to_owned();
    }

    pub fn get_value(&self) -> &str{
        &self.value
    }

    pub fn write_element(&self, writer: &mut Writer<Cursor<Vec<u8>>>) -> IoResult<()> {
        // create XML element -> istantiate element name (batch/batchstep/parameter/module)
        let mut element = BytesStart::new("parameter");

        // add the attribute(tag) to the element
        element.push_attribute(("name", self.name.as_str()));
        
        // add the value as text
        writer.write_event(Event::Start(element))
            .map_err(|e| IoError::new(ErrorKind::Other, e.to_string()))?;

        if !self.value.is_empty(){
            writer.write_event(Event::Text(BytesText::new(self.get_value())))
                .map_err(|e| IoError::new(ErrorKind::Other, e.to_string()))?;
        }

        // Write the end tag
        writer.write_event(Event::End(BytesEnd::new("parameter")))
            .map_err(|e| IoError::new(ErrorKind::Other, e.to_string()))?;

        Ok(())
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

    pub fn set_value(&mut self, value: &str){
        self.value = value.to_owned();
    }

    pub fn get_value(&self) -> &str{
        &self.value
    }

    pub fn write_element(&self, writer: &mut Writer<Cursor<Vec<u8>>>) -> IoResult<()> {
        // create XML element -> istantiate element name (batch/batchstep/parameter/module)
        let mut element = BytesStart::new("parameter");

        // add the attribute(tag) to the element
        element.push_attribute(("name", self.name.as_str()));
        
        // add the value as text
        writer.write_event(Event::Start(element))
            .map_err(|e| IoError::new(ErrorKind::Other, e.to_string()))?;

        // close the XML element
        if !self.value.is_empty(){
            writer.write_event(Event::Text(BytesText::new(self.get_value())))
                .map_err(|e| IoError::new(ErrorKind::Other, e.to_string()))?;
        }

        // Write the end tag
        writer.write_event(Event::End(BytesEnd::new("parameter")))
            .map_err(|e| IoError::new(ErrorKind::Other, e.to_string()))?;

        Ok(())
    }
}
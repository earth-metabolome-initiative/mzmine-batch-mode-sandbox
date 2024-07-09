use std::ops::Sub;

use simple_xml_builder::XMLElement;
use quick_xml;
use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize)]
pub struct AllSpectralDataImportModule{
    #[serde(rename = "method")]
    method: String,
    #[serde(rename = "parameter_version")]
    parameter_version: String,
    parameter: Vec<Parameter>,
}

#[derive(Default, Serialize, Deserialize)]
pub struct Parameter{
    #[serde(rename = "name")]
    name: String,

    #[serde(rename = "$value")]
    value: Option<String>,

    #[serde(rename = "parameter", skip_serializing_if = "Option::is_none")]
    nested_parameters: Option<Vec<SubParameters>>,

    #[serde(rename = "file", skip_serializing_if = "Option::is_none")]
    files: Option<Vec<String>>,

    #[serde(rename = "module", skip_serializing_if = "Option::is_none")]
    modules: Option<Vec<Module>>,
}

#[derive(Default, Serialize, Deserialize)]
pub struct SubParameters{
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "$value")]
    value: String,
    #[serde(rename = "parameter", skip_serializing_if = "Option::is_none")]
    nested_parameters: Option<Vec<SubParameters>>,
}

#[derive(Default, Serialize, Deserialize)]
pub struct Module{
    #[serde(rename = "name")]
    name: String,
    #[serde(rename = "parameter", skip_serializing_if = "Option::is_none")]
    parameters: Option<Vec<SubParameters>>,
}
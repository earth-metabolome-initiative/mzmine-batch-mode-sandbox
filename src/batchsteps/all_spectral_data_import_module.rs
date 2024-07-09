use simple_xml_builder::XMLElement;
use quick_xml;
use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize)]
pub struct AllSpectralDataImportModule{
    selected: bool,

    // list of all input files for analysis
    File_names: Vec<String>,
}
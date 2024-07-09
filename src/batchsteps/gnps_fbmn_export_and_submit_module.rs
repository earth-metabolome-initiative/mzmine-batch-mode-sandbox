use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize)]
pub struct GnpsFbmnExportAndSubmitModule{
    selected: bool,
}
use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize)]
pub struct SiriusExportModule{
    selected: bool,
}
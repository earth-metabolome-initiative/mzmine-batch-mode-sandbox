use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize)]
pub struct MassDetectionModule{
    selected: bool,
}
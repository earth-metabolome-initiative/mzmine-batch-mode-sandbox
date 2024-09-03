use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename = "parameter")]
pub struct Parameter {
    #[serde(rename = "@name")]
    name: String,

    #[serde(skip_serializing_if = "Option::is_none", rename = "@selected")]
    selected: Option<bool>,

    #[serde(skip_serializing_if = "String::is_empty", rename = "$text")]
    value: String,
}

impl Parameter {
    pub fn new(name: &str, selected: Option<bool>, value: &str) -> Self {
        Parameter {
            name: name.to_owned(),
            selected,
            value: value.to_owned(),
        }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn set_name(&mut self, name: &str) {
        self.name = name.to_owned();
    }

    pub fn is_selected(&self) -> &Option<bool> {
        &self.selected
    }

    pub fn select(&mut self) {
        self.selected = Some(true);
    }

    pub fn deselect(&mut self) {
        self.selected = Some(false);
    }

    pub fn get_value(&self) -> &str {
        &self.value
    }

    pub fn set_value(&mut self, value: &str) {
        self.value = value.to_owned();
    }
}

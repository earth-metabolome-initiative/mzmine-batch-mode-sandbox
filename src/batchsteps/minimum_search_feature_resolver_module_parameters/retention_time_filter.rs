use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize)]
#[serde(default, rename_all = "lowercase")]
pub struct RetentionTimeFilter{
    #[serde(rename="@name")]
    name: String,

    #[serde(rename="@selected")]
    selected: String,

    #[serde(rename="@unit")]
    unit: String,

    #[serde(rename="$text")]
    value: Option<f32>,
}

impl RetentionTimeFilter{
    pub fn new() -> Self{
        RetentionTimeFilter{
            name: "Retention time filter".to_owned(),
            selected: "Use feature edges".to_owned(),
            unit: "MINUTES".to_owned(),
            value: None,
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn get_selected(&self) -> &str{
        &self.selected
    }

    pub fn set_selected(&mut self, selection:&str){
        self.selected = selection.to_owned();
    }

    pub fn get_unit(&self) -> &str{
        &self.unit
    }

    pub fn set_unit(&mut self, unit:&str){
        self.unit = unit.to_owned();
    }

    pub fn get_value(&self) -> &Option<f32>{
        &self.value
    }

    pub fn set_value(&mut self, value:Option<f32>){
        self.value = value;
    }

}
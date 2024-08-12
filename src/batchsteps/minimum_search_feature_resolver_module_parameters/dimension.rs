use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
pub struct Dimension{
    #[serde(rename="@name")]
    name: String,

    #[serde(rename = "$text")]
    value: String,
}

impl Dimension{
    pub fn new() -> Self{
        Dimension{
            name: "Dimension".to_owned(),
            value: "Retention time".to_owned(),
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn set_value(&mut self, value:&str){
        self.value = value.to_owned();
    }

    pub fn get_value(&self) -> &str{
        &self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suffix_initialization(){
        let dimension_obj = Dimension::new();
        assert_eq!(dimension_obj.get_name(), "Dimension");
        assert_eq!(dimension_obj.get_value(), "Retention time");
    }

    #[test]
    fn test_suffix_set_get_value(){
        let mut dimension_obj = Dimension::new();
        assert_eq!(dimension_obj.get_value(), "Retention time");
        dimension_obj.set_value("TEST");
        assert_eq!(dimension_obj.get_value(), "TEST");
    }
}
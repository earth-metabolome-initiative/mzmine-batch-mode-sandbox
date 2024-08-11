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

    pub fn set_value(&mut self, value:String){
        self.value = value;
    }

    pub fn get_value(&self) -> String{
        self.value.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suffix_initialization(){
        let dimension_obj = Dimension::new();
        assert_eq!(dimension_obj.name, "Dimension");
        assert_eq!(dimension_obj.value, "Retention time");
    }

    #[test]
    fn test_suffix_set_value(){
        let mut dimension_obj = Dimension::new();
        assert_eq!(dimension_obj.value, "Retention time");
        dimension_obj.set_value("TEST".to_owned());
        assert_eq!(dimension_obj.value, "TEST");
    }

    #[test]
    fn test_suffix_get_value(){
        let mut dimension_obj = Dimension::new();
        assert_eq!(dimension_obj.value, "Retention time");
        dimension_obj.value = "TEST".to_owned();
        assert_eq!(dimension_obj.get_value(), "TEST");
    }
}
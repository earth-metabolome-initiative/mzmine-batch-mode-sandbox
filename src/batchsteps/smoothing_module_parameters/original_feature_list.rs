use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
pub struct OriginalFeatureList{
    #[serde(rename="@name")]
    name: String,

    #[serde(rename="$text")]
    value: String,
}

impl OriginalFeatureList{
    pub fn new() -> Self{
        OriginalFeatureList{
            name: "Original feature list".to_owned(),
            value: "KEEP".to_owned()
        }
    }

    pub fn get_value(&self) -> String{
        self.value.clone()
    }

    pub fn set_value(&mut self, value:String){
        self.value = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ofl_initialization(){
        let test_ofl_obj = OriginalFeatureList::new();
        assert_eq!{test_ofl_obj.name, "Original feature list"};
        assert_eq!{test_ofl_obj.value, "KEEP"};
    }

    #[test]
    fn test_ofl_get_value(){
        let mut test_ofl_obj = OriginalFeatureList::new();
        assert_eq!(test_ofl_obj.value, "KEEP");
        test_ofl_obj.value = "VALUE".to_owned();
        assert_eq!(test_ofl_obj.get_value(), "VALUE");
    }

    #[test]
    fn test_ofl_set_value(){
        let mut test_ofl_obj = OriginalFeatureList::new();
        assert_eq!(test_ofl_obj.value, "KEEP");
        test_ofl_obj.set_value("VALUE".to_owned());
        assert_eq!(test_ofl_obj.value, "VALUE");
    }
}

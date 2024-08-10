use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize)]
#[serde(default, rename_all = "lowercase")]
pub struct OriginalFeatureList{
    #[serde(rename="@name")]
    name: String,

    #[serde(rename = "$text")]
    value: String,
}

impl OriginalFeatureList{
    pub fn new() -> Self{
        OriginalFeatureList{
            name: "Suffix".to_owned(),
            value: "KEEP".to_owned(),
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
        let suffix_obj = OriginalFeatureList::new();
        assert_eq!(suffix_obj.name, "Suffix");
        assert_eq!(suffix_obj.value, "KEEP");
    }

    #[test]
    fn test_suffix_set_value(){
        let mut suffix_obj = OriginalFeatureList::new();
        assert_eq!(suffix_obj.value, "KEEP");
        suffix_obj.set_value("TEST".to_owned());
        assert_eq!(suffix_obj.value, "TEST");
    }

    #[test]
    fn test_suffix_get_value(){
        let mut suffix_obj = OriginalFeatureList::new();
        assert_eq!(suffix_obj.value, "KEEP");
        suffix_obj.value = "TEST".to_owned();
        assert_eq!(suffix_obj.get_value(), "TEST");
    }
}
use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
pub struct Suffix{
    #[serde(rename="@name")]
    name: String,

    #[serde(rename="$text")]
    value: String,
}

impl Suffix{
    pub fn new() -> Self{
        Suffix{
            name: "Suffix".to_owned(),
            value: "sm".to_owned()
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
    fn test_suffix_initialization(){
        let suffix_obj = Suffix::new();
        assert_eq!{suffix_obj.name, "Suffix"};
        assert_eq!{suffix_obj.value, "sm"};
    }

    #[test]
    fn test_suffix_get_value(){
        let mut suffix_obj = Suffix::new();
        assert_eq!(suffix_obj.value, "sm");
        suffix_obj.value = "VALUE".to_owned();
        assert_eq!(suffix_obj.get_value(), "VALUE");
    }

    #[test]
    fn test_suffix_set_value(){
        let mut suffix_obj = Suffix::new();
        assert_eq!(suffix_obj.value, "sm");
        suffix_obj.set_value("VALUE".to_owned());
        assert_eq!(suffix_obj.value, "VALUE");
    }
}

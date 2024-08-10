use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize)]
#[serde(default, rename_all = "lowercase")]
pub struct Suffix{
    #[serde(rename="@name")]
    name: String,

    #[serde(rename = "$text")]
    value: char,
}

impl Suffix{
    pub fn new() -> Self{
        Suffix{
            name: "Suffix".to_owned(),
            value: 'r',
        }
    }

    pub fn set_value(&mut self, value:char){
        self.value = value;
    }

    pub fn get_value(&self) -> char{
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suffix_initialization(){
        let suffix_obj = Suffix::new();
        assert_eq!(suffix_obj.name, "Suffix");
        assert_eq!(suffix_obj.value, 'r');
    }

    #[test]
    fn test_suffix_set_value(){
        let mut suffix_obj = Suffix::new();
        assert_eq!(suffix_obj.value, 'r');
        suffix_obj.set_value('f');
        assert_eq!(suffix_obj.value, 'f');
    }

    #[test]
    fn test_suffix_get_value(){
        let mut suffix_obj = Suffix::new();
        assert_eq!(suffix_obj.value, 'r');
        suffix_obj.value = 'p';
        assert_eq!(suffix_obj.get_value(), 'p');
    }
}
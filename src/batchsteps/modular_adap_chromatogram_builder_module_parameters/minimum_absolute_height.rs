use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
pub struct MinimumAbsoluteHeight{
    #[serde(rename="@name")]
    name: String,

    #[serde(rename="$text")]
    value: Option<f32>
}

impl MinimumAbsoluteHeight{
    pub fn new() -> Self{
        MinimumAbsoluteHeight{
            name: "Minimum absolute height".to_owned(),
            value: None,
        }
    }

    pub fn set_value(&mut self, value:Option<f32>){
        self.value = value;
    }

    pub fn get_value(&self) -> Option<f32>{
        self.value
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_absolute_height_initialization(){
        let minimum_obj = MinimumAbsoluteHeight::new();
        assert_eq!(minimum_obj.name, "Minimum absolute height");
        assert_eq!(minimum_obj.value, None);
    }

    #[test]
    fn test_minimum_absolute_height_get_value(){
        let mut minimum_obj = MinimumAbsoluteHeight::new();
        minimum_obj.value = Some(344.5);
        assert_eq!(minimum_obj.get_value(), Some(344.5));
    }

    #[test]
    fn test_minimum_absolute_height_set_value(){
        let mut minimum_obj = MinimumAbsoluteHeight::new();
        assert_eq!(minimum_obj.value, None);
        minimum_obj.set_value(Some(244.8));
        assert_eq!(minimum_obj.value, Some(244.8));
    }
}
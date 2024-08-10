use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize)]
#[serde(default, rename_all = "lowercase")]
pub struct MinimumRealtiveHeight{
    #[serde(rename="@name")]
    name: String,

    #[serde(rename = "@name")]
    value: Option<f32>,
}

impl MinimumRealtiveHeight{
    pub fn new() -> Self{
        MinimumRealtiveHeight{
            name: "Minimum relative height".to_owned(),
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
    fn test_min_rel_height_initialization(){
        let min_rel_height_obj = MinimumRealtiveHeight::new();
        assert_eq!(min_rel_height_obj.name, "Minimum relative height");
        assert_eq!(min_rel_height_obj.value, None);
    }

    #[test]
    fn test_min_rel_height_set_value(){
        let mut min_rel_height_obj = MinimumRealtiveHeight::new();
        assert_eq!(min_rel_height_obj.value, None);
        min_rel_height_obj.set_value(Some(13.8));
        assert_eq!(min_rel_height_obj.value, Some(13.8));
    }

    #[test]
    fn test_min_rel_height_get_value(){
        let mut min_rel_height_obj = MinimumRealtiveHeight::new();
        assert_eq!(min_rel_height_obj.value, None);
        min_rel_height_obj.value = Some(2.45);
        assert_eq!(min_rel_height_obj.get_value(), Some(2.45));
    }
}
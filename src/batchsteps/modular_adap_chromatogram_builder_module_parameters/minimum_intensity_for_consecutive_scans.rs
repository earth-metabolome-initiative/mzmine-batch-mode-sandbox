use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
pub struct MinimumIntensityForConsecutiveScans{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "$text")]
    value: Option<f32>,
}

impl MinimumIntensityForConsecutiveScans{
    pub fn new() -> Self{
        MinimumIntensityForConsecutiveScans{
            name: "Minimum intensity for consecutive scans".to_owned(),
            value: None,
        }
    }

    pub fn get_value(&self) -> Option<f32>{
        self.value
    }

    pub fn set_value(&mut self, value: Option<f32>){
        self.value = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_int_cons_scans_initialization(){
        let min_scans_obj = MinimumIntensityForConsecutiveScans::new();
        assert_eq!(min_scans_obj.name, "Minimum intensity for consecutive scans");
        assert_eq!(min_scans_obj.value, None);
    }

    #[test]
    fn test_min_int_cons_scans_set_value(){
        let mut min_scans_obj = MinimumIntensityForConsecutiveScans::new();
        assert_eq!(min_scans_obj.value, None);
        min_scans_obj.set_value(Some(31.0));
        assert_eq!(min_scans_obj.value, Some(31.0));
    }

    #[test]
    fn test_min_int_cons_scans_get_value(){
        let mut min_scans_obj = MinimumIntensityForConsecutiveScans::new();
        assert_eq!(min_scans_obj.value, None);
        min_scans_obj.value = Some(122.2);
        assert_eq!(min_scans_obj.get_value(), Some(122.2));
    }
}

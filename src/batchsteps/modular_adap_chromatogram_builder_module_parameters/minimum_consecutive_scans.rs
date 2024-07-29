use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
pub struct MinimumConsecutiveScans{
    #[serde(rename = "@name")]
    name: String,
    #[serde(rename = "$text")]
    value: Option<u8>,
}

impl MinimumConsecutiveScans{
    fn new() -> Self{
        MinimumConsecutiveScans{
            name: "Minimum consecutive scans".to_owned(),
            value: None,
        }
    }

    fn set_value(&mut self, value: Option<u8>){
        self.value = value;
    }

    fn get_value(&self) -> Option<u8>{
        self.value
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_consecutive_scans_initialization(){
        let mcs_obj = MinimumConsecutiveScans::new();
        assert_eq!(mcs_obj.name, "Minimum consecutive scans", "NOT right name initialized");
        assert_eq!(mcs_obj.value, None);
    }

    #[test]
    fn test_minimum_consecutive_scans_get_value(){
        let mut mcs_obj = MinimumConsecutiveScans::new();
        mcs_obj.value = Some(14);
        assert_eq!(mcs_obj.get_value(), mcs_obj.value);
    }

    #[test]
    fn test_minimum_consecutive_scans_set_value(){
        let mut mcs_obj = MinimumConsecutiveScans::new();
        mcs_obj.set_value(Some(36));
        assert_eq!(mcs_obj.value, Some(36));
    }

}

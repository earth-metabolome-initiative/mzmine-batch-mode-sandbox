use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
pub struct DenormalizeFragmentScanTraps {
    #[serde(rename = "@name")]
    name: String,
    
    value: bool,
}

impl DenormalizeFragmentScanTraps {
    pub fn new() -> Self{
        DenormalizeFragmentScanTraps { 
            name: "Denormalize fragment scans (traps)".to_owned(), 
            value: true,
        }
    }

    pub fn set_value(&mut self, value:bool){
        self.value = value;
    }

    pub fn get_value(&self) -> bool{
        self.value
    }

    pub fn switch_value(&mut self){
        self.value = !self.value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_denormalize_fragment_scan_traps_initialization(){
        let dfst_obj = DenormalizeFragmentScanTraps::new();
        assert_eq!(dfst_obj.name, "Denormalize fragment scans (traps)");
        assert_eq!(dfst_obj.value, true);
    }

    #[test]
    fn test_denormalize_fragment_scan_traps_set_value(){
        let mut dfst_obj = DenormalizeFragmentScanTraps::new();
        assert_eq!(dfst_obj.value, true);
        dfst_obj.set_value(false);
        assert_eq!(dfst_obj.value, false);
    }

    #[test]
    fn test_denormalize_fragment_scan_traps_switch_value(){
        let mut dfst_obj = DenormalizeFragmentScanTraps::new();
        assert_eq!(dfst_obj.value, true);
        dfst_obj.switch_value();
        assert_eq!(dfst_obj.value, false);
    }

    #[test]
    fn test_denormalize_fragment_scan_traps_get_value(){
        let mut dfst_obj = DenormalizeFragmentScanTraps::new();
        assert_eq!(dfst_obj.value, true);
        dfst_obj.value = false;
        assert_eq!(dfst_obj.get_value(), false);
    }
}
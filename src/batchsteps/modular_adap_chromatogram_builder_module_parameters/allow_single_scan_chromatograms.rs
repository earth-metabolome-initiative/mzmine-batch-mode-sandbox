use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
pub struct AllowSingleScanChromatograms{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "$text")]
    value: Option<f32>
}

impl AllowSingleScanChromatograms{
    fn new() -> Self{
        AllowSingleScanChromatograms{
            name: "Allow single scan chromatograms".to_owned(),
            value: None,
        }
    }

    pub fn get_value(&self) -> Option<f32>{
        self.value
    }

    pub fn set_value(&mut self, value:Option<f32>){
        self.value = value;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allow_single_scan_chr_initialization(){
        let a_s_s_chr_obj = AllowSingleScanChromatograms::new();
        assert_eq!(a_s_s_chr_obj.name, "Allow single scan chromatograms");
        assert_eq!(a_s_s_chr_obj.value, None);
    }

    #[test]
    fn test_allow_single_scan_chr_get_value(){
        let mut a_s_s_chr_obj = AllowSingleScanChromatograms::new();
        assert_eq!(a_s_s_chr_obj.value, None);
        a_s_s_chr_obj.value = Some(12.8);
        assert_eq!(a_s_s_chr_obj.get_value(), Some(12.8));
    }

    #[test]
    fn test_allow_single_scan_chr_set_value(){
        let mut a_s_s_chr_obj = AllowSingleScanChromatograms::new();
        assert_eq!(a_s_s_chr_obj.value, None);
        a_s_s_chr_obj.set_value(Some(34.8));
        assert_eq!(a_s_s_chr_obj.value, Some(34.8));
    }
}
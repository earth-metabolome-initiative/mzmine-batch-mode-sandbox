use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
pub struct ChromatographicThreshold{
    #[serde(rename="@name")]
    name: String,

    #[serde(rename = "@name")]
    value: Option<f32>,
}

impl ChromatographicThreshold{
    pub fn new() -> Self{
        ChromatographicThreshold{
            name: "Chromatographic threshold".to_owned(),
            value: None,
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn set_value(&mut self, value:Option<f32>){
        self.value = value;
    }

    pub fn get_value(&self) -> &Option<f32>{
        &self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chr_thr_initialization(){
        let chr_obj = ChromatographicThreshold::new();
        assert_eq!(chr_obj.get_name(), "Chromatographic threshold");
        assert_eq!(*chr_obj.get_value(), None);
    }

    #[test]
    fn test_chr_thr_set_get_value(){
        let mut chr_obj = ChromatographicThreshold::new();
        assert_eq!(*chr_obj.get_value(), None);
        chr_obj.set_value(Some(13.8));
        assert_eq!(*chr_obj.get_value(), Some(13.8));
    }
}
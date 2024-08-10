use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize)]
#[serde(default, rename_all = "lowercase")]
pub struct MinimumSearchRangeRTMobilityAbsolute{
    #[serde(rename="@name")]
    name: String,

    #[serde(rename = "@name")]
    value: Option<f32>,
}

impl MinimumSearchRangeRTMobilityAbsolute{
    pub fn new() -> Self{
        MinimumSearchRangeRTMobilityAbsolute{
            name: "Minimum search range RT/Mobility (absolute)".to_owned(),
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
    fn test_msrRTma_initialization(){
        let msrRTma_obj = MinimumSearchRangeRTMobilityAbsolute::new();
        assert_eq!(msrRTma_obj.name, "Minimum search range RT/Mobility (absolute)");
        assert_eq!(msrRTma_obj.value, None);
    }

    #[test]
    fn test_msrRTma_set_value(){
        let mut msrRTma_obj = MinimumSearchRangeRTMobilityAbsolute::new();
        assert_eq!(msrRTma_obj.value, None);
        msrRTma_obj.set_value(Some(13.8));
        assert_eq!(msrRTma_obj.value, Some(13.8));
    }

    #[test]
    fn test_msrRTma_get_value(){
        let mut msrRTma_obj = MinimumSearchRangeRTMobilityAbsolute::new();
        assert_eq!(msrRTma_obj.value, None);
        msrRTma_obj.value = Some(2.45);
        assert_eq!(msrRTma_obj.get_value(), Some(2.45));
    }
}
use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
pub struct FeatureLists{
    #[serde(rename="@name")]
    name: String,

    #[serde(rename="@type")]
    _type: String,

    #[serde(rename="$text")]
    value: Option<f32>
}

impl FeatureLists{
    pub fn new() -> Self{
        FeatureLists{
            name: "Feature lists".to_owned(),
            _type: "BATCH_LAST_FEATURELISTS".to_owned(),
            value: None,
        }
    }

    pub fn set_type(&mut self, _type:String){
        self._type = _type;
    }

    pub fn get_type(&self) -> String{
        self._type.clone()
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
    fn test_feature_lists_initialization(){
        let feature_lists_obj = FeatureLists::new();
        assert_eq!(feature_lists_obj.name, "Feature lists");
        assert_eq!(feature_lists_obj._type, "BATCH_LAST_FEATURELISTS");
        assert_eq!(feature_lists_obj.value, None);
    }

    #[test]
    fn test_feature_lists_get_type(){
        let mut feature_lists_obj = FeatureLists::new();
        assert_eq!(feature_lists_obj._type, "BATCH_LAST_FEATURELISTS");
        feature_lists_obj._type = "A type".to_owned();
        assert_eq!(feature_lists_obj.get_type(), "A type");
    }

    #[test]
    fn test_feature_lists_set_type(){
        let mut feature_lists_obj = FeatureLists::new();
        assert_eq!(feature_lists_obj._type, "BATCH_LAST_FEATURELISTS");
        feature_lists_obj.set_type("Another type".to_owned());
        assert_eq!(feature_lists_obj._type, "Another type");
    }

    #[test]
    fn test_feature_lists_get_value(){
        let mut feature_lists_obj = FeatureLists::new();
        assert_eq!(feature_lists_obj.value, None);
        feature_lists_obj.value = Some(12.3);
        assert_eq!(feature_lists_obj.get_value(), Some(12.3));
    }

    #[test]
    fn test_feature_lists_set_value(){
        let mut feature_lists_obj = FeatureLists::new();
        assert_eq!(feature_lists_obj.value, None);
        feature_lists_obj.set_value(Some(45.3));
        assert_eq!(feature_lists_obj.value, Some(45.3));
    }
}
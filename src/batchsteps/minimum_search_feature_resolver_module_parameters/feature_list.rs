use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize)]
#[serde(default, rename_all = "lowercase")]
pub struct FeatureLists{
    #[serde(rename="@name")]
    name: String,

    #[serde(rename = "@name")]
    _type: String,
}

impl FeatureLists{
    pub fn new() -> Self{
        FeatureLists{
            name: "Feature lists".to_owned(),
            _type: "BATCH_LAST_FEATURELISTS".to_owned(),
        }
    }

    pub fn set_type(&mut self, _type:String){
        self._type = _type;
    }

    pub fn get_type(&self) -> String{
        self._type.clone()
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
    }

    #[test]
    fn test_feature_lists_set_type(){
        let mut feature_lists_obj = FeatureLists::new();
        assert_eq!(feature_lists_obj._type, "BATCH_LAST_FEATURELISTS");
        feature_lists_obj.set_type("newType".to_owned());
        assert_eq!(feature_lists_obj._type, "newType");
    }

    #[test]
    fn test_feature_lists_get_type(){
        let mut feature_lists_obj = FeatureLists::new();
        assert_eq!(feature_lists_obj._type, "BATCH_LAST_FEATURELISTS");
        feature_lists_obj._type = "newType".to_owned();
        assert_eq!(feature_lists_obj.get_type(), "newType");
    }
}
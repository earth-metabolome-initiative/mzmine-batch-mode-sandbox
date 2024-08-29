use mzbatch_generator::smoothing_module_parameters::FeatureLists;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_lists_initialization(){
        let feature_lists_obj = FeatureLists::new();
        assert_eq!(feature_lists_obj.get_name(), "Feature lists");
        assert_eq!(feature_lists_obj.get_type(), "BATCH_LAST_FEATURELISTS");
        assert_eq!(*feature_lists_obj.get_value(), None);
    }

    #[test]
    fn test_feature_lists_set_get_type(){
        let mut feature_lists_obj = FeatureLists::new();
        assert_eq!(feature_lists_obj.get_type(), "BATCH_LAST_FEATURELISTS");
        feature_lists_obj.set_type("A type");
        assert_eq!(feature_lists_obj.get_type(), "A type");
    }

    #[test]
    fn test_feature_lists_set_get_value(){
        let mut feature_lists_obj = FeatureLists::new();
        assert_eq!(*feature_lists_obj.get_value(), None);
        feature_lists_obj.set_value(Some(12.3));
        assert_eq!(*feature_lists_obj.get_value(), Some(12.3));
    }

    #[test]
    fn feature_lists_serialization() -> Result<(), Box<dyn std::error::Error>> {
        let mut feature_lists = FeatureLists::new();
    
        let mut buffer = "".to_owned(); // Create the string buffer for the XML content
        quick_xml::se::to_writer(&mut buffer, &feature_lists)?;
    
        let expected = r#"<parameter name="Feature lists" type="BATCH_LAST_FEATURELISTS"/>"#;
        
        assert_eq!(buffer, expected);
    
        Ok(())
    }
    
}
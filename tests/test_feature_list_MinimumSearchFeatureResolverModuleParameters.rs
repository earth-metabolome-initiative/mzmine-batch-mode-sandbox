use mzbatch_generator::minimum_search_feature_resolver_module_parameters::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_lists_initialization(){
        let feature_lists_obj = FeatureLists::new();
        assert_eq!(feature_lists_obj.get_name(), "Feature lists");
        assert_eq!(feature_lists_obj.get_type(), "");
    }

    #[test]
    fn test_feature_lists_set_get_type(){
        let mut feature_lists_obj = FeatureLists::new();
        assert_eq!(feature_lists_obj.get_type(), "");
        feature_lists_obj.set_type("BATCH_LAST_FEATURELISTS");
        assert_eq!(feature_lists_obj.get_type(), "BATCH_LAST_FEATURELISTS");
    }

    #[test]
    fn feature_lists_serialization() -> Result<(), Box<dyn std::error::Error>> {
        let mut buffer = "".to_owned();
        let mut chr_obj = FeatureLists::new();
        chr_obj.set_type("BATCH_LAST_FEATURELISTS");

        quick_xml::se::to_writer(&mut buffer, &chr_obj)?;

        let expected = r#"<parameter name="Feature lists" type="BATCH_LAST_FEATURELISTS"/>"#;
        
        assert_eq!(buffer, expected);
        
        Ok(())
    }
}
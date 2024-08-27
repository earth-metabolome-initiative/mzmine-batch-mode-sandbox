use mzbatch_generator::minimum_search_feature_resolver_module_parameters::OriginalFeatureList;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suffix_initialization(){
        let suffix_obj = OriginalFeatureList::new();
        assert_eq!(suffix_obj.get_name(), "Original feature list");
        assert_eq!(suffix_obj.get_value(), "KEEP");
    }

    #[test]
    fn test_suffix_set_get_value(){
        let mut suffix_obj = OriginalFeatureList::new();
        assert_eq!(suffix_obj.get_value(), "KEEP");
        suffix_obj.set_value("TEST");
        assert_eq!(suffix_obj.get_value(), "TEST");
    }

    #[test]
    fn suffix_serialization() -> Result<(), Box<dyn std::error::Error>>{
        let mut buffer = "".to_owned();
        let mut original_feature_list = OriginalFeatureList::new();

        original_feature_list.set_value("KEEP");

        quick_xml::se::to_writer(&mut buffer, &original_feature_list)?;

        let expected = r#"<parameter name="Original feature list">KEEP</parameter>"#;

        assert_eq!(buffer, expected);

        Ok(())
    }
}
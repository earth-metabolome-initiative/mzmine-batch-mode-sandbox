use mzbatch_generator::isotope_grouper_module_parameters::OriginalFeatureList;

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn original_feature_list_initialization(){
        let orig_obj = OriginalFeatureList::new();
        assert_eq!(orig_obj.get_name(), "Original feature list");
        assert_eq!(orig_obj.get_value(), "");
    }

    #[test]
    fn original_feature_list_get_set_value(){
        let mut orig_obj = OriginalFeatureList::new();
        assert_eq!(orig_obj.get_value(), "");
        orig_obj.set_value("something");
        assert_eq!(orig_obj.get_value(), "something");
    }

    #[test]
    fn maximum_charge_serialization() -> Result<(), Box<dyn std::error::Error>>{
        let mut buffer = "".to_owned();

        let mut orig_obj = OriginalFeatureList::new();
        orig_obj.set_value("KEEP");

        quick_xml::se::to_writer(&mut buffer, &orig_obj)?;

        let expected = r#"<parameter name="Original feature list">KEEP</parameter>"#;

        assert_eq!(buffer, expected);
        Ok(())
    }
} 
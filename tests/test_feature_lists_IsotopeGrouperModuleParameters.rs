use mzbatch_generator::isotope_grouper_module_parameters::FeatureLists;
use mzbatch_generator::xml_serialization::*;

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn feature_lists_initialization(){
        let fl_obj  = FeatureLists::new();
        assert_eq!(fl_obj.get_name(), "Feature lists");
        assert_eq!(fl_obj.get_type(), "BATCH_LAST_FEATURELISTS");
    }

    #[test]
    fn feature_lists_get_set_type(){
        let mut fl_obj  = FeatureLists::new();
        assert_eq!(fl_obj.get_type(), "BATCH_LAST_FEATURELISTS");
        fl_obj.set_type("type");
        assert_eq!(fl_obj.get_type(), "type");
    }

    #[test]
    fn feature_lists_serialization() -> IoResult<()>{
        //<parameter name="Feature lists" type="BATCH_LAST_FEATURELISTS"></parameter>

        // Create a writer with an in-memory buffer
        let mut writer = Writer::new(Cursor::new(Vec::new()));

        let mut scan_number = FeatureLists::new();
        scan_number.set_type("BATCH_LAST_FEATURELISTS");

        // Write the ScanTypes element
        scan_number.write_element(&mut writer)?;

        // Convert buffer to string
        let result = writer.into_inner().into_inner();
        let result_str = String::from_utf8(result).expect("Failed to convert result to string");

        // Define the expected XML output
        let expected = r#"<parameter name="Feature lists" type="BATCH_LAST_FEATURELISTS"></parameter>"#;

        // Assert the result matches the expected output
        assert_eq!(result_str, expected);

        Ok(())
    }
}
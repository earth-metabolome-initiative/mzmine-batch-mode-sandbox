use mzbatch_generator::isotope_grouper_module_parameters::OriginalFeatureList;

use mzbatch_generator::xml_serialization::*;

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
    fn maximum_charge_serialization() -> IoResult<()>{
        // Create a writer with an in-memory buffer
        let mut writer = Writer::new(Cursor::new(Vec::new()));

        let mut orig_obj = OriginalFeatureList::new();
        orig_obj.set_value("KEEP");

        // Write the ScanTypes element
        orig_obj.write_element(&mut writer)?;

        // Convert buffer to string
        let result = writer.into_inner().into_inner();
        let result_str = String::from_utf8(result).expect("Failed to convert result to string");

        // Define the expected XML output
        let expected = r#"<parameter name="Original feature list">KEEP</parameter>"#;

        // Assert the result matches the expected output
        assert_eq!(result_str, expected);

        Ok(())
    }
} 
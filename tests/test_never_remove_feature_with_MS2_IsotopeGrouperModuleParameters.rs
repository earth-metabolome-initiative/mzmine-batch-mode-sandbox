use mzbatch_generator::isotope_grouper_module_parameters::NeverRemoveFeatureWithMs2;

use mzbatch_generator::xml_serialization::*;

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn never_remove_feature_with_MS2_initialization(){
        let never_obj = NeverRemoveFeatureWithMs2::new();
        assert_eq!(never_obj.get_name(), "Never remove feature with MS2");
        assert_eq!(*never_obj.get_value(), false);
    }

    #[test]
    fn never_remove_feature_with_MS2_get_set_value(){
        let mut never_obj = NeverRemoveFeatureWithMs2::new();
        assert_eq!(*never_obj.get_value(), false);
        never_obj.set_value(true);
        assert_eq!(*never_obj.get_value(), true);
    }

    #[test]
    fn maximum_charge_serialization() -> IoResult<()>{
        // Create a writer with an in-memory buffer
        let mut writer = Writer::new(Cursor::new(Vec::new()));

        let mut never_obj = NeverRemoveFeatureWithMs2::new();
        never_obj.set_value(true);

        // Write the ScanTypes element
        never_obj.write_element(&mut writer)?;

        // Convert buffer to string
        let result = writer.into_inner().into_inner();
        let result_str = String::from_utf8(result).expect("Failed to convert result to string");

        // Define the expected XML output
        let expected = r#"<parameter name="Never remove feature with MS2">true</parameter>"#;

        // Assert the result matches the expected output
        assert_eq!(result_str, expected);

        Ok(())
    }
} 
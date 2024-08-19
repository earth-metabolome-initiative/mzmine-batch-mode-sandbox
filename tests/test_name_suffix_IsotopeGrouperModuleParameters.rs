use mzbatch_generator::isotope_grouper_module_parameters::NameSuffix;
use mzbatch_generator::xml_serialization::*;

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn name_suffix_initialization(){
        let name_obj = NameSuffix::new();
        assert_eq!(name_obj.get_name(), "Name suffix");
        assert_eq!(name_obj.get_value(), "deiso");
    }

    #[test]
    fn name_suffix_get_set_value(){
        let mut name_obj = NameSuffix::new();
        assert_eq!(name_obj.get_value(), "deiso");
        name_obj.set_value("value");
        assert_eq!(name_obj.get_value(), "value");
    }

    #[test]
    fn maximum_charge_serialization() -> IoResult<()>{
        // Create a writer with an in-memory buffer
        let mut writer = Writer::new(Cursor::new(Vec::new()));

        let mut name_obj = NameSuffix::new();
        name_obj.set_value("deiso");

        // Write the ScanTypes element
        name_obj.write_element(&mut writer)?;

        // Convert buffer to string
        let result = writer.into_inner().into_inner();
        let result_str = String::from_utf8(result).expect("Failed to convert result to string");

        // Define the expected XML output
        let expected = r#"<parameter name="Name suffix">deiso</parameter>"#;

        // Assert the result matches the expected output
        assert_eq!(result_str, expected);

        Ok(())
    }
}
use mzbatch_generator::isotope_grouper_module_parameters::NameSuffix;

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
    fn maximum_charge_serialization() -> Result<(), Box<dyn std::error::Error>>{
        // Create a writer with an in-memory buffer
        let mut buffer = "".to_owned();

        let mut name_obj = NameSuffix::new();
        name_obj.set_value("deiso");

        quick_xml::se::to_writer(&mut buffer, &name_obj)?;
        
        let expected = r#"<parameter name="Name suffix">deiso</parameter>"#;

        // Assert the result matches the expected output
        assert_eq!(buffer, expected);

        Ok(())
    }
}
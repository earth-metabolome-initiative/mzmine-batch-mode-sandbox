use mzbatch_generator::isotope_grouper_module_parameters::RepresentativeIsotope;

use mzbatch_generator::xml_serialization::*;

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn representative_isotope_initialization(){
        let rep_obj = RepresentativeIsotope::new();
        assert_eq!(rep_obj.get_name(), "Representative isotope");
        assert_eq!(rep_obj.get_value(), "");
    }

    #[test]
    fn representative_isotope_get_set_value(){
        let mut rep_obj = RepresentativeIsotope::new();
        assert_eq!(rep_obj.get_value(), "");
        rep_obj.set_value("something");
        assert_eq!(rep_obj.get_value(), "something");
    }

    #[test]
    fn maximum_charge_serialization() -> IoResult<()>{
        // Create a writer with an in-memory buffer
        let mut writer = Writer::new(Cursor::new(Vec::new()));

        let mut rep_obj = RepresentativeIsotope::new();
        rep_obj.set_value("Most intense");

        // Write the ScanTypes element
        rep_obj.write_element(&mut writer)?;

        // Convert buffer to string
        let result = writer.into_inner().into_inner();
        let result_str = String::from_utf8(result).expect("Failed to convert result to string");

        // Define the expected XML output
        let expected = r#"<parameter name="Representative isotope">Most intense</parameter>"#;

        // Assert the result matches the expected output
        assert_eq!(result_str, expected);

        Ok(())
    }
}
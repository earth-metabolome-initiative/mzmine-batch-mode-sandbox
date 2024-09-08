use mzbatch_generator::isotope_grouper_module_parameters::RepresentativeIsotope;

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
    fn maximum_charge_serialization() -> Result<(), Box<dyn std::error::Error>>{
        // Create a writer with an in-memory buffer
        let mut buffer = "".to_owned();

        let mut rep_obj = RepresentativeIsotope::new();
        rep_obj.set_value("Most intense");


        quick_xml::se::to_writer(&mut buffer, &rep_obj)?;

        // Define the expected XML output
        let expected = r#"<parameter name="Representative isotope">Most intense</parameter>"#;

        // Assert the result matches the expected output
        assert_eq!(buffer, expected);

        Ok(())
    }
}
use mzbatch_generator::isotope_grouper_module_parameters::MaximumCharge;

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn maximum_charge_initialization(){
        let maximum_charge_obj = MaximumCharge::new();
        assert_eq!(maximum_charge_obj.get_name(), "Maximum charge");
        assert_eq!(*maximum_charge_obj.get_value(), None);
    }

    #[test]
    fn maximum_charge_get_set_value(){
        let mut maximum_charge_obj = MaximumCharge::new();
        assert_eq!(*maximum_charge_obj.get_value(), None);
        maximum_charge_obj.set_value(Some(1));
        assert_eq!(*maximum_charge_obj.get_value(), Some(1));
    }

    #[test]
    fn maximum_charge_serialization() -> Result<(), Box<dyn std::error::Error>>{
        // Create a writer with an in-memory buffer
        let mut buffer = "".to_owned();

        let mut maximum_charge_obj = MaximumCharge::new();
        maximum_charge_obj.set_value(Some(2));

        quick_xml::se::to_writer(&mut buffer, &maximum_charge_obj);
        // Define the expected XML output
        let expected = r#"<parameter name="Maximum charge">2</parameter>"#;

        // Assert the result matches the expected output
        assert_eq!(buffer, expected);

        Ok(())
    }
}
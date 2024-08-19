use mzbatch_generator::isotope_grouper_module_parameters::MaximumCharge;

use mzbatch_generator::xml_serialization::*;

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
    fn maximum_charge_serialization() -> IoResult<()>{
        // Create a writer with an in-memory buffer
        let mut writer = Writer::new(Cursor::new(Vec::new()));

        let mut maximum_charge_obj = MaximumCharge::new();
        maximum_charge_obj.set_value(Some(2));

        // Write the ScanTypes element
        maximum_charge_obj.write_element(&mut writer)?;

        // Convert buffer to string
        let result = writer.into_inner().into_inner();
        let result_str = String::from_utf8(result).expect("Failed to convert result to string");

        // Define the expected XML output
        let expected = r#"<parameter name="Maximum charge">2</parameter>"#;

        // Assert the result matches the expected output
        assert_eq!(result_str, expected);

        Ok(())
    }
}
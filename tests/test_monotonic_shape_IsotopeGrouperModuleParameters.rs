use mzbatch_generator::isotope_grouper_module_parameters::MonotonicShape;

use mzbatch_generator::xml_serialization::*;

#[cfg(test)]
mod test{
    use mzbatch_generator::prelude::MobilityTolerance;

    use super::*;

    #[test]
    fn monotonic_shape_initialization(){
        let monotonic_obj = MonotonicShape::new();
        assert_eq!(monotonic_obj.get_name(), "Monotonic shape");
        assert_eq!(*monotonic_obj.get_value(), true);
    }

    #[test]
    fn mobility_tolerance_get_set_value(){
        let mut monotonic_obj = MonotonicShape::new();
        assert_eq!(*monotonic_obj.get_value(), true);
        monotonic_obj.set_value(false);
        assert_eq!(*monotonic_obj.get_value(), false);
    }

    #[test]
    fn maximum_charge_serialization() -> IoResult<()>{
        // Create a writer with an in-memory buffer
        let mut writer = Writer::new(Cursor::new(Vec::new()));

        let mut monotonic_obj = MonotonicShape::new();
        
        if !*monotonic_obj.get_value(){
            monotonic_obj.set_value(true);
        }

        // Write the ScanTypes element
        monotonic_obj.write_element(&mut writer)?;

        // Convert buffer to string
        let result = writer.into_inner().into_inner();
        let result_str = String::from_utf8(result).expect("Failed to convert result to string");

        // Define the expected XML output
        let expected = r#"<parameter name="Monotonic shape">true</parameter>"#;

        // Assert the result matches the expected output
        assert_eq!(result_str, expected);

        Ok(())
    }
}
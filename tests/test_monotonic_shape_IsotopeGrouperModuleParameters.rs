use mzbatch_generator::isotope_grouper_module_parameters::MonotonicShape;

#[cfg(test)]
mod test{
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
    fn maximum_charge_serialization() -> Result<(), Box<dyn std::error::Error>>{
        // Create a writer with an in-memory buffer
        let mut buffer = "".to_owned();

        let mut monotonic_obj = MonotonicShape::new();
        
        if !*monotonic_obj.get_value(){
            monotonic_obj.set_value(true);
        }


        quick_xml::se::to_writer(&mut buffer, &monotonic_obj)?;

        let expected = r#"<parameter name="Monotonic shape">true</parameter>"#;

        // Assert the result matches the expected output
        assert_eq!(buffer, expected);

        Ok(())
    }
}
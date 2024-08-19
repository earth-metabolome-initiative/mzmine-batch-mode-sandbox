use mzbatch_generator::isotope_grouper_module_parameters::MobilityTolerance;

use mzbatch_generator::xml_serialization::*;

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn mobility_tolerance_initialization(){
        let mtol_obj = MobilityTolerance::new();
        assert_eq!(mtol_obj.get_name(), "Mobility tolerance");
        assert_eq!(*mtol_obj.is_selected(), false);
        assert_eq!(*mtol_obj.get_value(), None);
    }

    #[test]
    fn mobility_tolerance_selection(){
        let mut mtol_obj = MobilityTolerance::new();
        assert_eq!(*mtol_obj.is_selected(), false);
        mtol_obj.select();
        assert_eq!(*mtol_obj.is_selected(), true);
        mtol_obj.deselect();
        assert_eq!(*mtol_obj.is_selected(), false);
        mtol_obj.invert_selected();
        assert_eq!(*mtol_obj.is_selected(), true);
    }

    #[test]
    fn mobility_tolerance_get_set_value(){
        let mut mtol_obj = MobilityTolerance::new();
        assert_eq!(*mtol_obj.get_value(), None);
        mtol_obj.set_value(Some(1.1));
        assert_eq!(*mtol_obj.get_value(), Some(1.1));
    }

    #[test]
    fn maximum_charge_serialization() -> IoResult<()>{
        // Create a writer with an in-memory buffer
        let mut writer = Writer::new(Cursor::new(Vec::new()));

        let mut mtol_obj = MobilityTolerance::new();
        mtol_obj.set_value(Some(1.0));

        if *mtol_obj.is_selected(){
            mtol_obj.invert_selected();
        }

        // Write the ScanTypes element
        mtol_obj.write_element(&mut writer)?;

        // Convert buffer to string
        let result = writer.into_inner().into_inner();
        let result_str = String::from_utf8(result).expect("Failed to convert result to string");

        // Define the expected XML output
        let expected = r#"<parameter name="Mobility tolerance" selected="false">1.0</parameter>"#;

        // Assert the result matches the expected output
        assert_eq!(result_str, expected);

        Ok(())
    }
}
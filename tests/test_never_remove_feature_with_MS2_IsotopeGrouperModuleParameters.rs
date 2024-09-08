use mzbatch_generator::isotope_grouper_module_parameters::NeverRemoveFeatureWithMs2;

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
    fn maximum_charge_serialization() -> Result<(), Box<dyn std::error::Error>>{
        let mut buffer = "".to_owned();

        let mut never_obj = NeverRemoveFeatureWithMs2::new();
        never_obj.set_value(true);

        quick_xml::se::to_writer(&mut buffer, &never_obj)?;

        let expected = r#"<parameter name="Never remove feature with MS2">true</parameter>"#;

        assert_eq!(buffer, expected);

        Ok(())
    }
} 
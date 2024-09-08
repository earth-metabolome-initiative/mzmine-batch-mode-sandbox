use mzbatch_generator::isotope_grouper_module_parameters::RetentionTimeTolerance;

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn retention_time_tolerance_initialization(){
        let ret_obj = RetentionTimeTolerance::new();
        assert_eq!(ret_obj.get_name(), "Retention time tolerance");
        assert_eq!(ret_obj.get_unit(), "MINUTES");
        assert_eq!(*ret_obj.get_value(), None);
    }

    #[test]
    fn retention_time_tolerance_get_set_unit(){
        let mut ret_obj = RetentionTimeTolerance::new();
        assert_eq!(ret_obj.get_unit(), "MINUTES");
        ret_obj.set_unit("value");
        assert_eq!(ret_obj.get_unit(), "value");
    }

    #[test]
    fn retention_time_tolerance_get_set_value(){
        let mut ret_obj = RetentionTimeTolerance::new();
        assert_eq!(*ret_obj.get_value(), None);
        ret_obj.set_value(Some(23.4));
        assert_eq!(*ret_obj.get_value(), Some(23.4));
    }

    #[test]
    fn retention_time_tolerance_serialozation() -> Result<(), Box<dyn std::error::Error>>{
        let mut buffer = "".to_owned();
        let mut ret_obj = RetentionTimeTolerance::new();

        ret_obj.set_unit("MINUTES");
        ret_obj.set_value(Some(0.04));

        quick_xml::se::to_writer(&mut buffer, &ret_obj)?;

        let expected = r#"<parameter name="Retention time tolerance" unit="MINUTES">0.04</parameter>"#;
        assert_eq!(buffer, expected);
        Ok(())
    }
}
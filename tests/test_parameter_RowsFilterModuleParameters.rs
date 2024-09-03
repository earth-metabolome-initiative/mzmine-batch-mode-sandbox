use mzbatch_generator::rows_filter_module_parameters::Parameter;

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn parameter_initialization(){
        let parameter = Parameter::default();
        assert_eq!(parameter.get_name(), "");
        assert_eq!(*parameter.is_selected(), None);
    }

    #[test]
    fn parameter_select_deselect(){
        let mut parameter = Parameter::default();
        assert_eq!(*parameter.is_selected(), None);
        parameter.select();
        assert_eq!(*parameter.is_selected(), Some(true));
        parameter.deselect();
        assert_eq!(*parameter.is_selected(), Some(false));
    }

    #[test]
    fn parameter_simple_serializazion() -> Result<(), Box<dyn std::error::Error>>{
        let mut buffer = "".to_owned();
        let parameter = Parameter::default();
        quick_xml::se::to_writer(&mut buffer, &parameter)?;
        let expected = r#"<parameter name=""/>"#;
        assert_eq!(buffer, expected);
        Ok(())
    }

    #[test]
    fn parameter_possible_serializations() -> Result<(), Box<dyn std::error::Error>>{
        //name and value
        let mut buffer = "".to_owned();
        let mut parameter = Parameter::new("Parameter", None, "No parameters defined");
        quick_xml::se::to_writer(&mut buffer, &parameter)?;
        let expected = r#"<parameter name="Parameter">No parameters defined</parameter>"#;
        assert_eq!(buffer, expected);

        //name, selected
        let mut buffer = "".to_owned();
        let mut parameter = Parameter::new("Text in identity", Some(false), "");
        quick_xml::se::to_writer(&mut buffer, &parameter)?;
        let expected = r#"<parameter name="Text in identity" selected="false"/>"#;
        assert_eq!(buffer, expected);
        
        Ok(())
    }

}
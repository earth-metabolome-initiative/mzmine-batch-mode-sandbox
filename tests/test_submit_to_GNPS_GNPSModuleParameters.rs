use mzbatch_generator::gnps_fbmn_export_and_submit_module_parameters::SubmitToGNPS;
use mzbatch_generator::rows_filter_module_parameters::Parameter;

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn submit_to_GNPS_initialization(){
        let sub_obj = SubmitToGNPS::new();
        assert_eq!(sub_obj.get_name(), "Submit to GNPS");
        assert_eq!(*sub_obj.is_selected(), false);
        assert_eq!(sub_obj.get_parameters_length(), 0);
    }

    #[test]
    fn submit_to_GNPS_add_parameter(){
        let mut sub_obj = SubmitToGNPS::new();
        assert_eq!(sub_obj.get_parameters_length(), 0);
        sub_obj.add_parameter(Parameter::new("Parameter", None, "value"));
        assert_eq!(sub_obj.get_parameters_length(), 1);
    }

    #[test]
    fn submit_to_GNPS_serialization() -> Result<(), Box<dyn std::error::Error>>{
        let mut buffer = "".to_owned();
        let mut sub_obj = SubmitToGNPS::new();

        sub_obj.add_parameter(Parameter::new("Meta data file", Some(false), ""));
        sub_obj.add_parameter(Parameter::new("Export ion identity networks", None, "true"));
        sub_obj.add_parameter(Parameter::new("Presets", None, "HIGHRES"));
        sub_obj.add_parameter(Parameter::new("Job title", None, ""));
        sub_obj.add_parameter(Parameter::new("Email", None, ""));
        sub_obj.add_parameter(Parameter::new("Username", None, ""));
        sub_obj.add_parameter(Parameter::new("Password", None, ""));
        sub_obj.add_parameter(Parameter::new("Open website", None, "true"));

        quick_xml::se::to_writer(&mut buffer, &sub_obj)?;
        
        let expected = r#"<parameter name="Submit to GNPS" selected="false"><parameter name="Meta data file" selected="false"/><parameter name="Export ion identity networks">true</parameter><parameter name="Presets">HIGHRES</parameter><parameter name="Job title"/><parameter name="Email"/><parameter name="Username"/><parameter name="Password"/><parameter name="Open website">true</parameter></parameter>"#;
        
        assert_eq!(buffer, expected);
        Ok(())
    }
}
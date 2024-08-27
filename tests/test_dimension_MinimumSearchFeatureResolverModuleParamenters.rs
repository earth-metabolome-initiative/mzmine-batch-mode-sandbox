use mzbatch_generator::minimum_search_feature_resolver_module_parameters::Dimension;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suffix_initialization(){
        let dimension_obj = Dimension::new();
        assert_eq!(dimension_obj.get_name(), "Dimension");
        assert_eq!(dimension_obj.get_value(), "");
    }

    #[test]
    fn test_suffix_set_get_value(){
        let mut dimension_obj = Dimension::new();
        assert_eq!(dimension_obj.get_value(), "");
        dimension_obj.set_value("TEST");
        assert_eq!(dimension_obj.get_value(), "TEST");
    }

    #[test]
    fn dimension_serialization() -> Result<(), Box<dyn std::error::Error>> {
        let mut buffer = "".to_owned();
        let mut chr_obj = Dimension::new();
        chr_obj.set_value("Retention time");

        quick_xml::se::to_writer(&mut buffer, &chr_obj)?;

        let expected = r#"<parameter name="Dimension">Retention time</parameter>"#;
        
        assert_eq!(buffer, expected);
        
        Ok(())
    }
}
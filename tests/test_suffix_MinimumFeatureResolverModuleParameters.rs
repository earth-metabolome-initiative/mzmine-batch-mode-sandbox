use mzbatch_generator::minimum_search_feature_resolver_module_parameters::Suffix;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suffix_initialization(){
        let suffix_obj = Suffix::new();
        assert_eq!(suffix_obj.get_name(), "Suffix");
        assert_eq!(*suffix_obj.get_value(), '\0');
    }

    #[test]
    fn test_suffix_set_value(){
        let mut suffix_obj = Suffix::new();
        assert_eq!(*suffix_obj.get_value(), '\0');
        suffix_obj.set_value('f');
        assert_eq!(*suffix_obj.get_value(), 'f');
    }

    #[test]
    fn suffix_serialization() -> Result<(), Box<dyn std::error::Error>> {
        let mut buffer = "".to_owned();
        let mut chr_obj = Suffix::new();
        chr_obj.set_value('r');

        quick_xml::se::to_writer(&mut buffer, &chr_obj)?;

        let expected = r#"<parameter name="Suffix">r</parameter>"#;
        
        assert_eq!(buffer, expected);
        
        Ok(())
    }
}
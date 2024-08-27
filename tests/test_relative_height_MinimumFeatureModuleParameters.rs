use mzbatch_generator::minimum_search_feature_resolver_module_parameters::MinimumRelativeHeight;

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn minimum_relative_height_initialization(){
        let chr_obj = MinimumRelativeHeight::new();
        assert_eq!(chr_obj.get_name(), "Minimum relative height");
        assert_eq!(*chr_obj.get_value(), None);
    }

    #[test]
    fn minimum_relative_height_set_get_value(){
        let mut chr_obj = MinimumRelativeHeight::new();
        assert_eq!(*chr_obj.get_value(), None);
        chr_obj.set_value(Some(3.2));
        assert_eq!(*chr_obj.get_value(), Some(3.2));
    }

    #[test]
    fn minimum_relative_height_serialization() -> Result<(), Box<dyn std::error::Error>> {
        let mut buffer = "".to_owned();
        let mut chr_obj = MinimumRelativeHeight::new();
        chr_obj.set_value(Some(0.0));

        quick_xml::se::to_writer(&mut buffer, &chr_obj)?;

        let expected = r#"<parameter name="Minimum relative height">0</parameter>"#;
        
        assert_eq!(buffer, expected);
        
        Ok(())
    }
}

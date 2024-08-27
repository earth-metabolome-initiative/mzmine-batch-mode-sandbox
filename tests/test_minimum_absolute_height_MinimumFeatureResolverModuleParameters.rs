use mzbatch_generator::minimum_search_feature_resolver_module_parameters::MinimumAbsoluteHeight;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_rel_height_initialization(){
        let min_rel_height_obj = MinimumAbsoluteHeight::new();
        assert_eq!(min_rel_height_obj.get_name(), "Minimum absolute height");
        assert_eq!(*min_rel_height_obj.get_value(), None);
    }

    #[test]
    fn min_rel_height_set_get_value(){
        let mut min_rel_height_obj = MinimumAbsoluteHeight::new();
        assert_eq!(*min_rel_height_obj.get_value(), None);
        min_rel_height_obj.set_value(Some(13.8));
        assert_eq!(*min_rel_height_obj.get_value(), Some(13.8));
    }

    #[test]
    fn minimum_absolute_height_serialization() -> Result<(), Box<dyn std::error::Error>> {
        let mut buffer = "".to_owned();
        let mut chr_obj = MinimumAbsoluteHeight::new();
        chr_obj.set_value(Some(50000.0));

        quick_xml::se::to_writer(&mut buffer, &chr_obj)?;

        let expected = r#"<parameter name="Minimum absolute height">50000</parameter>"#;
        
        assert_eq!(buffer, expected);
        
        Ok(())
    }
}
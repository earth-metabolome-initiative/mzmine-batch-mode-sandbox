use mzbatch_generator::rows_filter_module_parameters::MinimumFeaturesInAnIsotopePattern;

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn minimum_feature_in_an_isotope_pattern_initialization(){
        let min_obj = MinimumFeaturesInAnIsotopePattern::new();
        assert_eq!(min_obj.get_name(), "Minimum features in an isotope pattern");
        assert_eq!(*min_obj.is_selected(), false);
        assert_eq!(*min_obj.get_value(), None);
    }

    #[test]
    fn minimum_feature_in_an_isotope_pattern_get_set_value(){
        let mut min_obj = MinimumFeaturesInAnIsotopePattern::new();
        assert_eq!(*min_obj.get_value(), None);
        min_obj.set_value(Some(1));
        assert_eq!(*min_obj.get_value(), Some(1));
    }

    #[test]
    fn minimum_feature_in_an_isotope_pattern_selection(){
        let mut min_obj = MinimumFeaturesInAnIsotopePattern::new();
        assert_eq!(*min_obj.is_selected(), false);
        min_obj.select();
        assert_eq!(*min_obj.is_selected(), true);
        min_obj.deselect();
        assert_eq!(*min_obj.is_selected(), false);
    }

    #[test]
    fn minimum_feature_in_an_isotope_pattern_serialization() -> Result<(), Box<dyn std::error::Error>>{
        let mut buffer = "".to_owned();
        let mut min_obj = MinimumFeaturesInAnIsotopePattern::new();

        min_obj.select();
        min_obj.set_value(Some(2));

        quick_xml::se::to_writer(&mut buffer, &min_obj)?;
        let expected = r#"<parameter name="Minimum features in an isotope pattern" selected="true">2</parameter>"#;
        assert_eq!(buffer, expected);
        Ok(())
    }
}
use mzbatch_generator::rows_filter_module_parameters::FeaturesDurationRange;

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn features_duration_range_initialization(){
        let features_obj = FeaturesDurationRange::default();
        assert_eq!(features_obj.get_name(), "features duration range");
        assert_eq!(*features_obj.is_selected(), false);
        assert_eq!(*features_obj.get_min_value(), None);
        assert_eq!(*features_obj.get_max_value(), None);
    }

    #[test]
    fn feature_lists_selection(){
        let mut features_obj = FeaturesDurationRange::default();
        assert_eq!(*features_obj.is_selected(), false);
        features_obj.select();
        assert_eq!(*features_obj.is_selected(), true);
        features_obj.deselect();
        assert_eq!(*features_obj.is_selected(), false);
    }

    #[test]
    fn feature_lists_serialization() -> Result<(), Box<dyn std::error::Error>>{
        let mut buffer = "".to_owned();
        
        let feature_obj = FeaturesDurationRange::new(false, Some(0.0), Some(3.0));

        quick_xml::se::to_writer(&mut buffer, &feature_obj)?;
        let expected = r#"<parameter name="features duration range" selected="false"><min>0</min><max>3</max></parameter>"#;
        assert_eq!(buffer, expected);
        Ok(())
    }
}
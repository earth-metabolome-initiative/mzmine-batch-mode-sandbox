use mzbatch_generator::rows_filter_module_parameters::MinimumAlignedFeaturesSamples;

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn minimum_aligned_feature_samples_initialization(){
        let min_obj = MinimumAlignedFeaturesSamples::new();
        assert_eq!(min_obj.get_name(), "Minimum aligned features (samples)");
        assert_eq!(*min_obj.is_selected(), false);
        assert_eq!(*min_obj.get_abs_value(), None);
        assert_eq!(*min_obj.get_rel_value(), None);
    }

    #[test]
    fn minimum_aligned_feature_samples_set_get_abs_rel_value(){
        let mut min_obj = MinimumAlignedFeaturesSamples::new();
        min_obj.set_abs_value(Some(3.3));
        min_obj.set_rel_value(Some(4.5));

        assert_eq!(*min_obj.get_abs_value(), Some(3.3));
        assert_eq!(*min_obj.get_rel_value(), Some(4.5));
    }

    #[test]
    fn minimum_aligned_feature_samples_serialization() -> Result<(), Box<dyn std::error::Error>>{
        let mut buffer = "".to_owned();
        let mut min_obj = MinimumAlignedFeaturesSamples::new();

        min_obj.set_abs_value(Some(1.0));
        min_obj.set_rel_value(Some(0.0));

        quick_xml::se::to_writer(&mut buffer, &min_obj)?;

        let expected =  r#"<parameter name="Minimum aligned features (samples)" selected="false"><abs>1</abs><rel>0</rel></parameter>"#;

        assert_eq!(buffer, expected);
        Ok(())
    }
}
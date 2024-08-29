use mzbatch_generator::minimum_search_feature_resolver_module_parameters::Ms1Ms2PrecursorTolerance as MzToleranceIntraSample;

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn m_z_tolerance_intra_sample_initialization(){
        let mz_obj = MzToleranceIntraSample::new_isotope_grouper_module();
        assert_eq!(mz_obj.get_name(), "m/z tolerance (intra-sample)");
        assert_eq!(*mz_obj.get_absolute_value(), None);
        assert_eq!(*mz_obj.get_ppm_value(), None);
    }

    #[test]
    fn m_z_tolerance_intra_sample_serialization() -> Result<(), Box<dyn std::error::Error>>{

        let mut buffer = "".to_owned();
        let mut mz_obj = MzToleranceIntraSample::new();

        mz_obj.set_name("m/z tolerance (intra-sample)");
        mz_obj.set_absolute_value(Some(0.0015));
        mz_obj.set_ppm_value(Some(3.0));

        quick_xml::se::to_writer(&mut buffer, &mz_obj)?;
        let expected = r#"<parameter name="m/z tolerance (intra-sample)"><absolutetolerance>0.0015</absolutetolerance><ppmtolerance>3</ppmtolerance></parameter>"#;

        // Assert the result matches the expected output
        assert_eq!(buffer, expected);

        Ok(())
    }

}
use mzbatch_generator::minimum_search_feature_resolver_module_parameters::Ms1Ms2PrecursorTolerance;

#[cfg(test)]
mod test{

    use super::*;

    #[test]
    fn ms1_to_ms2_precursor_tolerance_initialization(){
        let ms12pt_obj = Ms1Ms2PrecursorTolerance::new();
        assert_eq!(ms12pt_obj.get_name(), "MS1 to MS2 precursor tolerance (m/z)");
        assert_eq!(*ms12pt_obj.get_absolute_value(), None);
        assert_eq!(*ms12pt_obj.get_ppm_value(), None)
    }

    #[test]
    fn ms1_to_ms2_precursor_tolerance_serialization() -> Result<(), Box<dyn std::error::Error>> {
        let mut buffer = "".to_owned();

        let mut obj = Ms1Ms2PrecursorTolerance::new();

        obj.set_absolute_value(Some(0.01));
        obj.set_ppm_value(Some(10.0));
    
        quick_xml::se::to_writer(&mut buffer, &obj)?;
    
        let expected = r#"<parameter name="MS1 to MS2 precursor tolerance (m/z)"><absolutetolerance>0.01</absolutetolerance><ppmtolerance>10</ppmtolerance></parameter>"#;
    
        assert_eq!(buffer, expected);
    
        Ok(())
    }
}
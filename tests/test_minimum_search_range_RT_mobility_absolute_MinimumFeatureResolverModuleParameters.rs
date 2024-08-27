use mzbatch_generator::minimum_search_feature_resolver_module_parameters::MinimumSearchRangeRTMobilityAbsolute;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_msrRTma_initialization(){
        let msrRTma_obj = MinimumSearchRangeRTMobilityAbsolute::new();
        assert_eq!(msrRTma_obj.get_name(), "Minimum search range RT/Mobility (absolute)");
        assert_eq!(*msrRTma_obj.get_value(), None);
    }

    #[test]
    fn test_msrRTma_set_get_value(){
        let mut msrRTma_obj = MinimumSearchRangeRTMobilityAbsolute::new();
        assert_eq!(*msrRTma_obj.get_value(), None);
        msrRTma_obj.set_value(Some(13.8));
        assert_eq!(*msrRTma_obj.get_value(), Some(13.8));
    }

    #[test]
    fn minimum_search_range_RT_mobility_absolute_serialization() -> Result<(), Box<dyn std::error::Error>> {
        let mut buffer = "".to_owned();
        let mut chr_obj = MinimumSearchRangeRTMobilityAbsolute::new();
        chr_obj.set_value(Some(5000.0));

        quick_xml::se::to_writer(&mut buffer, &chr_obj)?;

        let expected = r#"<parameter name="Minimum search range RT/Mobility (absolute)">5000</parameter>"#;
        
        assert_eq!(buffer, expected);
        
        Ok(())
    }
}
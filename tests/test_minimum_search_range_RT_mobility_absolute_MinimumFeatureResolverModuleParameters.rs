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
}
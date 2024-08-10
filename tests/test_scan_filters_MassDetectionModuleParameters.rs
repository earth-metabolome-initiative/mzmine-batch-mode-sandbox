use mzbatch_generator::mass_detection_module::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_scan_filters_initialization(){
        let scan_filters_obj = ScanFilters::new();
        assert_eq!(scan_filters_obj.get_name(), "Scan filters");
        assert_eq!(scan_filters_obj.get_parameters_length(), 0);
    }

    #[test]
    fn test_scan_filters_add_parameter(){
        let mut scan_filters_obj = ScanFilters::new();
        assert_eq!(scan_filters_obj.get_parameters_length(), 0);
        scan_filters_obj.add_parameter(ScanFiltersParameter::Mobility(Mobility::new()));
        assert_eq!(scan_filters_obj.get_parameters_length(), 1);
    }

    #[test]
    fn test_scan_number_initialization(){
        let scan_number_obj = ScanNumber::new();
        assert_eq!(scan_number_obj.get_name(), "Scan number");
        assert_eq!(scan_number_obj.get_value(), None);
    }

    #[test]
    fn test_scan_number_set_get_value(){
        let mut scan_number_obj = ScanNumber::new();
        assert_eq!(scan_number_obj.get_value(), None);
        scan_number_obj.set_value(Some(2));
        assert_eq!(scan_number_obj.get_value(), Some(2));
    }

    // // // 

    #[test]
    fn test_base_filtering_integer_initialization(){
        let base_filtering_integer_obj = BaseFilteringInteger::new();
        assert_eq!(base_filtering_integer_obj.get_name(), "Base Filtering Integer");
        assert_eq!(base_filtering_integer_obj.get_value(), None);
    }

    #[test]
    fn test_base_filtering_integer_set_get_value(){
        let mut base_filtering_integer_obj = BaseFilteringInteger::new();
        assert_eq!(base_filtering_integer_obj.get_value(), None);
        base_filtering_integer_obj.set_value(Some(2));
        assert_eq!(base_filtering_integer_obj.get_value(), Some(2));
    }

    // // //

    #[test]
    fn test_retention_time_initialization(){
        let retention_time_obj = RetentionTime::new();
        assert_eq!(retention_time_obj.get_name(), "Retention time");
        assert_eq!(retention_time_obj.get_value(), None);
    }

    #[test]
    fn test_retention_time_set_get_value(){
        let mut retention_time_obj = RetentionTime::new();
        assert_eq!(retention_time_obj.get_value(), None);
        retention_time_obj.set_value(Some(2));
        assert_eq!(retention_time_obj.get_value(), Some(2));
    }

    // // //

    #[test]
    fn test_mobility_initialization(){
        let mobility_obj = Mobility::new();
        assert_eq!(mobility_obj.get_name(), "Mobility");
        assert_eq!(mobility_obj.get_value(), None);
    }

    #[test]
    fn test_mobility_set_get_value(){
        let mut mobility_obj = Mobility::new();
        assert_eq!(mobility_obj.get_value(), None);
        mobility_obj.set_value(Some(2));
        assert_eq!(mobility_obj.get_value(), Some(2));
    }

    // // //

    #[test]
    fn test_ms_level_initialization(){
        let ms_level_obj = MSLevelFilter::new();
        assert_eq!(ms_level_obj.get_name(), "MS level filter");
        assert_eq!(ms_level_obj.get_value(), None);
    }

    #[test]
    fn test_ms_level_set_get_value(){
        let mut ms_level_obj = MSLevelFilter::new();
        assert_eq!(ms_level_obj.get_value(), None);
        ms_level_obj.set_value(Some(2));
        assert_eq!(ms_level_obj.get_value(), Some(2));
    }

    #[test]
    fn test_ms_level_set_ms1(){
        let mut ms_level_obj = MSLevelFilter::new();
        assert_eq!(ms_level_obj.get_level(), "None selected");
        ms_level_obj.set_ms1(Some(4));
        assert_eq!(ms_level_obj.get_level(), "MS1, level = 1");
        assert_eq!(ms_level_obj.get_value(), Some(4));
    }

    #[test]
    fn test_ms_level_set_ms2(){
        let mut ms_level_obj = MSLevelFilter::new();
        assert_eq!(ms_level_obj.get_level(), "None selected");
        ms_level_obj.set_ms2(Some(3));
        assert_eq!(ms_level_obj.get_level(), "MSn, level ≥ 2");
        assert_eq!(ms_level_obj.get_value(), Some(3));
    }

    #[test]
    fn test_ms_level_get_level(){
        let mut ms_level_obj = MSLevelFilter::new();
        assert_eq!(ms_level_obj.get_level(), "None selected");
        ms_level_obj.set_ms1(Some(4));
        assert_eq!(ms_level_obj.get_level(), "MS1, level = 1");
    }

    // // //

    #[test]
    fn test_scan_definition_initialization(){
        let scan_definition_obj = ScanDefinition::new();
        assert_eq!(scan_definition_obj.get_name(), "Scan definition");
        assert_eq!(scan_definition_obj.get_value(), None);
    }

    #[test]
    fn test_scan_definition_set_value(){
        let mut scan_definition_obj = ScanDefinition::new();
        assert_eq!(scan_definition_obj.get_value(), None);
        scan_definition_obj.set_value(Some(2));
        assert_eq!(scan_definition_obj.get_value(), Some(2));
    }
    
    #[test]
    fn test_scan_definition_get_value(){
        let mut scan_definition_obj = ScanDefinition::new();
        assert_eq!(scan_definition_obj.get_value(), None);
        scan_definition_obj.set_value(Some(3));
        assert_eq!(scan_definition_obj.get_value(), Some(3)); 
    }

    // // // 

    #[test]
    fn test_polarity_initalization(){
        let polarity_obj = Polarity::new();
        assert_eq!(polarity_obj.get_name(), "Polarity");
        assert_eq!(polarity_obj.get_value(), "Any");
    }

    #[test]
    fn test_polarity_set_get_value(){
        let mut polarity_obj = Polarity::new();
        assert_eq!(polarity_obj.get_value(), "Any");
        polarity_obj.set_value("Another".to_owned());
        assert_eq!(polarity_obj.get_value(), "Another");
    }

    // // // 

    #[test]
    fn test_spectrum_type_initalization(){
        let spectrum_type_obj = SpectrumType::new();
        assert_eq!(spectrum_type_obj.get_name(), "Spectrum type");
        assert_eq!(spectrum_type_obj.get_value(), "ANY");
    }

    #[test]
    fn test_spectrum_type_set_get_value(){
        let mut spectrum_type_obj = SpectrumType::new();
        assert_eq!(spectrum_type_obj.get_value(), "ANY");
        spectrum_type_obj.set_value("Another".to_owned());
        assert_eq!(spectrum_type_obj.get_value(), "Another");
    }
}
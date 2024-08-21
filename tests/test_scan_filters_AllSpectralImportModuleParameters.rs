use mzbatch_generator::all_spectral_data_import_module_parameters::*;

#[cfg(test)]
mod tests {
    use mzbatch_generator::batchsteps::mass_detection_module_parameters::scan_filters;

    use super::*;

    #[test]
    fn scan_filter_initialization(){
        let scan_filter_obj = ScanFilters::new();
        assert_eq!(scan_filter_obj.get_name(), "Scan filters");
        assert_eq!(scan_filter_obj.is_selected(), true);
        assert_eq!(scan_filter_obj.get_parameters_length(), 0);
    }

    #[test]
    fn scan_filter_add_parameter(){
        let mut scan_filter_obj = ScanFilters::new();
        assert_eq!(scan_filter_obj.get_parameters_length(), 0);
        let mut new_parameter = ScanFiltersParameters::Mobility;
        scan_filter_obj.add_parameter(ScanFiltersParameters::Mobility(Mobility::new()));
        assert_eq!(scan_filter_obj.get_parameters_length(), 1);
    }

    #[test]
    fn test_get_parameter() {
        let mut scan_filters = ScanFilters::default();

        // Test retrieving each parameter
        assert!(matches!(scan_filters.get_parameter("Scan number"), Some(ScanFiltersParameters::ScanNumber(_))));
        assert!(matches!(scan_filters.get_parameter("Base Filtering Integer"), Some(ScanFiltersParameters::BaseFilteringInteger(_))));
        assert!(matches!(scan_filters.get_parameter("Retention time"), Some(ScanFiltersParameters::RetentionTime(_))));
        assert!(matches!(scan_filters.get_parameter("Mobility"), Some(ScanFiltersParameters::Mobility(_))));
        assert!(matches!(scan_filters.get_parameter("MS Level Filter"), Some(ScanFiltersParameters::MSLevelFilter(_))));
        assert!(matches!(scan_filters.get_parameter("Scan definition"), Some(ScanFiltersParameters::ScanDefinition(_))));
        assert!(matches!(scan_filters.get_parameter("Polarity"), Some(ScanFiltersParameters::Polarity(_))));
        assert!(matches!(scan_filters.get_parameter("Spectrum type"), Some(ScanFiltersParameters::SpectrumType(_))));

        // Test a non-existing parameter
        assert_eq!(scan_filters.get_parameter("Nonexistent"), None);
    }

    #[test]
    fn scan_filters_serialization() -> Result<(), Box<dyn std::error::Error>>{
        let mut scan_filter_obj = ScanFilters::new();

        let scan_number = ScanNumber::new();
        scan_filter_obj.add_parameter(ScanFiltersParameters::ScanNumber(scan_number));

        let base_filtering = BaseFilteringInteger::new();
        scan_filter_obj.add_parameter(ScanFiltersParameters::BaseFilteringInteger(base_filtering));

        let retention_obj = RetentionTime::new();
        scan_filter_obj.add_parameter(ScanFiltersParameters::RetentionTime(retention_obj));

        let mobility_obj = Mobility::new();
        scan_filter_obj.add_parameter(ScanFiltersParameters::Mobility(mobility_obj));

        let mut ms_obj = MSLevelFilter::new();
        ms_obj.set_selected("All MS levels");
        ms_obj.set_value(Some(1));
        scan_filter_obj.add_parameter(ScanFiltersParameters::MSLevelFilter(ms_obj));

        let scan_def_obj = ScanDefinition::new();
        scan_filter_obj.add_parameter(ScanFiltersParameters::ScanDefinition(scan_def_obj));

        let mut polarity = Polarity::new();
        polarity.set_value("Any");
        scan_filter_obj.add_parameter(ScanFiltersParameters::Polarity(polarity));

        let mut spectrum = SpectrumType::new();
        spectrum.set_value("ANY");
        scan_filter_obj.add_parameter(ScanFiltersParameters::SpectrumType(spectrum));

        // continua ad aggiungere parametri
    
        let mut buffer = "".to_owned(); // Create the string buffer for the XML content
        quick_xml::se::to_writer(&mut buffer, &scan_filter_obj)?;
        
        // IMPORTANT
        // serializer print int if float is .0
        let expected = r#"<parameter name="Scan filters" selected="true"><parameter name="Scan number"/><parameter name="Base Filtering Integer"/><parameter name="Retention time"/><parameter name="Mobility"/><parameter name="MS level filter" selected="All MS levels">1</parameter><parameter name="Scan definition"/><parameter name="Polarity">Any</parameter><parameter name="Spectrum type">ANY</parameter></parameter>"#;
        
        assert_eq!(buffer, expected);

        Ok(())
    }

    #[test]
    fn scan_number_initialization(){
        let scan_number = ScanNumber::new();
        assert_eq!(scan_number.get_name(), "Scan number");
        assert_eq!(*scan_number.get_value(), None);
    }

    #[test]
    fn scan_number_get_set_value(){
        let mut scan_number = ScanNumber::new();
        scan_number.set_value(Some(2));
        assert_eq!(*scan_number.get_value(), Some(2));
    }
    #[test]
    fn scan_number_serialization() -> Result<(), Box<dyn std::error::Error>>{
        let scan_number = ScanNumber::new();

        let mut buffer = "".to_owned();
        quick_xml::se::to_writer(&mut buffer, &scan_number)?;

        let expected = r#"<parameter name="Scan number"/>"#;

        assert_eq!(buffer, expected);

        Ok(())
    }

    #[test]
    fn base_filetering_initialization(){
        let base_obj = BaseFilteringInteger::new();
        assert_eq!(base_obj.get_name(), "Base Filtering Integer");
        assert_eq!(*base_obj.get_value(), None);
    }

    #[test]
    fn base_filetering_get_set_value(){
        let mut base_obj = RetentionTime::new();
        base_obj.set_value(Some(2));
        assert_eq!(*base_obj.get_value(), Some(2));
    }

    #[test]
    fn base_filtering_serialization() -> Result<(), Box<dyn std::error::Error>>{
        let base = BaseFilteringInteger::new();

        let mut buffer = "".to_owned();
        quick_xml::se::to_writer(&mut buffer, &base)?;

        let expected = r#"<parameter name="Base Filtering Integer"/>"#;

        assert_eq!(buffer, expected);

        Ok(())
    }

    #[test]
    fn retention_time_initialization(){
        let retention = RetentionTime::new();
        assert_eq!(retention.get_name(), "Retention time");
        assert_eq!(*retention.get_value(), None);
    }

    #[test]
    fn retention_time_get_set_value(){
        let mut retention = RetentionTime::new();
        retention.set_value(Some(2));
        assert_eq!(*retention.get_value(), Some(2));
    }

    #[test]
    fn retention_time_serialization() -> Result<(), Box<dyn std::error::Error>>{
        let retention = RetentionTime::new();

        let mut buffer = "".to_owned();
        quick_xml::se::to_writer(&mut buffer, &retention)?;

        let expected = r#"<parameter name="Retention time"/>"#;

        assert_eq!(buffer, expected);

        Ok(())
    }

    #[test]
    fn mobility_initialization(){
        let mobility = Mobility::new();
        assert_eq!(mobility.get_name(), "Mobility");
        assert_eq!(*mobility.get_value(), None);
    }

    #[test]
    fn mobility_get_set_value(){
        let mut mobility = Mobility::new();
        mobility.set_value(Some(2));
        assert_eq!(*mobility.get_value(), Some(2));
    }

    #[test]
    fn mobility_serialization() -> Result<(), Box<dyn std::error::Error>>{
        let mobility = Mobility::new();

        let mut buffer = "".to_owned();
        quick_xml::se::to_writer(&mut buffer, &mobility)?;

        let expected = r#"<parameter name="Mobility"/>"#;

        assert_eq!(buffer, expected);

        Ok(())
    }

    #[test]
    fn ms_level_initialization(){
        let ms_obj = MSLevelFilter::new();
        assert_eq!(ms_obj.get_name(), "MS level filter");
        assert_eq!(ms_obj.get_selected(), "");
        assert_eq!(*ms_obj.get_value(), None);
    }

    #[test]
    fn ms_level_get_set_value(){
        let mut ms_obj = MSLevelFilter::new();
        ms_obj.set_value(Some(1));
        assert_eq!(*ms_obj.get_value(), Some(1));
    }

    #[test]
    fn ms_level_get_set_selected(){
        let mut ms_obj = MSLevelFilter::new();
        ms_obj.set_selected("selection");
        assert_eq!(ms_obj.get_selected(), "selection");
    }

    #[test]
    fn ms_level_serialization() -> Result<(), Box<dyn std::error::Error>>{
        let mut ms_obj = MSLevelFilter::new();
        ms_obj.set_selected("All MS levels");
        ms_obj.set_value(Some(1));

        let mut buffer = "".to_owned();
        quick_xml::se::to_writer(&mut buffer, &ms_obj)?;

        let expected = r#"<parameter name="MS level filter" selected="All MS levels">1</parameter>"#;

        assert_eq!(buffer, expected);

        Ok(())
    }

    #[test]
    fn scan_definition_initialization(){
        let defintion = ScanDefinition::new();
        assert_eq!(defintion.get_name(), "Scan definition");
        assert_eq!(*defintion.get_value(), None);
    }

    #[test]
    fn scan_definition_get_set_value(){
        let mut defintion = ScanDefinition::new();
        defintion.set_value(Some(1));
        assert_eq!(*defintion.get_value(), Some(1));
    }

    #[test]
    fn scan_definition_serialization() -> Result<(), Box<dyn std::error::Error>>{
        let mut scan_def = ScanDefinition::new();

        let mut buffer = "".to_owned();
        quick_xml::se::to_writer(&mut buffer, &scan_def)?;

        let expected = r#"<parameter name="Scan definition"/>"#;

        assert_eq!(buffer, expected);

        Ok(())
    }

    #[test]
    fn polarity_initialization(){
        let polarity = Polarity::new();
        assert_eq!(polarity.get_name(), "Polarity");
        assert_eq!(polarity.get_value(), "");
    }

    #[test]
    fn polarity_get_set_value(){
        let mut polarity = Polarity::new();
        polarity.set_value("value");
        assert_eq!(polarity.get_value(), "value");
    }

    #[test]
    fn polarity_serialization() -> Result<(), Box<dyn std::error::Error>>{
        let mut polarity = Polarity::new();
        polarity.set_value("Any");

        let mut buffer = "".to_owned();
        quick_xml::se::to_writer(&mut buffer, &polarity)?;

        let expected = r#"<parameter name="Polarity">Any</parameter>"#;

        assert_eq!(buffer, expected);

        Ok(())
    }

    #[test]
    fn spectrum_type_initialization(){
        let spectrum = SpectrumType::new();
        assert_eq!(spectrum.get_name(), "Spectrum type");
        assert_eq!(spectrum.get_value(), "");
    }

    #[test]
    fn spectrum_type_get_set_value(){
        let mut spectrum = SpectrumType::new();
        spectrum.set_value("value");
        assert_eq!(spectrum.get_value(), "value");
    }

    #[test]
    fn spectrum_type_serialization() -> Result<(), Box<dyn std::error::Error>>{
        let mut spectrum = SpectrumType::new();
        spectrum.set_value("ANY");

        let mut buffer = "".to_owned();
        quick_xml::se::to_writer(&mut buffer, &spectrum)?;

        let expected = r#"<parameter name="Spectrum type">ANY</parameter>"#;

        assert_eq!(buffer, expected);

        Ok(())
    }
}
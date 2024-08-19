use mzbatch_generator::mass_detection_module_parameters::ScanFilters;
use mzbatch_generator::mass_detection_module_parameters::ScanFiltersParameter;
use mzbatch_generator::mass_detection_module_parameters::Mobility;
use mzbatch_generator::mass_detection_module_parameters::ScanNumber;
use mzbatch_generator::mass_detection_module_parameters::BaseFilteringInteger;
use mzbatch_generator::mass_detection_module_parameters::Polarity;
use mzbatch_generator::mass_detection_module_parameters::SpectrumType;
use mzbatch_generator::mass_detection_module_parameters::RetentionTime;
use mzbatch_generator::mass_detection_module_parameters::MSLevelFilter;
use mzbatch_generator::mass_detection_module_parameters::ScanDefinition;


#[cfg(test)]
mod tests {
    use super::*;

    use quick_xml::writer::Writer;
    use std::{io::{Cursor, Result as IoResult}, iter::Scan};

    #[test]
    fn scan_filters_initialization(){
        let scan_filters_obj = ScanFilters::new();
        assert_eq!(scan_filters_obj.get_name(), "Scan filters");
        assert_eq!(scan_filters_obj.get_parameters_length(), 0);
    }

    #[test]
    fn scan_filters_add_parameter(){
        let mut scan_filters_obj = ScanFilters::new();
        assert_eq!(scan_filters_obj.get_parameters_length(), 0);
        scan_filters_obj.add_parameter(ScanFiltersParameter::Mobility(Mobility::new()));
        assert_eq!(scan_filters_obj.get_parameters_length(), 1);
    }

    #[test]
    fn scan_filters_serialization() -> IoResult<()>{
        

        // Create a writer with an in-memory buffer
        let mut writer = Writer::new(Cursor::new(Vec::new()));

        let mut scan_types = ScanFilters::new();

        // Add all sub parameters
        let mut scan_number = ScanNumber::new();
        let mut base = BaseFilteringInteger::new();
        let mut ret = RetentionTime::new();
        let mut mobility = Mobility::new();
        let mut ms = MSLevelFilter::new();
        let mut scan_definition = ScanDefinition::new();
        let mut polarity = Polarity::new();
        let mut spectr = SpectrumType::new();

        ms.set_ms1(Some(1));
        polarity.set_value("Any");
        spectr.set_value("ANY");

        // now push them in scan types
        scan_types.add_parameter(ScanFiltersParameter::ScanNumber(scan_number));
        scan_types.add_parameter(ScanFiltersParameter::BaseFilteringInteger(base));
        scan_types.add_parameter(ScanFiltersParameter::RetentionTime(ret));
        scan_types.add_parameter(ScanFiltersParameter::Mobility(mobility));
        scan_types.add_parameter(ScanFiltersParameter::MSLevelFiler(ms));
        scan_types.add_parameter(ScanFiltersParameter::ScanDefinition(scan_definition));
        scan_types.add_parameter(ScanFiltersParameter::Polarity(polarity));
        scan_types.add_parameter(ScanFiltersParameter::SpectrumType(spectr));


        // Write the ScanTypes element
        scan_types.write_element(&mut writer)?;

        // Convert buffer to string
        let result = writer.into_inner().into_inner();
        let result_str = String::from_utf8(result).expect("Failed to convert result to string");

        // Define the expected XML output
        let expected = r#"<parameter name="Scan filters" selected="true"><parameter name="Scan number"></parameter><parameter name="Base Filtering Integer"></parameter><parameter name="Retention time"></parameter><parameter name="Mobility"></parameter><parameter name="MS level filter" selected="MS1, level = 1">1</parameter><parameter name="Scan definition"></parameter><parameter name="Polarity">Any</parameter><parameter name="Spectrum type">ANY</parameter></parameter>"#;

        // Assert the result matches the expected output
        assert_eq!(result_str, expected);

        Ok(())
    }

    // // //

    #[test]
    fn scan_number_initialization(){
        let scan_number_obj = ScanNumber::new();
        assert_eq!(scan_number_obj.get_name(), "Scan number");
        assert_eq!(scan_number_obj.get_value(), None);
    }

    #[test]
    fn scan_number_set_get_value(){
        let mut scan_number_obj = ScanNumber::new();
        assert_eq!(scan_number_obj.get_value(), None);
        scan_number_obj.set_value(Some(2));
        assert_eq!(scan_number_obj.get_value(), Some(2));
    }

    #[test]
    fn scan_number_serialization() -> IoResult<()> {
        // Create a writer with an in-memory buffer
        let mut writer = Writer::new(Cursor::new(Vec::new()));

        let mut scan_number = ScanNumber::new();
        scan_number.set_value(None);

        // Write the ScanTypes element
        scan_number.write_element(&mut writer)?;

        // Convert buffer to string
        let result = writer.into_inner().into_inner();
        let result_str = String::from_utf8(result).expect("Failed to convert result to string");

        // Define the expected XML output
        let expected = r#"<parameter name="Scan number"></parameter>"#;

        // Assert the result matches the expected output
        assert_eq!(result_str, expected);

        Ok(())
    }

    // // // 

    #[test]
    fn base_filtering_integer_initialization(){
        let base_filtering_integer_obj = BaseFilteringInteger::new();
        assert_eq!(base_filtering_integer_obj.get_name(), "Base Filtering Integer");
        assert_eq!(base_filtering_integer_obj.get_value(), None);
    }

    #[test]
    fn base_filtering_integer_set_get_value(){
        let mut base_filtering_integer_obj = BaseFilteringInteger::new();
        assert_eq!(base_filtering_integer_obj.get_value(), None);
        base_filtering_integer_obj.set_value(Some(2));
        assert_eq!(base_filtering_integer_obj.get_value(), Some(2));
    }

    #[test]
    fn base_filtering_integer_serialization() -> IoResult<()> {
        // Create a writer with an in-memory buffer
        let mut writer = Writer::new(Cursor::new(Vec::new()));

        let mut scan_types = BaseFilteringInteger::new();
        scan_types.set_value(None);

        // Write the ScanTypes element
        scan_types.write_element(&mut writer)?;

        // Convert buffer to string
        let result = writer.into_inner().into_inner();
        let result_str = String::from_utf8(result).expect("Failed to convert result to string");

        // Define the expected XML output
        let expected = r#"<parameter name="Base Filtering Integer"></parameter>"#;

        // Assert the result matches the expected output
        assert_eq!(result_str, expected);

        Ok(())
    }


    // // //

    #[test]
    fn retention_time_initialization(){
        let retention_time_obj = RetentionTime::new();
        assert_eq!(retention_time_obj.get_name(), "Retention time");
        assert_eq!(retention_time_obj.get_value(), None);
    }

    #[test]
    fn retention_time_set_get_value(){
        let mut retention_time_obj = RetentionTime::new();
        assert_eq!(retention_time_obj.get_value(), None);
        retention_time_obj.set_value(Some(2));
        assert_eq!(retention_time_obj.get_value(), Some(2));
    }

    #[test]
    fn retention_time_serialization() -> IoResult<()> {
        // Create a writer with an in-memory buffer
        let mut writer = Writer::new(Cursor::new(Vec::new()));

        let mut scan_types = RetentionTime::new();
        scan_types.set_value(None);

        // Write the ScanTypes element
        scan_types.write_element(&mut writer)?;

        // Convert buffer to string
        let result = writer.into_inner().into_inner();
        let result_str = String::from_utf8(result).expect("Failed to convert result to string");

        // Define the expected XML output
        let expected = r#"<parameter name="Retention time"></parameter>"#;

        // Assert the result matches the expected output
        assert_eq!(result_str, expected);

        Ok(())
    }


    // // //

    #[test]
    fn mobility_initialization(){
        let mobility_obj = Mobility::new();
        assert_eq!(mobility_obj.get_name(), "Mobility");
        assert_eq!(mobility_obj.get_value(), None);
    }

    #[test]
    fn mobility_set_get_value(){
        let mut mobility_obj = Mobility::new();
        assert_eq!(mobility_obj.get_value(), None);
        mobility_obj.set_value(Some(2));
        assert_eq!(mobility_obj.get_value(), Some(2));
    }

    #[test]
    fn mobility_serialization() -> IoResult<()> {
        // Create a writer with an in-memory buffer
        let mut writer = Writer::new(Cursor::new(Vec::new()));

        let mut scan_types = Mobility::new();
        scan_types.set_value(None);

        // Write the ScanTypes element
        scan_types.write_element(&mut writer)?;

        // Convert buffer to string
        let result = writer.into_inner().into_inner();
        let result_str = String::from_utf8(result).expect("Failed to convert result to string");

        // Define the expected XML output
        let expected = r#"<parameter name="Mobility"></parameter>"#;

        // Assert the result matches the expected output
        assert_eq!(result_str, expected);

        Ok(())
    }

    // // //

    #[test]
    fn ms_level_initialization(){
        let ms_level_obj = MSLevelFilter::new();
        assert_eq!(ms_level_obj.get_name(), "MS level filter");
        assert_eq!(ms_level_obj.get_value(), None);
    }

    #[test]
    fn ms_level_set_get_value(){
        let mut ms_level_obj = MSLevelFilter::new();
        assert_eq!(ms_level_obj.get_value(), None);
        ms_level_obj.set_value(Some(2));
        assert_eq!(ms_level_obj.get_value(), Some(2));
    }

    #[test]
    fn ms_level_set_ms1(){
        let mut ms_level_obj = MSLevelFilter::new();
        assert_eq!(ms_level_obj.get_selected(), "None selected");
        ms_level_obj.set_ms1(Some(4));
        assert_eq!(ms_level_obj.get_selected(), "MS1, level = 1");
        assert_eq!(ms_level_obj.get_value(), Some(4));
    }

    #[test]
    fn ms_level_set_ms2(){
        let mut ms_level_obj = MSLevelFilter::new();
        assert_eq!(ms_level_obj.get_selected(), "None selected");
        ms_level_obj.set_ms2(Some(3));
        assert_eq!(ms_level_obj.get_selected(), "MSn, level ≥ 2");
        assert_eq!(ms_level_obj.get_value(), Some(3));
    }

    #[test]
    fn ms_level_get_level(){
        let mut ms_level_obj = MSLevelFilter::new();
        assert_eq!(ms_level_obj.get_selected(), "None selected");
        ms_level_obj.set_ms1(Some(4));
        assert_eq!(ms_level_obj.get_selected(), "MS1, level = 1");
    }

    #[test]
    fn ms1_serialization() -> IoResult<()> {
        // Create a writer with an in-memory buffer
        let mut writer = Writer::new(Cursor::new(Vec::new()));

        let mut scan_types = MSLevelFilter::new();
        scan_types.set_ms1(Some(1));

        // Write the ScanTypes element
        scan_types.write_element(&mut writer)?;

        // Convert buffer to string
        let result = writer.into_inner().into_inner();
        let result_str = String::from_utf8(result).expect("Failed to convert result to string");

        // Define the expected XML output
        let expected = r#"<parameter name="MS level filter" selected="MS1, level = 1">1</parameter>"#;

        // Assert the result matches the expected output
        assert_eq!(result_str, expected);

        Ok(())
    }

    #[test]
    fn ms2_serialization() -> IoResult<()> {
        // Create a writer with an in-memory buffer
        let mut writer = Writer::new(Cursor::new(Vec::new()));

        let mut scan_types = MSLevelFilter::new();
        scan_types.set_ms2(Some(3));

        // Write the ScanTypes element
        scan_types.write_element(&mut writer)?;

        // Convert buffer to string
        let result = writer.into_inner().into_inner();
        let result_str = String::from_utf8(result).expect("Failed to convert result to string");

        // Define the expected XML output
        let expected = r#"<parameter name="MS level filter" selected="MSn, level ≥ 2">3</parameter>"#;

        // Assert the result matches the expected output
        assert_eq!(result_str, expected);

        Ok(())
    }

    // // //

    #[test]
    fn scan_definition_initialization(){
        let scan_definition_obj = ScanDefinition::new();
        assert_eq!(scan_definition_obj.get_name(), "Scan definition");
        assert_eq!(scan_definition_obj.get_value(), None);
    }

    #[test]
    fn scan_definition_set_value(){
        let mut scan_definition_obj = ScanDefinition::new();
        assert_eq!(scan_definition_obj.get_value(), None);
        scan_definition_obj.set_value(Some(2));
        assert_eq!(scan_definition_obj.get_value(), Some(2));
    }
    
    #[test]
    fn scan_definition_get_value(){
        let mut scan_definition_obj = ScanDefinition::new();
        assert_eq!(scan_definition_obj.get_value(), None);
        scan_definition_obj.set_value(Some(3));
        assert_eq!(scan_definition_obj.get_value(), Some(3)); 
    }

    #[test]
    fn scan_definition_serialization() -> IoResult<()> {
        // Create a writer with an in-memory buffer
        let mut writer = Writer::new(Cursor::new(Vec::new()));

        let mut scan_types = ScanDefinition::new();
        scan_types.set_value(None);

        // Write the ScanTypes element
        scan_types.write_element(&mut writer)?;

        // Convert buffer to string
        let result = writer.into_inner().into_inner();
        let result_str = String::from_utf8(result).expect("Failed to convert result to string");

        // Define the expected XML output
        let expected = r#"<parameter name="Scan definition"></parameter>"#;

        // Assert the result matches the expected output
        assert_eq!(result_str, expected);

        Ok(())
    }

    // // // 

    #[test]
    fn polarity_initalization(){
        let polarity_obj = Polarity::new();
        assert_eq!(polarity_obj.get_name(), "Polarity");
        assert_eq!(polarity_obj.get_value(), "Any");
    }

    #[test]
    fn polarity_set_get_value(){
        let mut polarity_obj = Polarity::new();
        assert_eq!(polarity_obj.get_value(), "Any");
        polarity_obj.set_value("Another");
        assert_eq!(polarity_obj.get_value(), "Another");
    }

    #[test]
    fn polarity_serialization() -> IoResult<()> {
        // Create a writer with an in-memory buffer
        let mut writer = Writer::new(Cursor::new(Vec::new()));

        let mut scan_types = Polarity::new();
        scan_types.set_value("Any");

        // Write the ScanTypes element
        scan_types.write_element(&mut writer)?;

        // Convert buffer to string
        let result = writer.into_inner().into_inner();
        let result_str = String::from_utf8(result).expect("Failed to convert result to string");

        // Define the expected XML output
        let expected = r#"<parameter name="Polarity">Any</parameter>"#;

        // Assert the result matches the expected output
        assert_eq!(result_str, expected);

        Ok(())
    }


    // // // 

    #[test]
    fn spectrum_type_initialization(){
        let spectrum_type_obj = SpectrumType::new();
        assert_eq!(spectrum_type_obj.get_name(), "Spectrum type");
        assert_eq!(spectrum_type_obj.get_value(), "ANY");
    }

    #[test]
    fn spectrum_type_set_get_value(){
        let mut spectrum_type_obj = SpectrumType::new();
        assert_eq!(spectrum_type_obj.get_value(), "ANY");
        spectrum_type_obj.set_value("Another");
        assert_eq!(spectrum_type_obj.get_value(), "Another");
    }

    #[test]
    fn spectrum_type_serialization() -> IoResult<()> {
        // Create a writer with an in-memory buffer
        let mut writer = Writer::new(Cursor::new(Vec::new()));

        let mut scan_types = SpectrumType::new();
        scan_types.set_value("ANY");

        // Write the ScanTypes element
        scan_types.write_element(&mut writer)?;

        // Convert buffer to string
        let result = writer.into_inner().into_inner();
        let result_str = String::from_utf8(result).expect("Failed to convert result to string");

        // Define the expected XML output
        let expected = r#"<parameter name="Spectrum type">ANY</parameter>"#;

        // Assert the result matches the expected output
        assert_eq!(result_str, expected);

        Ok(())
    }
}
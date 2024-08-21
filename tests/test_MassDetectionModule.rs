use mzbatch_generator::modules::MassDetectionModule;
use mzbatch_generator::modules::MassDetectionModuleParameter;
use mzbatch_generator::mass_detection_module_parameters::RawDataFiles;
use mzbatch_generator::all_spectral_data_import_module_parameters::{ScanFilters, ScanFiltersParameters, MSDetectorAdvanced};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mass_detection_module_initialization(){
        let mass_detection_obj = MassDetectionModule::new();
        assert_eq!(mass_detection_obj.get_method(), "io.github.mzmine.modules.dataprocessing.featdet_massdetection.MassDetectionModule", "NOT right method name");
        assert_eq!(mass_detection_obj.get_parameter_version(), 1, "NOT parameter version 1");
        assert_eq!(mass_detection_obj.get_parameters_length(), 0, "NOT empty parameter vector initialization");
    }

    #[test]
    fn test_mass_detection_module_add_parameter(){
        let mut mass_detection_module_obj = MassDetectionModule::new();
        assert_eq!(mass_detection_module_obj.get_parameters_length(), 0, "NOT empty parameter vector initalization");
        mass_detection_module_obj.add_parameter(MassDetectionModuleParameter::MSDetectorAdvanced(MSDetectorAdvanced::new()));
        assert_eq!(mass_detection_module_obj.get_parameter_version(), 1, "NOT correct length of parameter vector after adding parameter");
    }

    #[test]
    fn mass_detection_module_get_parameter(){
        let mut mass_detection_module_obj = MassDetectionModule::new();
        assert_eq!(mass_detection_module_obj.get_parameters_length(), 0, "NOT empty parameter vector initalization");
        mass_detection_module_obj.add_parameter(MassDetectionModuleParameter::MSDetectorAdvanced(MSDetectorAdvanced::new()));
        let retrieved_parameter = mass_detection_module_obj.get_parameter("MSDetectorAdvanced");
        assert_eq!(retrieved_parameter.get_name(), "");
    }

    #[test]
    fn mass_detection_serialization() -> Result<(), Box<dyn std::error::Error>>{
        let mut buffer = "".to_owned();

        let mut mass_detection = MassDetectionModule::new();

        //Raw data types
        let raw_data = RawDataFiles::new();
        let mut raw_data_buffer = "".to_owned();
        quick_xml::se::to_writer(&mut raw_data_buffer, &raw_data)?;
        let expected = r#"<parameter name="Raw data files" type="BATCH_LAST_FILES"/>"#;
        assert_eq!(raw_data_buffer, expected);
        mass_detection.add_parameter(MassDetectionModuleParameter::RawDataFiles(raw_data));


        //Scan filters
        let mut scan_filters = ScanFilters::default();
        let ms_parameter = scan_filters.get_parameter("MS Level Filter").unwrap();
        assert!(matches!(ms_parameter, ScanFiltersParameters::MSLevelFilter(_)));    //IS INDEED OF THIS TYPE
        assert_eq!(ms_parameter.get_name(), "MS level filter");
        // ms_parameter.set_name("Mass detector");
        // ms_parameter.set_selected("MS1, level = 1");
        // ms_parameter.set_value(Some(1));

        // let mut scan_buffer = "".to_owned();
        // let expected = r#"<parameter name="Scan filters" selected="true"><parameter name="Scan number"/><parameter name="Base Filtering Integer"/><parameter name="Retention time"/><parameter name="Mobility"/><parameter name="MS level filter" selected="MS1, level = 1">1</parameter><parameter name="Scan definition"/><parameter name="Polarity">Any</parameter><parameter name="Spectrum type">ANY</parameter></parameter>"#;
        // quick_xml::se::to_writer(&mut scan_buffer, &scan_filters)?;
        // assert_eq!(scan_buffer, expected);


        // let mut scan_filters_buffer = "".to_owned();

        // quick_xml::se::to_writer(&mut scan_filters_buffer, &scan_filters)?;
        // assert_eq!(scan_filters_buffer, expected);
        // mass_detection.add_parameter(MassDetectionModuleParameter::ScanFilters(scan_filters));
        Ok(())
    }
}
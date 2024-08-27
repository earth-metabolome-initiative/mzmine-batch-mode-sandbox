use mzbatch_generator::modules::{MassDetectionModule, MassDetectionModuleParameter};
use mzbatch_generator::mass_detection_module_parameters::{RawDataFiles, ScanTypes};
use mzbatch_generator::all_spectral_data_import_module_parameters::{ScanFilters, ScanFiltersParameters, MSDetectorAdvanced, DenormalizeFragmentScansTraps};

#[cfg(test)]
mod tests {

    use mzbatch_generator::prelude::MSDetectorAdvancedModules;
    use quick_xml::se::to_writer;

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
    fn mass_detection_ms1_serialization() -> Result<(), Box<dyn std::error::Error>>{
        let mut buffer = "".to_owned();

        let mut mass_detection = MassDetectionModule::new();

        //Raw data types
        let raw_data = RawDataFiles::default();
        let mut raw_data_buffer = "".to_owned();
        quick_xml::se::to_writer(&mut raw_data_buffer, &raw_data)?;
        let mut expected = r#"<parameter name="Raw data files" type="BATCH_LAST_FILES"/>"#;
        assert_eq!(raw_data_buffer, expected);
        mass_detection.add_parameter(MassDetectionModuleParameter::RawDataFiles(raw_data));

        //Scan filters
        let mut scan_filters = ScanFilters::default();
        let ms_parameter = scan_filters.get_parameter("MS Level Filter");
        assert!(matches!(ms_parameter, ScanFiltersParameters::MSLevelFilter(_)));
        assert_eq!(ms_parameter.get_name(), "MS level filter");
        ms_parameter.set_name("MS level filter");
        ms_parameter.set_selected("MS1, level = 1");
        ms_parameter.set_value(Some(1));

        let mut scan_filters_buffer = "".to_owned();
        expected = r#"<parameter name="Scan filters" selected="true"><parameter name="Scan number"/><parameter name="Base Filtering Integer"/><parameter name="Retention time"/><parameter name="Mobility"/><parameter name="MS level filter" selected="MS1, level = 1">1</parameter><parameter name="Scan definition"/><parameter name="Polarity">Any</parameter><parameter name="Spectrum type">ANY</parameter></parameter>"#;
        quick_xml::se::to_writer(&mut scan_filters_buffer, &scan_filters)?;
        assert_eq!(scan_filters_buffer, expected);
        mass_detection.add_parameter(MassDetectionModuleParameter::ScanFilters(scan_filters));

        //Scan types
        let mut scan_types = ScanTypes::new();
        let mut scan_types_buffer = "".to_owned();
        scan_types.set_value("All scan types");

        expected = r#"<parameter name="Scan types (IMS)">All scan types</parameter>"#;
        quick_xml::se::to_writer(&mut scan_types_buffer, &scan_types)?;
        assert_eq!(scan_types_buffer, expected);
        mass_detection.add_parameter(MassDetectionModuleParameter::ScanTypes(scan_types));

        //Mass Detector
        let mut mass_detector = MSDetectorAdvanced::default();
        assert_eq!(mass_detector.get_module_length(), 7);
        mass_detector.set_name("Mass detector");
        mass_detector.not_selected();
        let mut noise_factor = mass_detector.get_module("FactorOfLowestSignal");
        noise_factor.set_value(Some(5.0));
        let mut auto_factor = mass_detector.get_module("Auto");
        auto_factor.set_value(Some(1000.0));

        let mut mass_detector_buffer = "".to_owned();
        expected = r#"<parameter name="Mass detector" selected_item="Factor of lowest signal"><module name="Factor of lowest signal"><parameter name="Noise factor">5</parameter></module><module name="Auto"><parameter name="Noise level">1000</parameter></module><module name="Centroid"><parameter name="Noise level"/></module><module name="Exact mass"><parameter name="Noise level"/></module><module name="Local maxima"><parameter name="Noise level"/></module><module name="Recursive threshold"><parameter name="Noise level"/><parameter name="Min m/z peak width"/><parameter name="Max m/z peak width"/></module><module name="Wavelet transform"><parameter name="Noise level"/><parameter name="Scale level"/><parameter name="Wavelet window size (%)"/></module></parameter>"#;
        quick_xml::se::to_writer(&mut mass_detector_buffer, &mass_detector)?;
        assert_eq!(mass_detector_buffer, expected);
        mass_detection.add_parameter(MassDetectionModuleParameter::MSDetectorAdvanced(mass_detector));

        //Denormalized Fragment Scans Traps
        let mut denormalized_buffer = "".to_owned();
        let denormalized = DenormalizeFragmentScansTraps::new();
        expected = r#"<parameter name="Denormalize fragment scans (traps)">true</parameter>"#;
        quick_xml::se::to_writer(&mut denormalized_buffer, &denormalized)?;
        assert_eq!(denormalized_buffer, expected);
        mass_detection.add_parameter(MassDetectionModuleParameter::DenormalizeFragmentScanTraps(denormalized));

        let expected = r#"<batchstep method="io.github.mzmine.modules.dataprocessing.featdet_massdetection.MassDetectionModule" parameter_version="1"><parameter name="Raw data files" type="BATCH_LAST_FILES"/><parameter name="Scan filters" selected="true"><parameter name="Scan number"/><parameter name="Base Filtering Integer"/><parameter name="Retention time"/><parameter name="Mobility"/><parameter name="MS level filter" selected="MS1, level = 1">1</parameter><parameter name="Scan definition"/><parameter name="Polarity">Any</parameter><parameter name="Spectrum type">ANY</parameter></parameter><parameter name="Scan types (IMS)">All scan types</parameter><parameter name="Mass detector" selected_item="Factor of lowest signal"><module name="Factor of lowest signal"><parameter name="Noise factor">5</parameter></module><module name="Auto"><parameter name="Noise level">1000</parameter></module><module name="Centroid"><parameter name="Noise level"/></module><module name="Exact mass"><parameter name="Noise level"/></module><module name="Local maxima"><parameter name="Noise level"/></module><module name="Recursive threshold"><parameter name="Noise level"/><parameter name="Min m/z peak width"/><parameter name="Max m/z peak width"/></module><module name="Wavelet transform"><parameter name="Noise level"/><parameter name="Scale level"/><parameter name="Wavelet window size (%)"/></module></parameter><parameter name="Denormalize fragment scans (traps)">true</parameter></batchstep>"#;
        quick_xml::se::to_writer(&mut buffer, &mass_detection)?;
        assert_eq!(buffer, expected);
        Ok(())
    }

    #[test]
    fn mass_detection_ms2_serialization() -> Result<(), Box<dyn std::error::Error>>{
        let mut buffer = "".to_owned();

        let mut mass_detection = MassDetectionModule::new();

        //Raw data types
        let raw_data = RawDataFiles::default();
        let mut raw_data_buffer = "".to_owned();
        quick_xml::se::to_writer(&mut raw_data_buffer, &raw_data)?;
        let mut expected = r#"<parameter name="Raw data files" type="BATCH_LAST_FILES"/>"#;
        assert_eq!(raw_data_buffer, expected);
        mass_detection.add_parameter(MassDetectionModuleParameter::RawDataFiles(raw_data));

        //Scan filters
        let mut scan_filters = ScanFilters::default();
        let ms_parameter = scan_filters.get_parameter("MS Level Filter");
        assert!(matches!(ms_parameter, ScanFiltersParameters::MSLevelFilter(_)));
        assert_eq!(ms_parameter.get_name(), "MS level filter");
        ms_parameter.set_name("MS level filter");
        ms_parameter.set_selected("MSn, level ≥ 2");
        ms_parameter.set_value(Some(3));

        let mut scan_filters_buffer = "".to_owned();
        expected = r#"<parameter name="Scan filters" selected="true"><parameter name="Scan number"/><parameter name="Base Filtering Integer"/><parameter name="Retention time"/><parameter name="Mobility"/><parameter name="MS level filter" selected="MSn, level ≥ 2">3</parameter><parameter name="Scan definition"/><parameter name="Polarity">Any</parameter><parameter name="Spectrum type">ANY</parameter></parameter>"#;
        quick_xml::se::to_writer(&mut scan_filters_buffer, &scan_filters)?;
        assert_eq!(scan_filters_buffer, expected);
        mass_detection.add_parameter(MassDetectionModuleParameter::ScanFilters(scan_filters));

        //Scan types
        let mut scan_types = ScanTypes::new();
        let mut scan_types_buffer = "".to_owned();
        scan_types.set_value("All scan types");

        expected = r#"<parameter name="Scan types (IMS)">All scan types</parameter>"#;
        quick_xml::se::to_writer(&mut scan_types_buffer, &scan_types)?;
        assert_eq!(scan_types_buffer, expected);
        mass_detection.add_parameter(MassDetectionModuleParameter::ScanTypes(scan_types));

        //Mass Detector
        let mut mass_detector = MSDetectorAdvanced::default();
        assert_eq!(mass_detector.get_module_length(), 7);
        mass_detector.set_name("Mass detector");
        mass_detector.not_selected();
        let mut noise_factor = mass_detector.get_module("FactorOfLowestSignal");
        noise_factor.set_value(Some(0.0));
        let mut auto_factor = mass_detector.get_module("Auto");
        auto_factor.set_value(Some(1000.0));

        let mut mass_detector_buffer = "".to_owned();
        expected = r#"<parameter name="Mass detector" selected_item="Factor of lowest signal"><module name="Factor of lowest signal"><parameter name="Noise factor">0</parameter></module><module name="Auto"><parameter name="Noise level">1000</parameter></module><module name="Centroid"><parameter name="Noise level"/></module><module name="Exact mass"><parameter name="Noise level"/></module><module name="Local maxima"><parameter name="Noise level"/></module><module name="Recursive threshold"><parameter name="Noise level"/><parameter name="Min m/z peak width"/><parameter name="Max m/z peak width"/></module><module name="Wavelet transform"><parameter name="Noise level"/><parameter name="Scale level"/><parameter name="Wavelet window size (%)"/></module></parameter>"#;
        quick_xml::se::to_writer(&mut mass_detector_buffer, &mass_detector)?;
        assert_eq!(mass_detector_buffer, expected);
        mass_detection.add_parameter(MassDetectionModuleParameter::MSDetectorAdvanced(mass_detector));

        //Denormalized Fragment Scans Traps
        let mut denormalized_buffer = "".to_owned();
        let denormalized = DenormalizeFragmentScansTraps::new();
        expected = r#"<parameter name="Denormalize fragment scans (traps)">true</parameter>"#;
        quick_xml::se::to_writer(&mut denormalized_buffer, &denormalized)?;
        assert_eq!(denormalized_buffer, expected);
        mass_detection.add_parameter(MassDetectionModuleParameter::DenormalizeFragmentScanTraps(denormalized));

        let expected = r#"<batchstep method="io.github.mzmine.modules.dataprocessing.featdet_massdetection.MassDetectionModule" parameter_version="1"><parameter name="Raw data files" type="BATCH_LAST_FILES"/><parameter name="Scan filters" selected="true"><parameter name="Scan number"/><parameter name="Base Filtering Integer"/><parameter name="Retention time"/><parameter name="Mobility"/><parameter name="MS level filter" selected="MSn, level ≥ 2">3</parameter><parameter name="Scan definition"/><parameter name="Polarity">Any</parameter><parameter name="Spectrum type">ANY</parameter></parameter><parameter name="Scan types (IMS)">All scan types</parameter><parameter name="Mass detector" selected_item="Factor of lowest signal"><module name="Factor of lowest signal"><parameter name="Noise factor">0</parameter></module><module name="Auto"><parameter name="Noise level">1000</parameter></module><module name="Centroid"><parameter name="Noise level"/></module><module name="Exact mass"><parameter name="Noise level"/></module><module name="Local maxima"><parameter name="Noise level"/></module><module name="Recursive threshold"><parameter name="Noise level"/><parameter name="Min m/z peak width"/><parameter name="Max m/z peak width"/></module><module name="Wavelet transform"><parameter name="Noise level"/><parameter name="Scale level"/><parameter name="Wavelet window size (%)"/></module></parameter><parameter name="Denormalize fragment scans (traps)">true</parameter></batchstep>"#;
        quick_xml::se::to_writer(&mut buffer, &mass_detection)?;
        assert_eq!(buffer, expected);
        Ok(())
    }
}
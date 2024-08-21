use mzbatch_generator::all_spectral_data_import_module_parameters::*;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn advanced_import_initilization(){
        let advanced_obj = AdvancedImport::new();
        assert_eq!(advanced_obj.get_name(), "Advanced import");
        assert_eq!(advanced_obj.get_parameters_length(), 0);
        assert_eq!(advanced_obj.is_selected(), false);
    }

    #[test]
    fn advanced_import_add_parameter(){
        let mut advanced_obj = AdvancedImport::new();
        assert_eq!(advanced_obj.get_parameters_length(), 0);
        advanced_obj.add_parameter(AdvancedImportParameters::ScanFilters(ScanFilters::new()));
        assert_eq!(advanced_obj.get_parameters_length(), 1);
    }

    #[test]
    fn advanced_import_invert_selected(){
        let mut advanced_obj = AdvancedImport::new();
        assert_eq!(advanced_obj.is_selected(), false);
        advanced_obj.invert_selected();
        assert_eq!(advanced_obj.is_selected(), true);
    }

    #[test]
    fn advanced_import_select(){
        let mut advanced_obj = AdvancedImport::new();
        advanced_obj.select();
        assert_eq!(advanced_obj.is_selected(), true);
    }

    #[test]
    fn advanced_import_deselect(){
        let mut advanced_obj = AdvancedImport::new();
        advanced_obj.select();
        assert_eq!(advanced_obj.is_selected(), true);
        advanced_obj.deselect();
        assert_eq!(advanced_obj.is_selected(), false);
    }

    // ScanFilters
    // CropMS1mz
    // MSDetectorAdvanced
    // DenormalizeFragmentScansTraps

    #[test]
    fn advanced_import_serialization() -> Result<(), Box<dyn std::error::Error>>{
        let mut buffer = "".to_owned();

        let mut advanced_import = AdvancedImport::new();

        // ScanFilters
        let scan_filters = ScanFilters::default();
        let mut scan_filters_buffer = "".to_owned();
        quick_xml::se::to_writer(&mut scan_filters_buffer, &scan_filters)?;
        let scan_filters_expected = r#"<parameter name="Scan filters" selected="true"><parameter name="Scan number"/><parameter name="Base Filtering Integer"/><parameter name="Retention time"/><parameter name="Mobility"/><parameter name="MS level filter" selected="All MS levels">1</parameter><parameter name="Scan definition"/><parameter name="Polarity">Any</parameter><parameter name="Spectrum type">ANY</parameter></parameter>"#;
        assert_eq!(scan_filters_buffer, scan_filters_expected);
        advanced_import.add_parameter(AdvancedImportParameters::ScanFilters(scan_filters));

        //Crop MS1 m/z
        let crop_ms1_mz = CropMS1mz::new();
        let mut crop_buffer = "".to_owned();
        quick_xml::se::to_writer(&mut crop_buffer, &crop_ms1_mz)?;
        let crop_expected = r#"<parameter name="Crop MS1 m/z" selected="false"/>"#;
        assert_eq!(crop_buffer, crop_expected);
        advanced_import.add_parameter(AdvancedImportParameters::CropMS1mz(crop_ms1_mz));

        //MS1 Detector advanced
        let mut ms1_detector_advanced = MSDetectorAdvanced::default();
        ms1_detector_advanced.set_ms1(Some(5.0));
        let mut ms1_detector_buffer = "".to_owned();
        quick_xml::se::to_writer(&mut ms1_detector_buffer, &ms1_detector_advanced)?;
        let expected = r#"<parameter name="MS1 detector (Advanced)" selected="true" selected_item="Factor of lowest signal"><module name="Factor of lowest signal"><parameter name="Noise factor">5</parameter></module><module name="Auto"><parameter name="Noise level">1000</parameter></module><module name="Centroid"><parameter name="Noise level"/></module><module name="Exact mass"><parameter name="Noise level"/></module><module name="Local maxima"><parameter name="Noise level"/></module><module name="Recursive threshold"><parameter name="Noise level"/><parameter name="Min m/z peak width"/><parameter name="Max m/z peak width"/></module><module name="Wavelet transform"><parameter name="Noise level"/><parameter name="Scale level"/><parameter name="Wavelet window size (%)"/></module></parameter>"#;
        assert_eq!(ms1_detector_buffer, expected);
        advanced_import.add_parameter(AdvancedImportParameters::MSDetectorAdvanced(ms1_detector_advanced));

        //MS2 Detector advanced
        let mut ms2_detector_advanced = MSDetectorAdvanced::default();
        ms2_detector_advanced.set_ms2(Some(0.0));
        let mut ms2_detector_buffer = "".to_owned();
        quick_xml::se::to_writer(&mut ms2_detector_buffer, &ms2_detector_advanced)?;
        let expected = r#"<parameter name="MS2 detector (Advanced)" selected="true" selected_item="Factor of lowest signal"><module name="Factor of lowest signal"><parameter name="Noise factor">0</parameter></module><module name="Auto"><parameter name="Noise level">1000</parameter></module><module name="Centroid"><parameter name="Noise level"/></module><module name="Exact mass"><parameter name="Noise level"/></module><module name="Local maxima"><parameter name="Noise level"/></module><module name="Recursive threshold"><parameter name="Noise level"/><parameter name="Min m/z peak width"/><parameter name="Max m/z peak width"/></module><module name="Wavelet transform"><parameter name="Noise level"/><parameter name="Scale level"/><parameter name="Wavelet window size (%)"/></module></parameter>"#;
        assert_eq!(ms2_detector_buffer, expected);
        advanced_import.add_parameter(AdvancedImportParameters::MSDetectorAdvanced(ms2_detector_advanced));

        //Denormalized fragment scan
        let mut denormalized = DenormalizeFragmentScansTraps::new();
        denormalized.set_true();
        let mut denormalized_buffer = "".to_owned();
        quick_xml::se::to_writer(&mut denormalized_buffer, &denormalized)?;
        let expected = r#"<parameter name="Denormalize fragment scans (traps)">true</parameter>"#;
        assert_eq!(denormalized_buffer, expected);
        advanced_import.add_parameter(AdvancedImportParameters::DenormalizeFragmentScansTraps(denormalized));

        //test complete serialization
        quick_xml::se::to_writer(&mut buffer, &advanced_import)?;
        let expected = r#"<parameter name="Advanced import" selected="false"><parameter name="Scan filters" selected="true"><parameter name="Scan number"/><parameter name="Base Filtering Integer"/><parameter name="Retention time"/><parameter name="Mobility"/><parameter name="MS level filter" selected="All MS levels">1</parameter><parameter name="Scan definition"/><parameter name="Polarity">Any</parameter><parameter name="Spectrum type">ANY</parameter></parameter><parameter name="Crop MS1 m/z" selected="false"/><parameter name="MS1 detector (Advanced)" selected="true" selected_item="Factor of lowest signal"><module name="Factor of lowest signal"><parameter name="Noise factor">5</parameter></module><module name="Auto"><parameter name="Noise level">1000</parameter></module><module name="Centroid"><parameter name="Noise level"/></module><module name="Exact mass"><parameter name="Noise level"/></module><module name="Local maxima"><parameter name="Noise level"/></module><module name="Recursive threshold"><parameter name="Noise level"/><parameter name="Min m/z peak width"/><parameter name="Max m/z peak width"/></module><module name="Wavelet transform"><parameter name="Noise level"/><parameter name="Scale level"/><parameter name="Wavelet window size (%)"/></module></parameter><parameter name="MS2 detector (Advanced)" selected="true" selected_item="Factor of lowest signal"><module name="Factor of lowest signal"><parameter name="Noise factor">0</parameter></module><module name="Auto"><parameter name="Noise level">1000</parameter></module><module name="Centroid"><parameter name="Noise level"/></module><module name="Exact mass"><parameter name="Noise level"/></module><module name="Local maxima"><parameter name="Noise level"/></module><module name="Recursive threshold"><parameter name="Noise level"/><parameter name="Min m/z peak width"/><parameter name="Max m/z peak width"/></module><module name="Wavelet transform"><parameter name="Noise level"/><parameter name="Scale level"/><parameter name="Wavelet window size (%)"/></module></parameter><parameter name="Denormalize fragment scans (traps)">true</parameter></parameter>"#;
        assert_eq!(buffer, expected);

        Ok(())
    }
}
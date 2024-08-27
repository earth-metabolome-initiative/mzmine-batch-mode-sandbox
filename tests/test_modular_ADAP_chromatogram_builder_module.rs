pub use mzbatch_generator::modules::ModularADAPChromatogramBuilderModule;
pub use mzbatch_generator::modules::ModularADAPChromatogramBuilderModuleParameter;

use mzbatch_generator::prelude::RawDataFiles;

use std::any::Any;

#[cfg(test)]
mod tests {
    use mzbatch_generator::all_spectral_data_import_module_parameters::ScanFilters;

    use super::*;

    #[test]
    fn test_m_adap_chr_builder_module_initialization(){
        let obj = ModularADAPChromatogramBuilderModule::new();
        assert_eq!(obj.get_method(), "io.github.mzmine.modules.dataprocessing.featdet_adapchromatogrambuilder.ModularADAPChromatogramBuilderModule");
        assert_eq!(*obj.get_parameter_version(), 1u8);
        assert_eq!(obj.get_parameters_length(), 0);
    }

    #[test]
    fn test_m_adap_chr_builder_module_add_parameter(){
        let mut obj = ModularADAPChromatogramBuilderModule::new();
        assert_eq!(obj.get_parameters_length(), 0);
        obj.add_parameter(ModularADAPChromatogramBuilderModuleParameter::RawDataFiles(RawDataFiles::new()));
        assert_eq!(obj.get_parameters_length(), 1);
    }

    #[test]
    fn m_adap_chr_builder_module_get_parameter(){
        let mut obj = ModularADAPChromatogramBuilderModule::new();
        assert_eq!(obj.get_parameters_length(), 0);
        obj.add_parameter(ModularADAPChromatogramBuilderModuleParameter::RawDataFiles(RawDataFiles::default()));
        assert_eq!(obj.get_parameters_length(), 1);
        let get_parameter = obj.get_parameter("Raw data files");
        assert_eq!(get_parameter.get_type(), "BATCH_LAST_FILES")
    }

    #[test]
    fn m_adap_chr_builder_module_serialization() -> Result<(), Box<dyn std::error::Error>>{

        let mut m_adap = ModularADAPChromatogramBuilderModule::default();
        let mut m_adap_buffer = "".to_owned();

        let mut expected = r#" "#;

        //change ScanFilters Retention time
        let mut scan_filter_parameter = m_adap.get_parameter("Scan filters");
        // change MS Levle filter selected to MS1, level = 1
        // change Polarity form Any to +
        // add Minimum consecutive scans to 5
        // 10000.0 for minimum intensity for consecutive scans
        // 50000 for absolute height
        // <absolutetolerance>0.002</absolutetolerance><ppmtolerance>10</ppmtolerance> for mz tolerance scan to scan
        // eics for Suffix

        
        // expected = r#"<batchstep method="io.github.mzmine.modules.dataprocessing.featdet_adapchromatogrambuilder.ModularADAPChromatogramBuilderModule" parameter_version="1"><parameter name="Raw data files" type="BATCH_LAST_FILES"/><parameter name="Scan filters" selected="true"><parameter name="Scan number"/><parameter name="Base Filtering Integer"/><parameter name="Retention time"><min>0</min><max>12.01</max></parameter><parameter name="Mobility"/><parameter name="MS level filter" selected="MS1, level = 1">1</parameter><parameter name="Scan definition"/><parameter name="Polarity">+</parameter><parameter name="Spectrum type">ANY</parameter></parameter><parameter name="Minimum consecutive scans">5</parameter><parameter name="Minimum intensity for consecutive scans">10000.0</parameter><parameter name="Minimum absolute height">50000</parameter><parameter name="m/z tolerance (scan-to-scan)"><absolutetolerance>0.002</absolutetolerance><ppmtolerance>10</ppmtolerance></parameter><parameter name="Suffix">eics</parameter><parameter name="Allow single scan chromatograms"/></batchstep>"#;
        // quick_xml::se::to_writer(&mut m_adap_buffer, &m_adap)?;
        // assert_eq!(m_adap_buffer, expected);
        Ok(())
    }
}
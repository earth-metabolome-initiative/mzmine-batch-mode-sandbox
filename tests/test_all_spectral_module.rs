use mzbatch_generator::modules::AllSpectralDataImportModule;
use mzbatch_generator::all_spectral_data_import_module_parameters::*;

#[cfg(test)]
mod tests {
    use mzbatch_generator::{batch, batchsteps::all_spectral_data_import_module_parameters::advanced_import};

    use super::*;

    #[test]
    fn initialization(){
        let all_spectral_data_import_module_obj = AllSpectralDataImportModule::new();
        assert_eq!(all_spectral_data_import_module_obj.method, "io.github.mzmine.modules.io.import_rawdata_all.AllSpectralDataImportModule");
        assert_eq!(all_spectral_data_import_module_obj.parameter_version, 1);
        assert_eq!(all_spectral_data_import_module_obj.parameters.len(), 0);
    }

    #[test]
    fn add_parameter(){
        let mut all_spectral_data_import_module_obj = AllSpectralDataImportModule::new();
        assert_eq!(all_spectral_data_import_module_obj.parameters.len(), 0);
        all_spectral_data_import_module_obj.add_parameter(Parameter::FileNames(FileNames::new()));
        assert_eq!(all_spectral_data_import_module_obj.parameters.len(), 1);       
    }

    #[test]
    fn serialization() -> Result<(), Box< dyn std::error::Error>>{
        let mut buffer = "".to_owned();

        let mut batchstep = AllSpectralDataImportModule::new();

        // FileNames
        // AdvancedImport
        // MetadataFile
        // SpectralLibraryFiles

        let mut file_names = FileNames::new();

        for i in 1..=17 {
            let mut file = InputFile::new();
            file.set_name(&format!("path/to/file{}.mzML", i));
            file_names.add_file_name(file);
        }

        batchstep.add_parameter(Parameter::FileNames(file_names));


        // Advanced import
        let mut advanced_import = AdvancedImport::new();
        advanced_import.deselect();

        // Scan filters
        let scan_filters = ScanFilters::default();
        advanced_import.add_parameter(AdvancedImportParameters::ScanFilters(scan_filters));

        // CropMS1mz
        advanced_import.add_parameter(AdvancedImportParameters::CropMS1mz(CropMS1mz::new()));

        // MS1 and MS2 Detectors
        let mut  ms1_detector = MSDetectorAdvanced::default();
        ms1_detector.set_ms1(Some(5.0));

        let mut ms2_detector = MSDetectorAdvanced::default();
        ms2_detector.set_ms2(Some(0.0));

        advanced_import.add_parameter(AdvancedImportParameters::MSDetectorAdvanced(ms1_detector));
        advanced_import.add_parameter(AdvancedImportParameters::MSDetectorAdvanced(ms2_detector));

        // Denormalized fragment scans traps
        advanced_import.add_parameter(AdvancedImportParameters::DenormalizeFragmentScansTraps(DenormalizeFragmentScansTraps::new()));

        batchstep.add_parameter(Parameter::AdvancedImport(advanced_import));

        // Metadata
        let mut metadata_files = MetaData::new();
        let mut current_file = MetaDataFile::new();
        current_file.set_name("path/to/metadata1.csv");
        metadata_files.set_current_file(current_file);

        for i in 2..=4 {
            let mut file = MetaDataFile::new();
            file.set_name(&format!("path/to/metadata{}.csv", i));
            metadata_files.add_last_file_name(file);
        }

        batchstep.add_parameter(Parameter::MetadataFile(metadata_files));

        // Spectrum library

        batchstep.add_parameter(Parameter::SpectralLibraryFiles(SpectralLibrary::new()));


        quick_xml::se::to_writer(&mut buffer, &batchstep)?;

        //path/to/file1.mzML
        // path/to/metadata4.csv
        let expected = r#"<batchstep method="io.github.mzmine.modules.io.import_rawdata_all.AllSpectralDataImportModule" parameter_version="1"><parameter name="File names"><file>path/to/file1.mzML</file><file>path/to/file2.mzML</file><file>path/to/file3.mzML</file><file>path/to/file4.mzML</file><file>path/to/file5.mzML</file><file>path/to/file6.mzML</file><file>path/to/file7.mzML</file><file>path/to/file8.mzML</file><file>path/to/file9.mzML</file><file>path/to/file10.mzML</file><file>path/to/file11.mzML</file><file>path/to/file12.mzML</file><file>path/to/file13.mzML</file><file>path/to/file14.mzML</file><file>path/to/file15.mzML</file><file>path/to/file16.mzML</file><file>path/to/file17.mzML</file></parameter><parameter name="Advanced import" selected="false"><parameter name="Scan filters" selected="true"><parameter name="Scan number"/><parameter name="Base Filtering Integer"/><parameter name="Retention time"/><parameter name="Mobility"/><parameter name="MS level filter" selected="All MS levels">1</parameter><parameter name="Scan definition"/><parameter name="Polarity">Any</parameter><parameter name="Spectrum type">ANY</parameter></parameter><parameter name="Crop MS1 m/z" selected="false"/><parameter name="MS1 detector (Advanced)" selected="true" selected_item="Factor of lowest signal"><module name="Factor of lowest signal"><parameter name="Noise factor">5</parameter></module><module name="Auto"><parameter name="Noise level">1000</parameter></module><module name="Centroid"><parameter name="Noise level"/></module><module name="Exact mass"><parameter name="Noise level"/></module><module name="Local maxima"><parameter name="Noise level"/></module><module name="Recursive threshold"><parameter name="Noise level"/><parameter name="Min m/z peak width"/><parameter name="Max m/z peak width"/></module><module name="Wavelet transform"><parameter name="Noise level"/><parameter name="Scale level"/><parameter name="Wavelet window size (%)"/></module></parameter><parameter name="MS2 detector (Advanced)" selected="true" selected_item="Factor of lowest signal"><module name="Factor of lowest signal"><parameter name="Noise factor">0</parameter></module><module name="Auto"><parameter name="Noise level">1000</parameter></module><module name="Centroid"><parameter name="Noise level"/></module><module name="Exact mass"><parameter name="Noise level"/></module><module name="Local maxima"><parameter name="Noise level"/></module><module name="Recursive threshold"><parameter name="Noise level"/><parameter name="Min m/z peak width"/><parameter name="Max m/z peak width"/></module><module name="Wavelet transform"><parameter name="Noise level"/><parameter name="Scale level"/><parameter name="Wavelet window size (%)"/></module></parameter><parameter name="Denormalize fragment scans (traps)">true</parameter></parameter><parameter name="Metadata file" selected="true"><current_file>path/to/metadata1.csv</current_file><last_file>path/to/metadata2.csv</last_file><last_file>path/to/metadata3.csv</last_file><last_file>path/to/metadata4.csv</last_file></parameter><parameter name="Spectral library files"/></batchstep>"#;

        assert_eq!(buffer, expected);
        Ok(())
    }
}
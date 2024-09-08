use mzbatch_generator::modules::{SiriusExportModule, SiriusExportModuleParameter};
use mzbatch_generator::rows_filter_module_parameters::Parameter;
use mzbatch_generator::all_spectral_data_import_module_parameters::{MetaData, MetaDataFile};
use mzbatch_generator::minimum_search_feature_resolver_module_parameters::{FeatureLists, Ms1Ms2PrecursorTolerance as MZTolerance};
use mzbatch_generator::gnps_fbmn_export_and_submit_module_parameters::{MergeMSMSExperimental as MergeMSMS, MergeMSMSExperimentalParameter as MergeMSMSParameter};

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn sirius_export_module_initialization(){
        let sirius = SiriusExportModule::new();
        assert_eq!(sirius.get_method(), "io.github.mzmine.modules.io.export_features_sirius.SiriusExportModule");
        assert_eq!(*sirius.get_parameter_version(), 1);
        assert_eq!(sirius.get_parameters_length(), 0);
    }

    #[test]
    fn sirius_export_module_add_parameter(){
        let mut sirius = SiriusExportModule::new();
        assert_eq!(sirius.get_parameters_length(), 0);
        sirius.add_parameter(SiriusExportModuleParameter::Parameter(Parameter::new("new", None, "")));
    }

    #[test]
    fn sirius_export_module_serialization() -> Result<(), Box<dyn std::error::Error>>{
        let mut buffer = "".to_owned();
        let mut sirius = SiriusExportModule::new();

        let mut feature_lists = FeatureLists::new();
        feature_lists.set_type("BATCH_LAST_FEATURELISTS");
        sirius.add_parameter(SiriusExportModuleParameter::FeatureLists(feature_lists));

        let mut file_name = MetaData::new("/home/jopitim/mzmine-batch-mode-sandbox/data/mzml/{}_sirius",
                                                    vec![MetaDataFile::new("/tmp/cli/dbgi_batch_00001/{}_sirius"), MetaDataFile::new("/home/jopitim/mzmine-batch-mode-sandbox/data/mzml/mapp_batch_00074_sirius.mgf")]
                                                );
        file_name.set_name("Filename");
        file_name.set_selected_to_none();
        sirius.add_parameter(SiriusExportModuleParameter::FileName(file_name));

        let mut merge_obj = MergeMSMS::new();
        merge_obj.set_name("Merge MS/MS");
        merge_obj.select();
        merge_obj.add_parameter(MergeMSMSParameter::Parameter(Parameter::new("Select spectra to merge", None, "across samples")));
        merge_obj.add_parameter(MergeMSMSParameter::Parameter(Parameter::new("m/z merge mode", None, "weighted average (remove outliers)")));
        merge_obj.add_parameter(MergeMSMSParameter::Parameter(Parameter::new("intensity merge mode", None, "sum intensities")));
        
        let mut expected_mass_deviation = MZTolerance::new();
        expected_mass_deviation.set_name("Expected mass deviation");
        expected_mass_deviation.set_absolute_value(Some(0.001));
        expected_mass_deviation.set_ppm_value(Some(5.0));
        merge_obj.add_parameter(MergeMSMSParameter::ExpectedMassDeviation(expected_mass_deviation));

        merge_obj.add_parameter(MergeMSMSParameter::Parameter(Parameter::new("Cosine threshold (%)", None, "0.7")));
        merge_obj.add_parameter(MergeMSMSParameter::Parameter(Parameter::new("Signal count threshold (%)", None, "0.2")));
        merge_obj.add_parameter(MergeMSMSParameter::Parameter(Parameter::new("Isolation window offset (m/z)", None, "0.0")));
        merge_obj.add_parameter(MergeMSMSParameter::Parameter(Parameter::new("Isolation window width (m/z)", None, "3.0")));
        sirius.add_parameter(SiriusExportModuleParameter::MergeMSMS(merge_obj));

        let mut mz_tolerance = MZTolerance::new();
        mz_tolerance.set_name("m/z tolerance");
        mz_tolerance.set_absolute_value(Some(0.002));
        mz_tolerance.set_ppm_value(Some(5.0));
        sirius.add_parameter(SiriusExportModuleParameter::MZTolerance(mz_tolerance));

        sirius.add_parameter(SiriusExportModuleParameter::Parameter(Parameter::new("Only rows with annotation", None, "false")));
        sirius.add_parameter(SiriusExportModuleParameter::Parameter(Parameter::new("Exclude multiple charge", None, "false")));
        sirius.add_parameter(SiriusExportModuleParameter::Parameter(Parameter::new("Exclude multimers", None, "false")));

        quick_xml::se::to_writer(&mut buffer, &sirius)?;

        let expected = r#"<batchstep method="io.github.mzmine.modules.io.export_features_sirius.SiriusExportModule" parameter_version="1"><parameter name="Feature lists" type="BATCH_LAST_FEATURELISTS"/><parameter name="Filename"><current_file>/home/jopitim/mzmine-batch-mode-sandbox/data/mzml/{}_sirius</current_file><last_file>/tmp/cli/dbgi_batch_00001/{}_sirius</last_file><last_file>/home/jopitim/mzmine-batch-mode-sandbox/data/mzml/mapp_batch_00074_sirius.mgf</last_file></parameter><parameter name="Merge MS/MS" selected="true"><parameter name="Select spectra to merge">across samples</parameter><parameter name="m/z merge mode">weighted average (remove outliers)</parameter><parameter name="intensity merge mode">sum intensities</parameter><parameter name="Expected mass deviation"><absolutetolerance>0.001</absolutetolerance><ppmtolerance>5</ppmtolerance></parameter><parameter name="Cosine threshold (%)">0.7</parameter><parameter name="Signal count threshold (%)">0.2</parameter><parameter name="Isolation window offset (m/z)">0.0</parameter><parameter name="Isolation window width (m/z)">3.0</parameter></parameter><parameter name="m/z tolerance"><absolutetolerance>0.002</absolutetolerance><ppmtolerance>5</ppmtolerance></parameter><parameter name="Only rows with annotation">false</parameter><parameter name="Exclude multiple charge">false</parameter><parameter name="Exclude multimers">false</parameter></batchstep>"#;
        assert_eq!(buffer, expected);
        Ok(())
    }
}
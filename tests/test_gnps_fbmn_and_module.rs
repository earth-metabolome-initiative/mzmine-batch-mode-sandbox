use mzbatch_generator::modules::GnpsFbmnExportAndSubmitModule;

use mzbatch_generator::rows_filter_module_parameters::Parameter;
use mzbatch_generator::minimum_search_feature_resolver_module_parameters::{FeatureLists, Ms1Ms2PrecursorTolerance as ExpectedMassDeviation};
use mzbatch_generator::all_spectral_data_import_module_parameters::{MetaData as FileName, MetaDataFile};
use mzbatch_generator::gnps_fbmn_export_and_submit_module_parameters::{MergeMSMSExperimental, MergeMSMSExperimentalParameter, SubmitToGNPS};

#[cfg(test)]
mod test{
    use mzbatch_generator::modules::GnpsParameter;

    use super::*;

    #[test]
    fn gnps_initialization(){
        let gnps = GnpsFbmnExportAndSubmitModule::new();
        assert_eq!(gnps.get_method(), "io.github.mzmine.modules.io.export_features_gnps.fbmn.GnpsFbmnExportAndSubmitModule");
        assert_eq!(*gnps.get_parameter_version(), 2);
        assert_eq!(gnps.get_parameters_length(), 0);
    }

    #[test]
    fn gnps_add_get_parameter(){
        let mut gnps = GnpsFbmnExportAndSubmitModule::new();
        assert_eq!(gnps.get_parameters_length(), 0);
        gnps.add_parameter( GnpsParameter::Parameter(Parameter::new("parameter", Some(false), "ok")));
        assert_eq!(gnps.get_parameters_length(), 1);
        assert_eq!(gnps.get_parameter("Parameter").unwrap().get_name(), "parameter");
    }

    #[test]
    fn gnps_serialization() -> Result<(), Box<dyn std::error::Error>>{
        let mut buffer = "".to_owned();
        let mut gnps = GnpsFbmnExportAndSubmitModule::new();

        let mut feature_lists = FeatureLists::new();
        feature_lists.set_type("BATCH_LAST_FEATURELISTS");
        gnps.add_parameter(GnpsParameter::FeatureLists(feature_lists));

        let mut file_name = FileName::new("/home/jopitim/mzmine-batch-mode-sandbox/data/mzml/{}",
                                                    vec![MetaDataFile::new("/tmp/cli/dbgi_batch_00001/{}")]);
        file_name.set_name("Filename");
        file_name.set_selected_to_none();

        gnps.add_parameter(GnpsParameter::FileName(file_name));

        let mut merge_obj = MergeMSMSExperimental::new();
        merge_obj.add_parameter(MergeMSMSExperimentalParameter::Parameter(Parameter::new("Select spectra to merge", None, "across samples")));
        merge_obj.add_parameter(MergeMSMSExperimentalParameter::Parameter(Parameter::new("m/z merge mode", None, "weighted average (remove outliers)")));
        merge_obj.add_parameter(MergeMSMSExperimentalParameter::Parameter(Parameter::new("intensity merge mode", None, "sum intensities")));
        
        let mut expected_mass_deviation = ExpectedMassDeviation::new();
        expected_mass_deviation.set_name("Expected mass deviation");
        expected_mass_deviation.set_absolute_value(Some(0.001));
        expected_mass_deviation.set_ppm_value(Some(5.0));
        merge_obj.add_parameter(MergeMSMSExperimentalParameter::ExpectedMassDeviation(expected_mass_deviation));

        merge_obj.add_parameter(MergeMSMSExperimentalParameter::Parameter(Parameter::new("Cosine threshold (%)", None, "0.7")));
        merge_obj.add_parameter(MergeMSMSExperimentalParameter::Parameter(Parameter::new("Signal count threshold (%)", None, "0.2")));
        merge_obj.add_parameter(MergeMSMSExperimentalParameter::Parameter(Parameter::new("Isolation window offset (m/z)", None, "0.0")));
        merge_obj.add_parameter(MergeMSMSExperimentalParameter::Parameter(Parameter::new("Isolation window width (m/z)", None, "3.0")));
        gnps.add_parameter(GnpsParameter::MergeMSMSExperimental(merge_obj));

        gnps.add_parameter(GnpsParameter::Parameter(Parameter::new("Filter rows", None, "MS2 OR ION IDENTITY")));
        gnps.add_parameter(GnpsParameter::Parameter(Parameter::new("Feature intensity", None, "Height")));
        gnps.add_parameter(GnpsParameter::Parameter(Parameter::new("CSV export", None, "SIMPLE")));

        let mut sub_obj = SubmitToGNPS::new();
        sub_obj.add_parameter(Parameter::new("Meta data file", Some(false), ""));
        sub_obj.add_parameter(Parameter::new("Export ion identity networks", None, "true"));
        sub_obj.add_parameter(Parameter::new("Presets", None, "HIGHRES"));
        sub_obj.add_parameter(Parameter::new("Job title", None, ""));
        sub_obj.add_parameter(Parameter::new("Email", None, ""));
        sub_obj.add_parameter(Parameter::new("Username", None, ""));
        sub_obj.add_parameter(Parameter::new("Password", None, ""));
        sub_obj.add_parameter(Parameter::new("Open website", None, "true"));
        gnps.add_parameter(GnpsParameter::SubmitToGNPS(sub_obj));

        gnps.add_parameter(GnpsParameter::Parameter(Parameter::new("Open folder", None, "false")));

        quick_xml::se::to_writer(&mut buffer, &gnps)?;
    
        let expected = r#"<batchstep method="io.github.mzmine.modules.io.export_features_gnps.fbmn.GnpsFbmnExportAndSubmitModule" parameter_version="2"><parameter name="Feature lists" type="BATCH_LAST_FEATURELISTS"/><parameter name="Filename"><current_file>/home/jopitim/mzmine-batch-mode-sandbox/data/mzml/{}</current_file><last_file>/tmp/cli/dbgi_batch_00001/{}</last_file></parameter><parameter name="Merge MS/MS (experimental)" selected="false"><parameter name="Select spectra to merge">across samples</parameter><parameter name="m/z merge mode">weighted average (remove outliers)</parameter><parameter name="intensity merge mode">sum intensities</parameter><parameter name="Expected mass deviation"><absolutetolerance>0.001</absolutetolerance><ppmtolerance>5</ppmtolerance></parameter><parameter name="Cosine threshold (%)">0.7</parameter><parameter name="Signal count threshold (%)">0.2</parameter><parameter name="Isolation window offset (m/z)">0.0</parameter><parameter name="Isolation window width (m/z)">3.0</parameter></parameter><parameter name="Filter rows">MS2 OR ION IDENTITY</parameter><parameter name="Feature intensity">Height</parameter><parameter name="CSV export">SIMPLE</parameter><parameter name="Submit to GNPS" selected="false"><parameter name="Meta data file" selected="false"/><parameter name="Export ion identity networks">true</parameter><parameter name="Presets">HIGHRES</parameter><parameter name="Job title"/><parameter name="Email"/><parameter name="Username"/><parameter name="Password"/><parameter name="Open website">true</parameter></parameter><parameter name="Open folder">false</parameter></batchstep>"#;
        assert_eq!(buffer, expected);
        Ok(())
    }
}
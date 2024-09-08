use mzbatch_generator::gnps_fbmn_export_and_submit_module_parameters::{MergeMSMSExperimental, MergeMSMSExperimentalParameter};
use mzbatch_generator::rows_filter_module_parameters::Parameter;
use mzbatch_generator::minimum_search_feature_resolver_module_parameters::Ms1Ms2PrecursorTolerance as ExpectedMassDeviation;

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn merge_MSMS_experimental_initialization(){
        let merge_obj = MergeMSMSExperimental::new();
        assert_eq!(merge_obj.get_name(), "Merge MS/MS (experimental)");
        assert_eq!(*merge_obj.is_selected(), false);
        assert_eq!(merge_obj.get_parameters_length(), 0);
    }

    #[test]
    fn merge_MSMS_experimental_add_get_parameter(){
        let mut merge_obj = MergeMSMSExperimental::new();
        assert_eq!(merge_obj.get_parameters_length(), 0);

        merge_obj.add_parameter(MergeMSMSExperimentalParameter::Parameter(Parameter::new("This", Some(false), "")));
        assert_eq!(merge_obj.get_parameter("Parameter").unwrap().get_name(), "This");
    }

    #[test]
    fn merge_MSMS_experimental_serialization() -> Result<(), Box<dyn std::error::Error>>{
        let mut buffer = "".to_owned();
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

        quick_xml::se::to_writer(&mut buffer, &merge_obj)?;

        let expected = r#"<parameter name="Merge MS/MS (experimental)" selected="false"><parameter name="Select spectra to merge">across samples</parameter><parameter name="m/z merge mode">weighted average (remove outliers)</parameter><parameter name="intensity merge mode">sum intensities</parameter><parameter name="Expected mass deviation"><absolutetolerance>0.001</absolutetolerance><ppmtolerance>5</ppmtolerance></parameter><parameter name="Cosine threshold (%)">0.7</parameter><parameter name="Signal count threshold (%)">0.2</parameter><parameter name="Isolation window offset (m/z)">0.0</parameter><parameter name="Isolation window width (m/z)">3.0</parameter></parameter>"#;
        assert_eq!(buffer, expected);
        Ok(())
    }
}
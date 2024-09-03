use serde::{Serialize, Deserialize};

use mzbatch_generator::modules::{IsotopeFinderModule, IsotopeFinderModuleParameters};
use mzbatch_generator::rows_filter_module_parameters::Parameter;
use mzbatch_generator::minimum_search_feature_resolver_module_parameters::FeatureLists;
use mzbatch_generator::minimum_search_feature_resolver_module_parameters::Ms1Ms2PrecursorTolerance as MzToleranceFeatureToScan;

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn isotope_finder_module_initialization(){
        let isotope = IsotopeFinderModule::new();
        assert_eq!(isotope.get_method(),  "io.github.mzmine.modules.dataprocessing.filter_isotopefinder.IsotopeFinderModule");
        assert_eq!(*isotope.get_parameter_version(), 1);
        assert_eq!(isotope.get_parameters_length(), 0);
    }

    #[test]
    fn isotope_finder_module_add_get_parameter(){
        let mut isotope = IsotopeFinderModule::new();
        assert_eq!(isotope.get_parameters_length(), 0);
        isotope.add_parameter(IsotopeFinderModuleParameters::Parameter(Parameter::new("This", None, "")));
        assert_eq!(isotope.get_parameters_length(), 1);
        assert_eq!(isotope.get_parameter("Parameter").unwrap().get_name(), "This")
    }

    #[test]
    fn isotope_finder_module_serialization() -> Result<(), Box<dyn std::error::Error>>{
        let mut buffer = "".to_owned();
        let mut isotope = IsotopeFinderModule::new();

        let mut feature_lists = FeatureLists::new();
        feature_lists.set_type("BATCH_LAST_FEATURELISTS");
        isotope.add_parameter(IsotopeFinderModuleParameters::FeatureLists(feature_lists));

        isotope.add_parameter(IsotopeFinderModuleParameters::Parameter(Parameter::new("Chemical elements", None, "H,C,N,O,S")));
       
        let mut mz_parameter = MzToleranceFeatureToScan::new();
        mz_parameter.set_name("m/z tolerance (feature-to-scan)");
        mz_parameter.set_absolute_value(Some(0.0015));
        mz_parameter.set_ppm_value(Some(3.0));
        isotope.add_parameter(IsotopeFinderModuleParameters::MzToleranceFeatureToScan(mz_parameter));

        isotope.add_parameter(IsotopeFinderModuleParameters::Parameter(Parameter::new("Maximum charge of isotope m/z", None, "1")));
        isotope.add_parameter(IsotopeFinderModuleParameters::Parameter(Parameter::new("Search in scans", None, "SINGLE MOST INTENSE")));
        
        quick_xml::se::to_writer(&mut buffer, &isotope)?;

        let expected:&str = r#"<batchstep method="io.github.mzmine.modules.dataprocessing.filter_isotopefinder.IsotopeFinderModule" parameter_version="1"><parameter name="Feature lists" type="BATCH_LAST_FEATURELISTS"/><parameter name="Chemical elements">H,C,N,O,S</parameter><parameter name="m/z tolerance (feature-to-scan)"><absolutetolerance>0.0015</absolutetolerance><ppmtolerance>3</ppmtolerance></parameter><parameter name="Maximum charge of isotope m/z">1</parameter><parameter name="Search in scans">SINGLE MOST INTENSE</parameter></batchstep>"#;
        assert_eq!(buffer, expected);
        Ok(())
    }
}
use mzbatch_generator::modules::{RowsFilterModule, RowsFilterModuleParameters};
use mzbatch_generator::rows_filter_module_parameters::*;

use mzbatch_generator::minimum_search_feature_resolver_module_parameters::FeatureLists;
use mzbatch_generator::isotope_grouper_module_parameters::NameSuffix;
use mzbatch_generator::minimum_search_feature_resolver_module_parameters::Ms1Ms2PrecursorTolerance as MzTolerance;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn rows_filter_module_initialization() {
        let module = RowsFilterModule::new();
        assert_eq!(module.get_method(), "io.github.mzmine.modules.dataprocessing.filter_rowsfilter.RowsFilterModule");
        assert_eq!(module.get_parameters_length(), 0);
        assert_eq!(*module.get_parameter_version(), 2);
    }

    #[test]
    fn rows_filter_module_add_get_parameter() {
        // Implement test logic here
    }

    #[test]
    fn rows_filter_module_serialization() -> Result<(), Box<dyn std::error::Error>> {
        let mut buffer = String::new();
        let mut module = RowsFilterModule::new();

        let mut feature_lists = FeatureLists::new();
        feature_lists.set_type("BATCH_LAST_FEATURELISTS");
        module.add_parameter(RowsFilterModuleParameters::FeatureLists(feature_lists));

        let mut name_suffix = NameSuffix::new();
        name_suffix.set_value("filtered");
        module.add_parameter(RowsFilterModuleParameters::NameSuffix(name_suffix));

        let mut minimum_aligned_feature_samples = MinimumAlignedFeaturesSamples::new();
        minimum_aligned_feature_samples.set_abs_value(Some(1.0));
        minimum_aligned_feature_samples.set_rel_value(Some(0.0));
        minimum_aligned_feature_samples.deselect();
        module.add_parameter(RowsFilterModuleParameters::MinimumAlignedFeaturesSamples(minimum_aligned_feature_samples));

        module.add_parameter(RowsFilterModuleParameters::Parameter(Parameter::new("Minimum features in an isotope pattern", Some(true), "2")));

        let mut validate_13_c = Validate13CIsotopeRows::new();

        let mut mz = MzTolerance::new();
        mz.set_name("m/z tolerance");
        mz.set_absolute_value(Some(5.0E-4));
        mz.set_ppm_value(Some(10.0));
        validate_13_c.add_parameter(Validate13CIsotopeRowsParameter::MzTolerance(mz));
        validate_13_c.add_parameter(Validate13CIsotopeRowsParameter::MaxCharge(MaxCharge::new(Some(1))));
        validate_13_c.add_parameter(Validate13CIsotopeRowsParameter::EstimateMinimumCarbon(EstimateMinimumCarbon::new(true)));
        validate_13_c.add_parameter(Validate13CIsotopeRowsParameter::Remove13C(Remove13C::new(true)));
        validate_13_c.add_parameter(Validate13CIsotopeRowsParameter::ExcludeIsotopes(ExcludeIsotopes::default()));

        module.add_parameter(RowsFilterModuleParameters::Validate13CIsotopeRows(validate_13_c));

        module.add_parameter(RowsFilterModuleParameters::Parameter(Parameter::new("Remove redundant isotope rows", None, "false")));
        module.add_parameter(RowsFilterModuleParameters::Parameter(Parameter::new("m/z", Some(false), "")));
        module.add_parameter(RowsFilterModuleParameters::Parameter(Parameter::new("Retention time", Some(false), "")));

        let features_duration_range = FeaturesDurationRange::new(false, Some(0.0), Some(3.0));
        module.add_parameter(RowsFilterModuleParameters::FeaturesDurationRange(features_duration_range));

        let mut chromatographic_FWHM = ChromatographicFWHM::new();
        chromatographic_FWHM.deselect();
        chromatographic_FWHM.set_max_value(Some(1.0));
        chromatographic_FWHM.set_min_value(Some(0.0));
        module.add_parameter(RowsFilterModuleParameters::ChromatographicFWHM(chromatographic_FWHM));
    
        let mut charge = Charge::new();
        charge.deselect();
        charge.set_min_value(Some(1.0));
        charge.set_max_value(Some(2.0));
        module.add_parameter(RowsFilterModuleParameters::Charge(charge));

        let mut kendrick = KendrickMassDefect::new();

        kendrick.add_parameter(KendrickMassDefectParameters::KendrickMassDefectParameter(KendrickMassDefectParameter::new(Some(0.0), Some(1.0))));
        kendrick.add_parameter(KendrickMassDefectParameters::KendrickMassBase(KendrickMassBase::new()));
        kendrick.add_parameter(KendrickMassDefectParameters::Shift(Shift::new(Some(0.0))));
        kendrick.add_parameter(KendrickMassDefectParameters::ChargeParameter(ChargeParameter::new(Some(1))));
        kendrick.add_parameter(KendrickMassDefectParameters::Divisor(Divisor::new(Some(1))));
        kendrick.add_parameter(KendrickMassDefectParameters::UseRemainderOfKendrickMass(UseRemainderOfKendrickMass::new(false)));

        module.add_parameter(RowsFilterModuleParameters::KendrickMassDefect(kendrick));
        module.add_parameter(RowsFilterModuleParameters::Parameter(Parameter::new("Parameter", None, "No parameters defined")));
        module.add_parameter(RowsFilterModuleParameters::Parameter(Parameter::new("Only identified?", None, "false")));
        module.add_parameter(RowsFilterModuleParameters::Parameter(Parameter::new("Text in identity", Some(false), "")));
        module.add_parameter(RowsFilterModuleParameters::Parameter(Parameter::new("Text in comment", Some(false), "")));
        module.add_parameter(RowsFilterModuleParameters::Parameter(Parameter::new("Keep or remove rows", None, "Keep rows that match all criteria")));
        module.add_parameter(RowsFilterModuleParameters::Parameter(Parameter::new("Feature with MS2 scan", None, "false")));
        module.add_parameter(RowsFilterModuleParameters::Parameter(Parameter::new("Never remove feature with MS2", None, "true")));
        module.add_parameter(RowsFilterModuleParameters::Parameter(Parameter::new("Never remove annotated rows", None, "false")));
        module.add_parameter(RowsFilterModuleParameters::Parameter(Parameter::new("Reset the feature number ID", None, "true")));
        module.add_parameter(RowsFilterModuleParameters::Parameter(Parameter::new("Mass defect", Some(false), "")));
        module.add_parameter(RowsFilterModuleParameters::Parameter(Parameter::new("Original feature list", None, "KEEP")));

        quick_xml::se::to_writer(&mut buffer, &module)?;
        let expected = r#"<batchstep method="io.github.mzmine.modules.dataprocessing.filter_rowsfilter.RowsFilterModule" parameter_version="2"><parameter name="Feature lists" type="BATCH_LAST_FEATURELISTS"/><parameter name="Name suffix">filtered</parameter><parameter name="Minimum aligned features (samples)" selected="false"><abs>1</abs><rel>0</rel></parameter><parameter name="Minimum features in an isotope pattern" selected="true">2</parameter><parameter name="Validate 13C isotope pattern" selected="false"><parameter name="m/z tolerance"><absolutetolerance>0.0005</absolutetolerance><ppmtolerance>10</ppmtolerance></parameter><parameter name="Max charge">1</parameter><parameter name="Estimate minimum carbon">true</parameter><parameter name="Remove if 13C">true</parameter><parameter name="Exclude isotopes">H,C,N,O,S</parameter></parameter><parameter name="Remove redundant isotope rows">false</parameter><parameter name="m/z" selected="false"/><parameter name="Retention time" selected="false"/><parameter name="features duration range" selected="false"><min>0</min><max>3</max></parameter><parameter name="Chromatographic FWHM" selected="false"><min>0</min><max>1</max></parameter><parameter name="Charge" selected="false"><min>1</min><max>2</max></parameter><parameter name="Kendrick mass defect" selected="false"><parameter name="Kendrick mass defect"><min>0</min><max>1</max></parameter><parameter name="Kendrick mass base"/><parameter name="Shift">0</parameter><parameter name="Charge">1</parameter><parameter name="Divisor">1</parameter><parameter name="Use Remainder of Kendrick mass">false</parameter></parameter><parameter name="Parameter">No parameters defined</parameter><parameter name="Only identified?">false</parameter><parameter name="Text in identity" selected="false"/><parameter name="Text in comment" selected="false"/><parameter name="Keep or remove rows">Keep rows that match all criteria</parameter><parameter name="Feature with MS2 scan">false</parameter><parameter name="Never remove feature with MS2">true</parameter><parameter name="Never remove annotated rows">false</parameter><parameter name="Reset the feature number ID">true</parameter><parameter name="Mass defect" selected="false"/><parameter name="Original feature list">KEEP</parameter></batchstep>"#; 
        assert_eq!(buffer, expected);
        Ok(())
    }
}

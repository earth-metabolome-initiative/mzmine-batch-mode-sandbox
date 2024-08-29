use mzbatch_generator::modules::{IsotopeGrouper, IsotopeGrouperParameters};
use mzbatch_generator::isotope_grouper_module_parameters::*;
use mzbatch_generator::minimum_search_feature_resolver_module_parameters::FeatureLists;
use mzbatch_generator::minimum_search_feature_resolver_module_parameters::Ms1Ms2PrecursorTolerance as MzToleranceIntraSample;


#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn isotope_grouper_module_initialization(){
        let iso_obj = IsotopeGrouper::new();
        assert_eq!(iso_obj.get_method(), "io.github.mzmine.modules.dataprocessing.filter_isotopegrouper.IsotopeGrouperModule");
        assert_eq!(*iso_obj.get_parameter_version(), 1);
        assert_eq!(iso_obj.get_parameters_length(), 0);
    }

    #[test]
    fn isotope_grouper_module_add_get_parameter(){
        let mut iso_obj = IsotopeGrouper::new();

        let mut feature = FeatureLists::new();
        feature.set_type("BATCH_LAST_FEATURELISTS");
        iso_obj.add_parameter(IsotopeGrouperParameters::FeatureLists(feature));

        assert_eq!(iso_obj.get_parameter("FeatureLists").get_type().get_str_value(), "BATCH_LAST_FEATURELISTS");


        let mut name_suff = NameSuffix::new();
        name_suff.set_value("deiso");
        iso_obj.add_parameter(IsotopeGrouperParameters::NameSuffix(name_suff));

        assert_eq!(iso_obj.get_parameter("NameSuffix").get_value().get_str_value(), "deiso");

        let mut mz_tol = MzToleranceIntraSample::new_isotope_grouper_module();
        mz_tol.set_absolute_value(Some(0.0015));
        mz_tol.set_ppm_value(Some(3.0));
        iso_obj.add_parameter(IsotopeGrouperParameters::MzToleranceIntraSample(mz_tol));
        assert_eq!(iso_obj.get_parameter("MzToleranceIntraSample").get_name(), "m/z tolerance (intra-sample)");

        let mut retention_obj = RetentionTimeTolerance::new();
        retention_obj.set_unit("MINUTES");
        retention_obj.set_value(Some(0.04));
        iso_obj.add_parameter(IsotopeGrouperParameters::RetentionTimeTolerance(retention_obj));

        assert_eq!(iso_obj.get_parameter("RetentionTimeTolerance").get_unit().get_str_value(), "MINUTES");
        assert_eq!(*iso_obj.get_parameter("RetentionTimeTolerance").get_value().get_float_value(), Some(0.04));

        let mut mobility = MobilityTolerance::new();
        mobility.deselect();
        mobility.set_value(Some(1.0));
        iso_obj.add_parameter(IsotopeGrouperParameters::MobilityTolerance(mobility));

        assert_eq!(*iso_obj.get_parameter("MobilityTolerance").get_value().get_float_value(), Some(1.0));
        assert_eq!(*iso_obj.get_parameter("MobilityTolerance").get_selected().get_bool_value(), false);

        let mut monotonic_shape = MonotonicShape::new();
        monotonic_shape.set_value(true);
        iso_obj.add_parameter(IsotopeGrouperParameters::MonotonicShape(monotonic_shape));
        
        assert_eq!(*iso_obj.get_parameter("MonotonicShape").get_value().get_bool_value(), true);

        let mut maximum_charge = MaximumCharge::new();
        maximum_charge.set_value(Some(2));
        iso_obj.add_parameter(IsotopeGrouperParameters::MaximumCharge(maximum_charge));

        assert_eq!(*iso_obj.get_parameter("MaximumCharge").get_value().get_u8_value(), Some(2));

        let mut representative_isotope = RepresentativeIsotope::new();
        representative_isotope.set_value("Most intense");
        iso_obj.add_parameter(IsotopeGrouperParameters::RepresentativeIsotope(representative_isotope));

        assert_eq!(iso_obj.get_parameter("RepresentativeIsotope").get_value().get_str_value(), "Most intense");

        let mut never_obj = NeverRemoveFeatureWithMs2::new();
        never_obj.set_value(true);
        iso_obj.add_parameter(IsotopeGrouperParameters::NeverRemoveFeatureWithMs2(never_obj));

        assert_eq!(*iso_obj.get_parameter("NeverRemoveFeatureWithMs2").get_value().get_bool_value(), true);

        let mut original_f_list = OriginalFeatureList::new();
        original_f_list.set_value("KEEP");
        iso_obj.add_parameter(IsotopeGrouperParameters::OriginalFeatureList(original_f_list));
        
        assert_eq!(iso_obj.get_parameter("OriginalFeatureList").get_value().get_str_value(), "KEEP");
    }

    #[test]
    fn isotope_grouper_module_serialization() -> Result<(), Box< dyn std::error::Error>>{
        let mut buffer = "".to_owned();

        let mut iso_obj = IsotopeGrouper::new();

        let mut feature = FeatureLists::new();
        feature.set_type("BATCH_LAST_FEATURELISTS");
        iso_obj.add_parameter(IsotopeGrouperParameters::FeatureLists(feature));

        let mut name_suff = NameSuffix::new();
        name_suff.set_value("deiso");
        iso_obj.add_parameter(IsotopeGrouperParameters::NameSuffix(name_suff));

        let mut mz_tol = MzToleranceIntraSample::new_isotope_grouper_module();
        mz_tol.set_absolute_value(Some(0.0015));
        mz_tol.set_ppm_value(Some(3.0));
        iso_obj.add_parameter(IsotopeGrouperParameters::MzToleranceIntraSample(mz_tol));

        let mut retention_obj = RetentionTimeTolerance::new();
        retention_obj.set_unit("MINUTES");
        retention_obj.set_value(Some(0.04));
        iso_obj.add_parameter(IsotopeGrouperParameters::RetentionTimeTolerance(retention_obj));

        let mut mobility = MobilityTolerance::new();
        mobility.deselect();
        mobility.set_value(Some(1.0));
        iso_obj.add_parameter(IsotopeGrouperParameters::MobilityTolerance(mobility));

        let mut monotonic_shape = MonotonicShape::new();
        monotonic_shape.set_value(true);
        iso_obj.add_parameter(IsotopeGrouperParameters::MonotonicShape(monotonic_shape));

        let mut maximum_charge = MaximumCharge::new();
        maximum_charge.set_value(Some(2));
        iso_obj.add_parameter(IsotopeGrouperParameters::MaximumCharge(maximum_charge));

        let mut representative_isotope = RepresentativeIsotope::new();
        representative_isotope.set_value("Most intense");
        iso_obj.add_parameter(IsotopeGrouperParameters::RepresentativeIsotope(representative_isotope));

        let mut never_obj = NeverRemoveFeatureWithMs2::new();
        never_obj.set_value(true);
        iso_obj.add_parameter(IsotopeGrouperParameters::NeverRemoveFeatureWithMs2(never_obj));

        let mut original_f_list = OriginalFeatureList::new();
        original_f_list.set_value("KEEP");
        iso_obj.add_parameter(IsotopeGrouperParameters::OriginalFeatureList(original_f_list));

        quick_xml::se::to_writer(&mut buffer, &iso_obj)?;

        // Define the expected XML output
        let expected = r#"<batchstep method="io.github.mzmine.modules.dataprocessing.filter_isotopegrouper.IsotopeGrouperModule" parameter_version="1"><parameter name="Feature lists" type="BATCH_LAST_FEATURELISTS"/><parameter name="Name suffix">deiso</parameter><parameter name="m/z tolerance (intra-sample)"><absolutetolerance>0.0015</absolutetolerance><ppmtolerance>3</ppmtolerance></parameter><parameter name="Retention time tolerance" unit="MINUTES">0.04</parameter><parameter name="Mobility tolerance" selected="false">1</parameter><parameter name="Monotonic shape">true</parameter><parameter name="Maximum charge">2</parameter><parameter name="Representative isotope">Most intense</parameter><parameter name="Never remove feature with MS2">true</parameter><parameter name="Original feature list">KEEP</parameter></batchstep>"#;

        // Assert the result matches the expected output
        assert_eq!(buffer, expected);
        Ok(())
    }
}
use mzbatch_generator::modules::{MinimumSearchFeatureResolverModule, MinimumSearchFeatureResolverModuleParameters};
use mzbatch_generator::minimum_search_feature_resolver_module_parameters::*;


#[cfg(test)]
mod tests{
    use super::*;

    #[test]
    fn minimum_search_feature_resolver_initialization(){
        let minimum_obj = MinimumSearchFeatureResolverModule::new();
        assert_eq!(minimum_obj.get_method(), "io.github.mzmine.modules.dataprocessing.featdet_chromatogramdeconvolution.minimumsearch.MinimumSearchFeatureResolverModule");
        assert_eq!(*minimum_obj.get_parameter_version(), 2u8);
        assert_eq!(minimum_obj.get_parameters_length(), 0);
    }

    #[test]
    fn minimum_search_feature_resolver_add_get_parameter(){
        let mut minimum_obj = MinimumSearchFeatureResolverModule::new();
        let parameter = MinimumSearchFeatureResolverModuleParameters::Dimension(Dimension::new());
        minimum_obj.add_parameter(parameter);
        assert_eq!(minimum_obj.get_parameters_length(), 1);
        let retrieved = minimum_obj.get_parameter("Dimension").unwrap();
        assert_eq!(retrieved.get_name(), "Dimension");
    }

    #[test]
    fn minimum_search_feature_resolver_module_serialization() -> Result<(), Box<dyn std::error::Error>>{
        let mut buffer = "".to_owned();
        let mut min_search_feature_obj = MinimumSearchFeatureResolverModule::new();

        let mut feature_lists = FeatureLists::new();
        feature_lists.set_type("BATCH_LAST_FEATURELISTS");
        min_search_feature_obj.add_parameter(MinimumSearchFeatureResolverModuleParameters::FeatureList(feature_lists));

        let mut suffix = Suffix::new();
        suffix.set_value('r');
        min_search_feature_obj.add_parameter(MinimumSearchFeatureResolverModuleParameters::Suffix(suffix));

        let mut original_feature_list = OriginalFeatureList::new();
        original_feature_list.set_value("KEEP");
        min_search_feature_obj.add_parameter(MinimumSearchFeatureResolverModuleParameters::OriginalFeatureList(original_feature_list));


        let mut msms_scan_pairing = MsMsScanPairing::new();

        let mut ms1_to_ms2 = Ms1Ms2PrecursorTolerance::new();
        ms1_to_ms2.set_absolute_value(Some(0.01));
        ms1_to_ms2.set_ppm_value(Some(10.0));
        msms_scan_pairing.add_parameter(MsMsScanPairingParameters::Ms1ToMs2PrecursorTolerance(ms1_to_ms2));

        let mut retention_time_filter = RetentionTimeFilter::new();
        retention_time_filter.set_selected("Use feature edges");
        retention_time_filter.set_unit("MINUTES");
        retention_time_filter.set_value(Some(0.05));
        msms_scan_pairing.add_parameter(MsMsScanPairingParameters::RetentionTimeFilter(retention_time_filter));

        let mut min_relative_height = MinimumRelativeFeatureHeight::new();
        min_relative_height.select();
        min_relative_height.set_value(Some(0.25));
        msms_scan_pairing.add_parameter(MsMsScanPairingParameters::MinimumRelativeFeatureHeight(min_relative_height));

        let mut min_required_signals = MinimumRequiredSignals::new();
        min_required_signals.select();
        min_required_signals.set_value(Some(1));
        msms_scan_pairing.add_parameter(MsMsScanPairingParameters::MinimumRequiredSignals(min_required_signals));

        let mut limit_by_ion = LimitByIonMobilityEdges::new();
        limit_by_ion.set_value(true);
        msms_scan_pairing.add_parameter(MsMsScanPairingParameters::LimitByIonMobilityEdges(limit_by_ion));

        let mut merge_msms = MergeMsMsSpectraTIMS::new();
        merge_msms.set_value(false);
        msms_scan_pairing.add_parameter(MsMsScanPairingParameters::MergeMsMsSpectraTIMS(merge_msms));

        let mut minimum_sign_intensity_abs = MinimumSignalIntensityAbsoluteTIMS::new();
        minimum_sign_intensity_abs.deselect();
        minimum_sign_intensity_abs.set_value(Some(0.0));
        msms_scan_pairing.add_parameter(MsMsScanPairingParameters::MinimumSignalIntensityAbsoluteTIMS(minimum_sign_intensity_abs));

        let mut minimum_sign_intensity_rel = MinimumSignalIntensityRelativeTIMS::new();
        minimum_sign_intensity_rel.deselect();
        minimum_sign_intensity_rel.set_value(Some(0.01));
        msms_scan_pairing.add_parameter(MsMsScanPairingParameters::MinimumSignalIntensityRelativeTIMS(minimum_sign_intensity_rel));

        min_search_feature_obj.add_parameter(MinimumSearchFeatureResolverModuleParameters::MsMsScanPairing(msms_scan_pairing));

        let mut dimension = Dimension::new();
        dimension.set_value("Retention time");
        min_search_feature_obj.add_parameter(MinimumSearchFeatureResolverModuleParameters::Dimension(dimension));

        let mut chr_thr = ChromatographicThreshold::new();
        chr_thr.set_value(Some(0.3));
        min_search_feature_obj.add_parameter(MinimumSearchFeatureResolverModuleParameters::ChromaticThreshold(chr_thr));

        let mut min_search_RT = MinimumSearchRangeRTMobilityAbsolute::new();
        min_search_RT.set_value(Some(0.05000000074505806));
        min_search_feature_obj.add_parameter(MinimumSearchFeatureResolverModuleParameters::MinimumSearchRangeRTMobilityAbsolute(min_search_RT));

        let mut min_rel_height = MinimumRelativeHeight::new();
        min_rel_height.set_value(Some(0.0));
        min_search_feature_obj.add_parameter(MinimumSearchFeatureResolverModuleParameters::MinimumRelativeHeight(min_rel_height));

        let mut min_abs_height = MinimumAbsoluteHeight::new();
        min_abs_height.set_value(Some(50000.0));
        min_search_feature_obj.add_parameter(MinimumSearchFeatureResolverModuleParameters::MinimumAbsoluteHeight(min_abs_height));

        let mut min_ratio_of_peak_top_edge = MinRatioOfPeakTopEdge::new();
        min_ratio_of_peak_top_edge.set_value(Some(2.0));
        min_search_feature_obj.add_parameter(MinimumSearchFeatureResolverModuleParameters::MinRatioOfPeakTopEdge(min_ratio_of_peak_top_edge));

        let mut peak_duration = PeakDuration::new();
        peak_duration.set_max_value(Some(1.5000000223517418));
        peak_duration.set_min_value(Some(0.0));
        min_search_feature_obj.add_parameter(MinimumSearchFeatureResolverModuleParameters::PeakDuration(peak_duration));

        let mut minimum_scans = MinimumScansDataPoints::new();
        minimum_scans.set_value(Some(5));
        min_search_feature_obj.add_parameter(MinimumSearchFeatureResolverModuleParameters::MinimumScansDataPoints(minimum_scans));
        

        quick_xml::se::to_writer(&mut buffer, &min_search_feature_obj)?;
        let expected = r#"<batchstep method="io.github.mzmine.modules.dataprocessing.featdet_chromatogramdeconvolution.minimumsearch.MinimumSearchFeatureResolverModule" parameter_version="2"><parameter name="Feature lists" type="BATCH_LAST_FEATURELISTS"/><parameter name="Suffix">r</parameter><parameter name="Original feature list">KEEP</parameter><parameter name="MS/MS scan pairing" selected="true"><parameter name="MS1 to MS2 precursor tolerance (m/z)"><absolutetolerance>0.01</absolutetolerance><ppmtolerance>10</ppmtolerance></parameter><parameter name="Retention time filter" selected="Use feature edges" unit="MINUTES">0.05</parameter><parameter name="Minimum relative feature height" selected="true">0.25</parameter><parameter name="Minimum required signals" selected="true">1</parameter><parameter name="Limit by ion mobility edges">true</parameter><parameter name="Merge MS/MS spectra (TIMS)">false</parameter><parameter name="Minimum signal intensity (absolute, TIMS)" selected="false">0</parameter><parameter name="Minimum signal intensity (relative, TIMS)" selected="false">0.01</parameter></parameter><parameter name="Dimension">Retention time</parameter><parameter name="Chromatographic threshold">0.3</parameter><parameter name="Minimum search range RT/Mobility (absolute)">0.05</parameter><parameter name="Minimum relative height">0</parameter><parameter name="Minimum absolute height">50000</parameter><parameter name="Min ratio of peak top/edge">2</parameter><parameter name="Peak duration range (min/mobility)"><min>0.0000000000000000</min><max>1.5000000223517418</max></parameter><parameter name="Minimum scans (data points)">5</parameter></batchstep>"#;

        assert_eq!(buffer, expected);
        Ok(())
    }
}
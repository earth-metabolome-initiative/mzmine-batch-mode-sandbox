use mzbatch_generator::minimum_search_feature_resolver_module_parameters::{MsMsScanPairing, MsMsScanPairingParameters,
    Ms1Ms2PrecursorTolerance, RetentionTimeFilter, MinimumRelativeFeatureHeight, MinimumRequiredSignals, LimitByIonMobilityEdges, MergeMsMsSpectraTIMS, MinimumSignalIntensityAbsoluteTIMS, MinimumSignalIntensityRelativeTIMS};

#[cfg(test)]
mod test{

    use std::path::absolute;

    use super::*;

    #[test]
    fn ms_ms_scan_pairing_initialization(){
        let ms_obj = MsMsScanPairing::new();
        assert_eq!(ms_obj.get_name(), "MS/MS scan pairing");
        assert_eq!(ms_obj.get_parameters_length(), 0);
    }

    #[test]
    fn ms_ms_scan_pairing_add_get_parameter(){
        let mut ms_obj = MsMsScanPairing::new();
        let parameter = MsMsScanPairingParameters::MinimumRelativeFeatureHeight(MinimumRelativeFeatureHeight::new());
        ms_obj.add_parameter(parameter);
        assert_eq!(ms_obj.get_parameter("MinimumRelativeFeatureHeight").unwrap().get_name(), "Minimum relative feature height");
    }

    #[test]
    fn ms_ms_set_get_value_parameters(){
        let mut ms1_to_ms2__obj = Ms1Ms2PrecursorTolerance::new();
        assert_eq!(*ms1_to_ms2__obj.get_absolute_value(), None);
        assert_eq!(*ms1_to_ms2__obj.get_ppm_value(), None);

        ms1_to_ms2__obj.set_absolute_value(Some(0.01));
        ms1_to_ms2__obj.set_ppm_value(Some(10.0));

        assert_eq!(*ms1_to_ms2__obj.get_absolute_value(), Some(0.01));
        assert_eq!(*ms1_to_ms2__obj.get_ppm_value(), Some(10.0));
    }

    #[test]
    fn ms_ms_scan_pairing_serialization() -> Result<(), Box<dyn std::error::Error>>{
        let mut buffer = "".to_owned();
        let mut ms_obj = MsMsScanPairing::new();

        let mut ms1_to_ms2__obj = Ms1Ms2PrecursorTolerance::new();
        ms1_to_ms2__obj.set_absolute_value(Some(0.01));
        ms1_to_ms2__obj.set_ppm_value(Some(10.0));

        let mut retention_time = RetentionTimeFilter::new();
        retention_time.set_selected("Use feature edges");
        retention_time.set_unit("MINUTES");
        retention_time.set_value(Some(0.05));

        let mut minimum_relative_height = MinimumRelativeFeatureHeight::new();
        minimum_relative_height.select();
        minimum_relative_height.set_value(Some(0.25));

        let mut min_required_signals = MinimumRequiredSignals::new();
        min_required_signals.select();
        min_required_signals.set_value(Some(1));
    
        let mut limit_ion = LimitByIonMobilityEdges::new();
        limit_ion.set_value(true);

        let mut merge_msms = MergeMsMsSpectraTIMS::new();
        merge_msms.set_value(false);

        let mut min_sign_absolute = MinimumSignalIntensityAbsoluteTIMS::new();
        min_sign_absolute.deselect();
        min_sign_absolute.set_value(Some(0.0));

        let mut min_sign_relative = MinimumSignalIntensityRelativeTIMS::new();
        min_sign_relative.deselect();
        min_sign_relative.set_value(Some(0.01));

        let all_parameters: Vec<MsMsScanPairingParameters>= vec![
            MsMsScanPairingParameters::Ms1ToMs2PrecursorTolerance(ms1_to_ms2__obj),
            MsMsScanPairingParameters::RetentionTimeFilter(retention_time),
            MsMsScanPairingParameters::MinimumRelativeFeatureHeight(minimum_relative_height),
            MsMsScanPairingParameters::MinimumRequiredSignals(min_required_signals),
            MsMsScanPairingParameters::LimitByIonMobilityEdges(limit_ion),
            MsMsScanPairingParameters::MergeMsMsSpectraTIMS(merge_msms),
            MsMsScanPairingParameters::MinimumSignalIntensityAbsoluteTIMS(min_sign_absolute),
            MsMsScanPairingParameters::MinimumSignalIntensityRelativeTIMS(min_sign_relative)
        ];

        for parameter in all_parameters{
            ms_obj.add_parameter(parameter);
        }

        quick_xml::se::to_writer(&mut buffer, &ms_obj)?;
        let expected = r#"<parameter name="MS/MS scan pairing" selected="true"><parameter name="MS1 to MS2 precursor tolerance (m/z)"><absolutetolerance>0.01</absolutetolerance><ppmtolerance>10</ppmtolerance></parameter><parameter name="Retention time filter" selected="Use feature edges" unit="MINUTES">0.05</parameter><parameter name="Minimum relative feature height" selected="true">0.25</parameter><parameter name="Minimum required signals" selected="true">1</parameter><parameter name="Limit by ion mobility edges">true</parameter><parameter name="Merge MS/MS spectra (TIMS)">false</parameter><parameter name="Minimum signal intensity (absolute, TIMS)" selected="false">0</parameter><parameter name="Minimum signal intensity (relative, TIMS)" selected="false">0.01</parameter></parameter>"#;
        assert_eq!(buffer, expected);
        Ok(())
    }
}
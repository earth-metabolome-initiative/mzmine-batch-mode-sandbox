use mzbatch_generator::modular_adap_chromatogram_builder_module::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_min_int_cons_scans_initialization(){
        let min_scans_obj = MinimumIntensityForConsecutiveScans::new();
        assert_eq!(min_scans_obj.get_name(), "Minimum intensity for consecutive scans");
        assert_eq!(*min_scans_obj.get_value(), None);
    }

    #[test]
    fn test_min_int_cons_scans_set_get_value(){
        let mut min_scans_obj = MinimumIntensityForConsecutiveScans::new();
        assert_eq!(*min_scans_obj.get_value(), None);
        min_scans_obj.set_value(Some(31.0));
        assert_eq!(*min_scans_obj.get_value(), Some(31.0));
    }
}

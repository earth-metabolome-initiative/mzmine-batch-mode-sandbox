use mzbatch_generator::modular_adap_chromatogram_builder_module_parameters::MinimumConsecutiveScans;
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_consecutive_scans_initialization(){
        let mcs_obj = MinimumConsecutiveScans::new();
        assert_eq!(mcs_obj.get_name(), "Minimum consecutive scans", "NOT right name initialized");
        assert_eq!(*mcs_obj.get_value(), None);
    }

    #[test]
    fn test_minimum_consecutive_scans_set_get_value(){
        let mut mcs_obj = MinimumConsecutiveScans::new();
        mcs_obj.set_value(Some(14));
        assert_eq!(*mcs_obj.get_value(), Some(14));
    }
}

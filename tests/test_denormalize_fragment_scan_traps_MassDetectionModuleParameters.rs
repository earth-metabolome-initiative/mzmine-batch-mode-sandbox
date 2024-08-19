use mzbatch_generator::mass_detection_module_parameters::DenormalizeFragmentScanTraps;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_denormalize_fragment_scan_traps_initialization(){
        let dfst_obj = DenormalizeFragmentScanTraps::new();
        assert_eq!(dfst_obj.get_name(), "Denormalize fragment scans (traps)");
        assert_eq!(*dfst_obj.get_value(), true);
    }

    #[test]
    fn test_denormalize_fragment_scan_traps_set_get_value(){
        let mut dfst_obj = DenormalizeFragmentScanTraps::new();
        assert_eq!(*dfst_obj.get_value(), true);
        dfst_obj.set_value(false);
        assert_eq!(*dfst_obj.get_value(), false);
    }

    #[test]
    fn test_denormalize_fragment_scan_traps_switch_value(){
        let mut dfst_obj = DenormalizeFragmentScanTraps::new();
        assert_eq!(*dfst_obj.get_value(), true);
        dfst_obj.switch_value();
        assert_eq!(*dfst_obj.get_value(), false);
    }
}
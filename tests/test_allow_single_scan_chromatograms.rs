use mzbatch_generator::modular_adap_chromatogram_builder_module_parameters::AllowSingleScanChromatograms;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allow_single_scan_chr_initialization(){
        let a_s_s_chr_obj = AllowSingleScanChromatograms::new();
        assert_eq!(a_s_s_chr_obj.get_name(), "Allow single scan chromatograms");
        assert_eq!(*a_s_s_chr_obj.get_value(), None);
    }

    #[test]
    fn test_allow_single_scan_chr_set_get_value(){
        let mut a_s_s_chr_obj = AllowSingleScanChromatograms::new();
        assert_eq!(*a_s_s_chr_obj.get_value(), None);
        a_s_s_chr_obj.set_value(Some(12.8));
        assert_eq!(*a_s_s_chr_obj.get_value(), Some(12.8));
    }
}
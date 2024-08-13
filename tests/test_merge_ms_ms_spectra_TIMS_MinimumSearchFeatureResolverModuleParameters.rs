use mzbatch_generator::minimum_search_feature_resolver_module::*;

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn merge_ms_ms_spectra_TIMS_initialization(){
        let merge_obj = MergeMsMsSpectraTIMS::new();
        assert_eq!(merge_obj.get_name(), "Merge MS/MS spectra (TIMS)");
        assert_eq!(*merge_obj.get_value(), false);
    }

    #[test]
    fn merge_ms_ms_spectra_TIMS_get_set_invert_value(){
        let mut merge_obj = MergeMsMsSpectraTIMS::new();
        assert_eq!(*merge_obj.get_value(), false);
        merge_obj.invert_value();
        assert_eq!(*merge_obj.get_value(), true);
        merge_obj.set_value(false);
        assert_eq!(*merge_obj.get_value(), false);
    }
}
use mzbatch_generator::minimum_search_feature_resolver_module::*;

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn minimum_signal_intensity_relative_TIMS_initialization(){
        let msirTIMS_obj = MinimumSignalIntensityRelativeTIMS::new();
        assert_eq!(msirTIMS_obj.get_name(), "Minimum signal intensity (relative, TIMS)");
        assert_eq!(msirTIMS_obj.is_selected(), false);
        assert_eq!(*msirTIMS_obj.get_value(), None);
    }

    #[test]
    fn minimum_signal_intensity_relative_TIMS_selection(){
        let mut msirTIMS_obj = MinimumSignalIntensityRelativeTIMS::new();
        assert_eq!(msirTIMS_obj.is_selected(), false);
        msirTIMS_obj.select();
        assert_eq!(msirTIMS_obj.is_selected(), true);
        msirTIMS_obj.deselect();
        assert_eq!(msirTIMS_obj.is_selected(), false);
        msirTIMS_obj.invert_selected();
        assert_eq!(msirTIMS_obj.is_selected(), true);
    }

    #[test]
    fn minimum_signal_intensity_relative_TIMS_get_set_value(){
        let mut msirTIMS_obj = MinimumSignalIntensityRelativeTIMS::new();
        assert_eq!(*msirTIMS_obj.get_value(), None);
        msirTIMS_obj.set_value(Some(2));
        assert_eq!(*msirTIMS_obj.get_value(), Some(2));
    }
}
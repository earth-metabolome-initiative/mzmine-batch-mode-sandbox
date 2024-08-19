use mzbatch_generator::minimum_search_feature_resolver_module_parameters::MinimumSignalIntensityAbsoluteTIMS;

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn minimum_signal_intensity_absolute_TIMS_initialization(){
        let msiaTIMS_obj = MinimumSignalIntensityAbsoluteTIMS::new();
        assert_eq!(msiaTIMS_obj.get_name(), "Minimum signal intensity (absolute, TIMS)");
        assert_eq!(*msiaTIMS_obj.get_value(), None);
        assert_eq!(msiaTIMS_obj.is_selected(), false);
    }

    #[test]
    fn minimum_signal_intensity_absolute_TIMS_selection(){
        let mut msiaTIMS_obj = MinimumSignalIntensityAbsoluteTIMS::new();
        assert_eq!(msiaTIMS_obj.is_selected(), false);
        msiaTIMS_obj.select();
        assert_eq!(msiaTIMS_obj.is_selected(), true);
        msiaTIMS_obj.invert_selected();
        assert_eq!(msiaTIMS_obj.is_selected(), false);
    }

    #[test]
    fn minimum_signal_intensity_absolute_TIMS_set_get_value(){
        let mut msiaTIMS_obj = MinimumSignalIntensityAbsoluteTIMS::new();
        assert_eq!(*msiaTIMS_obj.get_value(), None);
        msiaTIMS_obj.set_value(Some(2.2));
        assert_eq!(*msiaTIMS_obj.get_value(), Some(2.2));
    }
}
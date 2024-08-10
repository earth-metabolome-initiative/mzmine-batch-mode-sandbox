use mzbatch_generator::all_spectral_data_import_module_parameters::*;

#[cfg(test)]
mod tests {
    use super::*;
    // Crop MS1 m/z

    #[test]
    fn crop_msmz_initialization(){
        let crop_msmz_obj = CropMS1mz::new();
        assert_eq!(crop_msmz_obj.get_name(), "Crop MS1 m/z");
        assert_eq!(crop_msmz_obj.is_selected(), false);
        assert_eq!(crop_msmz_obj.get_value(), None);
    }

    #[test]
    fn crop_msmz_selection(){
        let mut crop_msmz_obj = CropMS1mz::new();
        assert_eq!(crop_msmz_obj.is_selected(), false);
        crop_msmz_obj.invert_selected();
        assert_eq!(crop_msmz_obj.is_selected(), true);
        crop_msmz_obj.deselect();
        assert_eq!(crop_msmz_obj.is_selected(), false);
        crop_msmz_obj.select();
        assert_eq!(crop_msmz_obj.is_selected(), true);
    }

    #[test]
    fn crop_msmz_set_value(){
        let mut crop_msmz_obj = CropMS1mz::new();
        assert_eq!(crop_msmz_obj.get_value(), None);
        crop_msmz_obj.set_value(Some(4));
        assert_eq!(crop_msmz_obj.get_value(), Some(4));
    }
}
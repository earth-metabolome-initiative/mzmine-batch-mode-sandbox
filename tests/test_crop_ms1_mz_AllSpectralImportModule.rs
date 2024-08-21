use mzbatch_generator::all_spectral_data_import_module_parameters::*;

#[cfg(test)]
mod tests {
    use quick_xml::writer;

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

    #[test]
    fn crop_msmz_serialization() -> Result<(), Box<dyn std::error::Error>>{
    // <parameter name="Crop MS1 m/z" selected="false"/>
        let mut crop = CropMS1mz::new();
        crop.deselect();

        let mut buffer = "".to_owned();

        quick_xml::se::to_writer(&mut buffer, &crop)?;


        let expected = r#"<parameter name="Crop MS1 m/z" selected="false"/>"#;

        assert_eq!(buffer, expected);
        
        Ok(())
    }
}
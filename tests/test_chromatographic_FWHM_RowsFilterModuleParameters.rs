use  mzbatch_generator::rows_filter_module_parameters::{ChromatographicFWHM, MinMax};

#[cfg(test)]
mod test{

    use super::*;

    #[test]
    fn chromatographic_FWHM_initialization(){
        let chr_obj = ChromatographicFWHM::new();
        assert_eq!(chr_obj.get_name(), "Chromatographic FWHM");
        assert_eq!(*chr_obj.is_selected(), false);
        assert_eq!(*chr_obj.get_max_value(), None);
        assert_eq!(*chr_obj.get_min_value(), None);
    }

    #[test]
    fn chromatographic_FWHM_selection(){
        let mut chr_obj = ChromatographicFWHM::new();
        assert_eq!(*chr_obj.is_selected(), false);
        chr_obj.select();
        assert_eq!(*chr_obj.is_selected(), true);
        chr_obj.deselect();
        assert_eq!(*chr_obj.is_selected(), false);
    }

    #[test]
    fn chromatographic_FWHM_set_get_min_max_value(){
        let mut chr_obj = ChromatographicFWHM::new();
        assert_eq!(*chr_obj.get_max_value(), None);
        assert_eq!(*chr_obj.get_min_value(), None);

        chr_obj.set_min_value(Some(1.1));
        chr_obj.set_max_value(Some(2.2));

        assert_eq!(*chr_obj.get_min_value(), Some(1.1));
        assert_eq!(*chr_obj.get_max_value(), Some(2.2));
    }

    #[test]
    fn chromatographic_FWHM_serialization() -> Result<(), Box<dyn std::error::Error>>{
        let mut buffer = "".to_owned();
        let mut chr_obj = ChromatographicFWHM::new();

        chr_obj.deselect();
        chr_obj.set_max_value(Some(1.0));
        chr_obj.set_min_value(Some(0.0));

        quick_xml::se::to_writer(&mut buffer, &chr_obj)?;
        let expected = r#"<parameter name="Chromatographic FWHM" selected="false"><min>0</min><max>1</max></parameter>"#;
        assert_eq!(buffer, expected);
        Ok(())
    }
}
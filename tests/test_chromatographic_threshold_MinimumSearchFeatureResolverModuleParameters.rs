use mzbatch_generator::minimum_search_feature_resolver_module_parameters::ChromatographicThreshold;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chr_thr_initialization(){
        let chr_obj = ChromatographicThreshold::new();
        assert_eq!(chr_obj.get_name(), "Chromatographic threshold");
        assert_eq!(*chr_obj.get_value(), None);
    }

    #[test]
    fn test_chr_thr_set_get_value(){
        let mut chr_obj = ChromatographicThreshold::new();
        assert_eq!(*chr_obj.get_value(), None);
        chr_obj.set_value(Some(13.8));
        assert_eq!(*chr_obj.get_value(), Some(13.8));
    }

    #[test]
    fn chromatographic_threshold_serialization() -> Result<(), Box<dyn std::error::Error>> {
        let mut buffer = "".to_owned();
        let mut chr_obj = ChromatographicThreshold::new();
        chr_obj.set_value(Some(0.3));

        quick_xml::se::to_writer(&mut buffer, &chr_obj)?;

        let expected = r#"<parameter name="Chromatographic threshold">0.3</parameter>"#;
        
        assert_eq!(buffer, expected);
        
        Ok(())
    }
}
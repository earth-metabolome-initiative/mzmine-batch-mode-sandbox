use mzbatch_generator::minimum_search_feature_resolver_module_parameters::MinimumScansDataPoints;

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn minimum_scans_data_points_initialization(){
        let msd_obj = MinimumScansDataPoints::new();
        assert_eq!(msd_obj.get_name(), "Minimum scans (data points)");
        assert_eq!(*msd_obj.get_value(), None);
    }

    #[test]
    fn minimum_scans_data_points_set_get_value(){
        let mut msd_obj = MinimumScansDataPoints::new();
        assert_eq!(*msd_obj.get_value(), None);
        msd_obj.set_value(Some(3));
        assert_eq!(*msd_obj.get_value(), Some(3));
    }

    #[test]
    fn minimum_scans_data_points_serialization() -> Result<(), Box<dyn std::error::Error>> {
        let mut buffer = "".to_owned();
        let mut chr_obj = MinimumScansDataPoints::new();
        chr_obj.set_value(Some(5));

        quick_xml::se::to_writer(&mut buffer, &chr_obj)?;

        let expected = r#"<parameter name="Minimum scans (data points)">5</parameter>"#;
        
        assert_eq!(buffer, expected);
        
        Ok(())
    }
}
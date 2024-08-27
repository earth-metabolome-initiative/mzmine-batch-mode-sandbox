use mzbatch_generator::minimum_search_feature_resolver_module_parameters::PeakDuration;

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn peak_duration_initialization(){
        let peak = PeakDuration::new();
        assert_eq!(peak.get_name(), "Peak duration range (min/mobility)");
        assert_eq!(peak.get_min_value(), "");
        assert_eq!(peak.get_max_value(), "");
    }

    #[test]
    fn peak_duration_set_get_min_max_value(){
        let mut peak = PeakDuration::new();
        assert_eq!(peak.get_min_value(), "");
        assert_eq!(peak.get_max_value(), "");

        peak.set_min_value(Some(1.1));
        peak.set_max_value(Some(2.2));

        // TODO: Find a way to compare these float - string optimally! 
        // For now I just aimed to get a similar result to verify it has set my value (with some variation because of float rounding error)
        assert_eq!(peak.get_min_value(), "1.1000000000000001");
        assert_eq!(peak.get_max_value(), "2.2000000000000002");
    }

    #[test]
    fn peak_duration_serialization() -> Result<(), Box<dyn std::error::Error>>{
        let mut buffer = "".to_owned();
        let mut peak = PeakDuration::new();

        peak.set_min_value(Some(0.0));
        peak.set_max_value(Some(1.5000000223517418));

        quick_xml::se::to_writer(&mut buffer, &peak)?;
        let expected = r#"<parameter name="Peak duration range (min/mobility)"><min>0.0000000000000000</min><max>1.5000000223517418</max></parameter>"#;
        assert_eq!(buffer, expected);

        Ok(())
    }
}
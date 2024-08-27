use mzbatch_generator::minimum_search_feature_resolver_module_parameters::MinRatioOfPeakTopEdge;

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn minimum_ratio_initialization(){
        let chr_obj = MinRatioOfPeakTopEdge::new();
        assert_eq!(chr_obj.get_name(), "Min ratio of peak top/edge");
        assert_eq!(*chr_obj.get_value(), None);
    }

    #[test]
    fn minimum_ratio_set_get_value(){
        let mut chr_obj = MinRatioOfPeakTopEdge::new();
        assert_eq!(*chr_obj.get_value(), None);
        chr_obj.set_value(Some(4.2));
        assert_eq!(*chr_obj.get_value(), Some(4.2));
    }

    #[test]
    fn minimum_ratio_serialization() -> Result<(), Box<dyn std::error::Error>> {
        let mut buffer = "".to_owned();
        let mut chr_obj = MinRatioOfPeakTopEdge::new();
        chr_obj.set_value(Some(2.0));

        quick_xml::se::to_writer(&mut buffer, &chr_obj)?;

        let expected = r#"<parameter name="Min ratio of peak top/edge">2</parameter>"#;
        
        assert_eq!(buffer, expected);
        
        Ok(())
    }
}
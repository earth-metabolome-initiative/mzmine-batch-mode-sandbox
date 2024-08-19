use mzbatch_generator::minimum_search_feature_resolver_module_parameters::MinRatioOfPeakTopEdge;

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn min_ratio_of_peak_top_edge_initialization(){
        let mropte_obj = MinRatioOfPeakTopEdge::new();
        assert_eq!(mropte_obj.get_name(), "Min ratio of peak top edge");
    }

    #[test]
    fn min_ratio_of_peak_top_edge_set_get_value(){
        let mut mropte_obj = MinRatioOfPeakTopEdge::new();
        assert_eq!(*mropte_obj.get_value(), None);
        mropte_obj.set_value(Some(1.1));
        assert_eq!(*mropte_obj.get_value(), Some(1.1));
    }
}
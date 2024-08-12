use mzbatch_generator::minimum_search_feature_resolver_module::*;

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn limit_by_ion_mobility_edges_initialization(){
        let lbime_obj = LimitByIonMobilityEdges::new();
        assert_eq!(lbime_obj.get_name(), "Limit by ion mobility edges");
        assert_eq!(*lbime_obj.get_value(), true);
    }

    #[test]
    fn limit_by_ion_mobility_edges_set_get_invert_value(){
        let mut lbime_obj = LimitByIonMobilityEdges::new();
        assert_eq!(*lbime_obj.get_value(), true);
        lbime_obj.set_value(false);
        assert_eq!(*lbime_obj.get_value(), false);
        lbime_obj.invert_value();
        assert_eq!(*lbime_obj.get_value(), true);
    }
}
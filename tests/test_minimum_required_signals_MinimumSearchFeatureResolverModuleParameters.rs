use mzbatch_generator::minimum_search_feature_resolver_module_parameters::MinimumRequiredSignals;

#[cfg(test)]
mod test{

    use super::*;

    #[test]
    fn minimum_required_signals_initialization(){
        let mrs_obj = MinimumRequiredSignals::new();
        assert_eq!(mrs_obj.get_name(), "Minimum required signals");
        assert_eq!(*mrs_obj.get_value(), None);
    }

    #[test]
    fn minimum_required_signals_get_set_value(){
        let mut mrs_obj = MinimumRequiredSignals::new();
        assert_eq!(*mrs_obj.get_value(), None);  
        mrs_obj.set_value(Some(23));
        assert_eq!(*mrs_obj.get_value(), Some(23));
    }
}
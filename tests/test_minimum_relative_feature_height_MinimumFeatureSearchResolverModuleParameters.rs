use mzbatch_generator::minimum_search_feature_resolver_module::*;

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn minimum_relative_feature_height_initialization(){
        let mrfh_obj = MinimumRelativeFeatureHeight::new();
        assert_eq!(mrfh_obj.get_name(), "Minimum relative feature height");
        assert_eq!(*mrfh_obj.get_value(), None);
    }

    #[test]
    fn minimum_relative_feature_height_set_get_value(){
        let mut mrfh_obj = MinimumRelativeFeatureHeight::new();
        assert_eq!(*mrfh_obj.get_value(), None); 
        mrfh_obj.set_value(Some(12.3));
        assert_eq!(*mrfh_obj.get_value(), Some(12.3));
    }
}
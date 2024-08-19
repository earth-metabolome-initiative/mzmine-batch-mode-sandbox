use mzbatch_generator::minimum_search_feature_resolver_module_parameters::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_feature_lists_initialization(){
        let feature_lists_obj = FeatureLists::new();
        assert_eq!(feature_lists_obj.get_name(), "Feature lists");
        assert_eq!(feature_lists_obj.get_type(), "BATCH_LAST_FEATURELISTS");
    }

    #[test]
    fn test_feature_lists_set_get_type(){
        let mut feature_lists_obj = FeatureLists::new();
        assert_eq!(feature_lists_obj.get_type(), "BATCH_LAST_FEATURELISTS");
        feature_lists_obj.set_type("newType");
        assert_eq!(feature_lists_obj.get_type(), "newType");
    }
}
use mzbatch_generator::minimum_search_feature_resolver_module_parameters::OriginalFeatureList;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suffix_initialization(){
        let suffix_obj = OriginalFeatureList::new();
        assert_eq!(suffix_obj.get_name(), "Suffix");
        assert_eq!(suffix_obj.get_value(), "KEEP");
    }

    #[test]
    fn test_suffix_set_get_value(){
        let mut suffix_obj = OriginalFeatureList::new();
        assert_eq!(suffix_obj.get_value(), "KEEP");
        suffix_obj.set_value("TEST".to_owned());
        assert_eq!(suffix_obj.get_value(), "TEST");
    }
}
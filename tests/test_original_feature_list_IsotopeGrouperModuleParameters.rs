use mzbatch_generator::isotope_grouper_module_parameters::OriginalFeatureList;

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn original_feature_list_initialization(){
        let mtol_obj = OriginalFeatureList::new();
        assert_eq!(mtol_obj.get_name(), "Original feature list");
        assert_eq!(mtol_obj.get_value(), "");
    }

    #[test]
    fn original_feature_list_get_set_value(){
        let mut mtol_obj = OriginalFeatureList::new();
        assert_eq!(mtol_obj.get_value(), "");
        mtol_obj.set_value("something");
        assert_eq!(mtol_obj.get_value(), "something");
    }
} 
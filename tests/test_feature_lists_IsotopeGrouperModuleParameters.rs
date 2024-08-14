use mzbatch_generator::isotope_grouper_module::*;

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn feature_lists_initialization(){
        let fl_obj  = FeatureLists::new();
        assert_eq!(fl_obj.get_name(), "Feature lists");
        assert_eq!(fl_obj.get_type(), "BATCH_LAST_FEATURELISTS");
    }

    #[test]
    fn feature_lists_get_set_type(){
        let mut fl_obj  = FeatureLists::new();
        assert_eq!(fl_obj.get_type(), "BATCH_LAST_FEATURELISTS");
        fl_obj.set_type("type");
        assert_eq!(fl_obj.get_type(), "type");
    }
}
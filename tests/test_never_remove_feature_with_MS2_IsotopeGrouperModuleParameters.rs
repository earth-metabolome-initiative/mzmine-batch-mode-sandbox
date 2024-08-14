use mzbatch_generator::isotope_grouper_module::*;

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn never_remove_feature_with_MS2_initialization(){
        let mtol_obj = NeverRemoveFeatureWithMs2::new();
        assert_eq!(mtol_obj.get_name(), "Never remove feature with MS2");
        assert_eq!(mtol_obj.get_value(), "");
    }

    #[test]
    fn never_remove_feature_with_MS2_get_set_value(){
        let mut mtol_obj = NeverRemoveFeatureWithMs2::new();
        assert_eq!(mtol_obj.get_value(), "");
        mtol_obj.set_value("something");
        assert_eq!(mtol_obj.get_value(), "something");
    }
} 
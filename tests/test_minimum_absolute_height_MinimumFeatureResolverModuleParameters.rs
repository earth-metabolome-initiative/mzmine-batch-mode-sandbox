use mzbatch_generator::minimum_search_feature_resolver_module_parameters::MinimumAbsoluteHeight;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn min_rel_height_initialization(){
        let min_rel_height_obj = MinimumAbsoluteHeight::new();
        assert_eq!(min_rel_height_obj.get_name(), "Minimum absolute height");
        assert_eq!(*min_rel_height_obj.get_value(), None);
    }

    #[test]
    fn min_rel_height_set_get_value(){
        let mut min_rel_height_obj = MinimumAbsoluteHeight::new();
        assert_eq!(*min_rel_height_obj.get_value(), None);
        min_rel_height_obj.set_value(Some(13.8));
        assert_eq!(*min_rel_height_obj.get_value(), Some(13.8));
    }
}
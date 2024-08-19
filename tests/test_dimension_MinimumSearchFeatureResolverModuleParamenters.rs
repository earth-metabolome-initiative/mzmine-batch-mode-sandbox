use mzbatch_generator::minimum_search_feature_resolver_module_parameters::Dimension;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suffix_initialization(){
        let dimension_obj = Dimension::new();
        assert_eq!(dimension_obj.get_name(), "Dimension");
        assert_eq!(dimension_obj.get_value(), "Retention time");
    }

    #[test]
    fn test_suffix_set_get_value(){
        let mut dimension_obj = Dimension::new();
        assert_eq!(dimension_obj.get_value(), "Retention time");
        dimension_obj.set_value("TEST");
        assert_eq!(dimension_obj.get_value(), "TEST");
    }
}
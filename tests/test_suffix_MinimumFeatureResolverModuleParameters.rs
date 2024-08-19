use mzbatch_generator::minimum_search_feature_resolver_module_parameters::Suffix;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_suffix_initialization(){
        let suffix_obj = Suffix::new();
        assert_eq!(suffix_obj.get_name(), "Suffix");
        assert_eq!(*suffix_obj.get_value(), 'r');
    }

    #[test]
    fn test_suffix_set_value(){
        let mut suffix_obj = Suffix::new();
        assert_eq!(*suffix_obj.get_value(), 'r');
        suffix_obj.set_value('f');
        assert_eq!(*suffix_obj.get_value(), 'f');
    }
}
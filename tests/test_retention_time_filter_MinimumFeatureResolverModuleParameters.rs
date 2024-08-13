use mzbatch_generator::minimum_search_feature_resolver_module::*;

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn retention_time_filter_initialization(){
        let rtf_obj = RetentionTimeFilter::new();
        assert_eq!(rtf_obj.get_name(), "Retention time filter");
        assert_eq!(rtf_obj.get_selected(), "Use feature edges");
        assert_eq!(rtf_obj.get_unit(), "MINUTES");
        assert_eq!(*rtf_obj.get_value(), None);
    }

    #[test]
    fn retention_time_filter_set_get_selected(){
        let mut rtf_obj = RetentionTimeFilter::new();
        assert_eq!(rtf_obj.get_selected(), "Use feature edges");  
        rtf_obj.set_selected("Selected");
        assert_eq!(rtf_obj.get_selected(), "Selected");
    }

    #[test]
    fn retention_time_filter_set_get_unit(){
        let mut rtf_obj = RetentionTimeFilter::new();
        assert_eq!(rtf_obj.get_unit(), "MINUTES");  
        rtf_obj.set_unit("Unit");
        assert_eq!(rtf_obj.get_unit(), "Unit");
    }

    #[test]
    fn retention_time_filter_set_get_value(){
        let mut rtf_obj = RetentionTimeFilter::new();
        assert_eq!(*rtf_obj.get_value(), None);  
        rtf_obj.set_value(Some(24.5));
        assert_eq!(*rtf_obj.get_value(), Some(24.5));
    }
}
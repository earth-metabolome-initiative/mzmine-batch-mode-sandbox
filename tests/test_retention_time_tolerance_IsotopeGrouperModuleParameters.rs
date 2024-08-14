use mzbatch_generator::isotope_grouper_module::*;

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn retention_time_tolerance_initialization(){
        let ret_obj = RetentionTimeTolerance::new();
        assert_eq!(ret_obj.get_name(), "Retention time tolerance");
        assert_eq!(ret_obj.get_unit(), "MINUTES");
        assert_eq!(*ret_obj.get_value(), None);
    }

    #[test]
    fn retention_time_tolerance_get_set_unit(){
        let mut ret_obj = RetentionTimeTolerance::new();
        assert_eq!(ret_obj.get_unit(), "MINUTES");
        ret_obj.set_unit("value");
        assert_eq!(ret_obj.get_unit(), "value");
    }

    #[test]
    fn retention_time_tolerance_get_set_value(){
        let mut ret_obj = RetentionTimeTolerance::new();
        assert_eq!(*ret_obj.get_value(), None);
        ret_obj.set_value(Some(23.4));
        assert_eq!(*ret_obj.get_value(), Some(23.4));
    }
}
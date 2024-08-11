use mzbatch_generator::smoothing_module::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ofl_initialization(){
        let test_ofl_obj = OriginalFeatureList::new();
        assert_eq!{test_ofl_obj.get_name(), "Original feature list"};
        assert_eq!{test_ofl_obj.get_value(), "KEEP"};
    }

    #[test]
    fn test_ofl_set_get_value(){
        let mut test_ofl_obj = OriginalFeatureList::new();
        assert_eq!(test_ofl_obj.get_value(), "KEEP");
        test_ofl_obj.set_value("VALUE");
        assert_eq!(test_ofl_obj.get_value(), "VALUE");
    }
}

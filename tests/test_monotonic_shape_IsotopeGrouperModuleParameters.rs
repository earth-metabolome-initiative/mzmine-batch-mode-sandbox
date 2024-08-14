use mzbatch_generator::isotope_grouper_module::*;

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn monotonic_shape_initialization(){
        let mtol_obj = MonotonicShape::new();
        assert_eq!(mtol_obj.get_name(), "Monotonic shape");
        assert_eq!(*mtol_obj.get_value(), true);
    }

    #[test]
    fn mobility_tolerance_get_set_value(){
        let mut mtol_obj = MonotonicShape::new();
        assert_eq!(*mtol_obj.get_value(), true);
        mtol_obj.set_value(false);
        assert_eq!(*mtol_obj.get_value(), false);
    }
}
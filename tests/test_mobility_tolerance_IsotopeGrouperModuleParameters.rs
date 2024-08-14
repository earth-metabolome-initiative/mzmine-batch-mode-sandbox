use mzbatch_generator::isotope_grouper_module::*;

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn mobility_tolerance_initialization(){
        let mtol_obj = MobilityTolerance::new();
        assert_eq!(mtol_obj.get_name(), "Mobility tolerance");
        assert_eq!(mtol_obj.is_selected(), false);
        assert_eq!(*mtol_obj.get_value(), None);
    }

    #[test]
    fn mobility_tolerance_selection(){
        let mut mtol_obj = MobilityTolerance::new();
        assert_eq!(mtol_obj.is_selected(), false);
        mtol_obj.select();
        assert_eq!(mtol_obj.is_selected(), true);
        mtol_obj.deselect();
        assert_eq!(mtol_obj.is_selected(), false);
        mtol_obj.invert_selected();
        assert_eq!(mtol_obj.is_selected(), true);
    }

    #[test]
    fn mobility_tolerance_get_set_value(){
        let mut mtol_obj = MobilityTolerance::new();
        assert_eq!(*mtol_obj.get_value(), None);
        mtol_obj.set_value(Some(1.1));
        assert_eq!(*mtol_obj.get_value(), Some(1.1));
    }
}
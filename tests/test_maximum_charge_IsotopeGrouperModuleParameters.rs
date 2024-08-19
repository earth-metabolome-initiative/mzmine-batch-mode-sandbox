use mzbatch_generator::isotope_grouper_module_parameters::MaximumCharge;

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn maximum_charge_initialization(){
        let mtol_obj = MaximumCharge::new();
        assert_eq!(mtol_obj.get_name(), "Maximum charge");
        assert_eq!(*mtol_obj.get_value(), None);
    }

    #[test]
    fn maximum_charge_get_set_value(){
        let mut mtol_obj = MaximumCharge::new();
        assert_eq!(*mtol_obj.get_value(), None);
        mtol_obj.set_value(Some(1.2));
        assert_eq!(*mtol_obj.get_value(), Some(1.2));
    }
}
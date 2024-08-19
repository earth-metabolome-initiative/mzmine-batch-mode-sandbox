use mzbatch_generator::isotope_grouper_module_parameters::RepresentativeIsotope;

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn representative_isotope_initialization(){
        let mtol_obj = RepresentativeIsotope::new();
        assert_eq!(mtol_obj.get_name(), "Representative isotope");
        assert_eq!(mtol_obj.get_value(), "");
    }

    #[test]
    fn representative_isotope_get_set_value(){
        let mut mtol_obj = RepresentativeIsotope::new();
        assert_eq!(mtol_obj.get_value(), "");
        mtol_obj.set_value("something");
        assert_eq!(mtol_obj.get_value(), "something");
    }
}
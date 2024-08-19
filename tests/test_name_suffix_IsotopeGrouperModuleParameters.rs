use mzbatch_generator::isotope_grouper_module_parameters::NameSuffix;

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn name_suffix_initialization(){
        let name_obj = NameSuffix::new();
        assert_eq!(name_obj.get_name(), "Name suffix");
        assert_eq!(name_obj.get_value(), "deiso");
    }

    #[test]
    fn name_suffix_get_set_value(){
        let mut name_obj = NameSuffix::new();
        assert_eq!(name_obj.get_value(), "deiso");
        name_obj.set_value("value");
        assert_eq!(name_obj.get_value(), "value");
    }
}
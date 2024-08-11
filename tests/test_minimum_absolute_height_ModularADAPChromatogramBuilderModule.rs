use mzbatch_generator::modular_adap_chromatogram_builder_module::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_minimum_absolute_height_initialization(){
        let minimum_obj = MinimumAbsoluteHeight::new();
        assert_eq!(minimum_obj.get_name(), "Minimum absolute height");
        assert_eq!(*minimum_obj.get_value(), None);
    }

    #[test]
    fn test_minimum_absolute_height_set_get_value(){
        let mut minimum_obj = MinimumAbsoluteHeight::new();
        minimum_obj.set_value(Some(344.5));
        assert_eq!(*minimum_obj.get_value(), Some(344.5));
    }
}
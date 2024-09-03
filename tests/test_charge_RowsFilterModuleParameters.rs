use mzbatch_generator::rows_filter_module_parameters::Charge;

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn charge_initialization(){
        let charge = Charge::new();
        assert_eq!(charge.get_name(), "Charge");
        assert_eq!(*charge.is_selected(), false);
    }

    #[test]
    fn charge_selection(){
        let mut charge = Charge::new();
        assert_eq!(*charge.is_selected(), false);
        charge.select();
        assert_eq!(*charge.is_selected(), true);
        charge.deselect();
        assert_eq!(*charge.is_selected(), false);
    }

    #[test]
    fn charge_serialization() -> Result<(), Box<dyn std::error::Error>>{
        let mut buffer = "".to_owned();
        let mut charge = Charge::new();

        charge.set_min_value(Some(1.0));
        charge.set_max_value(Some(2.0));

        quick_xml::se::to_writer(&mut buffer, &charge)?;

        let expected = r#"<parameter name="Charge" selected="false"><min>1</min><max>2</max></parameter>"#;
        assert_eq!(buffer, expected);
        Ok(())
    }
}
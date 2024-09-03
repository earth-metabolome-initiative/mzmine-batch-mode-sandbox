use mzbatch_generator::rows_filter_module_parameters::*;
use mzbatch_generator::minimum_search_feature_resolver_module_parameters::Ms1Ms2PrecursorTolerance as MzTolerance;

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn validate_13C_isotope_rows_initialization(){
        let validate_obj = Validate13CIsotopeRows::new();
        assert_eq!(validate_obj.get_name(), "Validate 13C isotope pattern");
        assert_eq!(*validate_obj.is_selected(), false);
        assert_eq!(validate_obj.get_parameters_length(), 0);
    }

    #[test]
    fn validate_13C_isotope_rows_selection(){
        let mut validate_obj = Validate13CIsotopeRows::new();
        assert_eq!(*validate_obj.is_selected(), false);
        validate_obj.select(); 
        assert_eq!(*validate_obj.is_selected(), true);
        validate_obj.deselect();
        assert_eq!(*validate_obj.is_selected(), false);
    }

    #[test]
    fn validate_13C_isotope_rows_add_get_parameter(){
        let mut validate_obj = Validate13CIsotopeRows::new();
        let mut mz_parameter = MzTolerance::new();
        mz_parameter.set_name("m/z tolerance");
        let max_charge_parameter = MaxCharge::default();
        let estimate_min_c_obj = EstimateMinimumCarbon::default();
        let remove_if_13_c = Remove13C::default();
        let exclude_isotopes_obj = ExcludeIsotopes::new();

        let parameter: Vec<Validate13CIsotopeRowsParameter> = vec![
            Validate13CIsotopeRowsParameter::MzTolerance(mz_parameter),
            Validate13CIsotopeRowsParameter::MaxCharge(max_charge_parameter),
            Validate13CIsotopeRowsParameter::EstimateMinimumCarbon(estimate_min_c_obj),
            Validate13CIsotopeRowsParameter::Remove13C(remove_if_13_c),
            Validate13CIsotopeRowsParameter::ExcludeIsotopes(exclude_isotopes_obj)
        ];

        for parameter in parameter{
            println!("Parameter name: {}, index: {:?}", parameter.get_name(), validate_obj.get_parameters_length());
            validate_obj.add_parameter(parameter);
        }

        assert_eq!(validate_obj.get_parameters_length(), 5);

        assert_eq!(*validate_obj.get_parameter("MzTolerance").unwrap().get_name(), "m/z tolerance".to_string());
        assert_eq!(*validate_obj.get_parameter("MaxCharge").unwrap().get_name(), "Max charge".to_string());
        assert_eq!(validate_obj.get_parameter("EstimateMinimumCarbon").unwrap().get_name(), "Estimate minimum carbon");
        assert_eq!(validate_obj.get_parameter("Remove13C").unwrap().get_name(), "Remove if 13C");
        assert_eq!(validate_obj.get_parameter("ExcludeIsotopes").unwrap().get_name(), "Exclude isotopes");        
    }

    #[test]
    fn validate_13C_isotope_rows_serializazion() -> Result<(), Box<dyn std::error::Error>>{
        let mut buffer = "".to_owned();
        let mut validate_obj = Validate13CIsotopeRows::new();

        let mut mz_parameter = MzTolerance::new();
        mz_parameter.set_name("m/z tolerance");
        mz_parameter.set_absolute_value(Some(5.0E-4));
        mz_parameter.set_ppm_value(Some(10.0));

        let max_charge_parameter = MaxCharge::new(Some(1));

        let estimate_min_c_obj = EstimateMinimumCarbon::new(true);

        let remove_if_13_c = Remove13C::new(true);

        let mut exclude_isotopes_obj = ExcludeIsotopes::new();
        exclude_isotopes_obj.set_value("H,C,N,O,S");

        let parameter: Vec<Validate13CIsotopeRowsParameter> = vec![
            Validate13CIsotopeRowsParameter::MzTolerance(mz_parameter),
            Validate13CIsotopeRowsParameter::MaxCharge(max_charge_parameter),
            Validate13CIsotopeRowsParameter::EstimateMinimumCarbon(estimate_min_c_obj),
            Validate13CIsotopeRowsParameter::Remove13C(remove_if_13_c),
            Validate13CIsotopeRowsParameter::ExcludeIsotopes(exclude_isotopes_obj)
        ];

        for parameter in parameter{
            validate_obj.add_parameter(parameter)
        }

        assert_eq!(validate_obj.get_parameters_length(), 5);

        quick_xml::se::to_writer(&mut buffer, &validate_obj)?;

        let expected = r#"<parameter name="Validate 13C isotope pattern" selected="false"><parameter name="m/z tolerance"><absolutetolerance>0.0005</absolutetolerance><ppmtolerance>10</ppmtolerance></parameter><parameter name="Max charge">1</parameter><parameter name="Estimate minimum carbon">true</parameter><parameter name="Remove if 13C">true</parameter><parameter name="Exclude isotopes">H,C,N,O,S</parameter></parameter>"#;
        assert_eq!(buffer, expected);
        Ok(())
    }
}
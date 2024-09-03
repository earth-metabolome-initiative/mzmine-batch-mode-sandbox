use mzbatch_generator::rows_filter_module_parameters::*;

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn kendrick_mass_defect_initialization(){
        let kendrick = KendrickMassDefect::new();
        assert_eq!(kendrick.get_name(), "Kendrick mass defect");
        assert_eq!(*kendrick.is_selected(), false);
        assert_eq!(kendrick.get_parameters_length(), 0);
    }

    #[test]
    fn kendrick_mass_defect_selection(){
        let mut kendrick = KendrickMassDefect::new();
        assert_eq!(*kendrick.is_selected(), false);
        kendrick.select();
        assert_eq!(*kendrick.is_selected(), true);
        kendrick.deselect();
        assert_eq!(*kendrick.is_selected(), false);
    }

    #[test]
    fn kendrick_mass_defect_add_get_parameter(){
        let mut kendrick = KendrickMassDefect::new();

        let kendrick_mass_defect_parameter = KendrickMassDefectParameter::default();
        let kendrick_mass_base = KendrickMassBase::new();
        let shift = Shift::default();
        let charge = ChargeParameter::default();
        let divisor = Divisor::default();
        let use_remainder_of_kendrick_mass = UseRemainderOfKendrickMass::default(); 

        kendrick.add_parameter(KendrickMassDefectParameters::KendrickMassDefectParameter(kendrick_mass_defect_parameter));
        kendrick.add_parameter(KendrickMassDefectParameters::KendrickMassBase(kendrick_mass_base));
        kendrick.add_parameter(KendrickMassDefectParameters::Shift(shift));
        kendrick.add_parameter(KendrickMassDefectParameters::ChargeParameter(charge));
        kendrick.add_parameter(KendrickMassDefectParameters::Divisor(divisor));
        kendrick.add_parameter(KendrickMassDefectParameters::UseRemainderOfKendrickMass(use_remainder_of_kendrick_mass));

        assert_eq!(kendrick.get_parameter("KendrickMassDefectParameter").unwrap().get_name(), "Kendrick mass defect");
        assert_eq!(kendrick.get_parameter("KendrickMassBase").unwrap().get_name(), "Kendrick mass base");
        assert_eq!(kendrick.get_parameter("Shift").unwrap().get_name(), "Shift");
        assert_eq!(kendrick.get_parameter("Charge").unwrap().get_name(), "Charge");
        assert_eq!(kendrick.get_parameter("Divisor").unwrap().get_name(), "Divisor");
        assert_eq!(kendrick.get_parameter("UseRemainderOfKendrickMass").unwrap().get_name(), "Use Remainder of Kendrick mass");        
    }

    #[test]
    fn kendrick_mass_defect_serializazion() -> Result<(), Box<dyn std::error::Error>>{
        let mut buffer = "".to_owned();
        let mut kendrick = KendrickMassDefect::new();
        kendrick.deselect();

        kendrick.add_parameter(KendrickMassDefectParameters::KendrickMassDefectParameter( KendrickMassDefectParameter::new(Some(0.0), Some(1.0))));
        kendrick.add_parameter(KendrickMassDefectParameters::KendrickMassBase(KendrickMassBase::new()));
        kendrick.add_parameter(KendrickMassDefectParameters::Shift(Shift::new(Some(0.0))));
        kendrick.add_parameter(KendrickMassDefectParameters::ChargeParameter(ChargeParameter::new(Some(1))));
        kendrick.add_parameter(KendrickMassDefectParameters::Divisor(Divisor::new(Some(1))));
        kendrick.add_parameter(KendrickMassDefectParameters::UseRemainderOfKendrickMass(UseRemainderOfKendrickMass::new(false)));

        quick_xml::se::to_writer(&mut buffer, &kendrick)?;

        let expected = r#"<parameter name="Kendrick mass defect" selected="false"><parameter name="Kendrick mass defect"><min>0</min><max>1</max></parameter><parameter name="Kendrick mass base"/><parameter name="Shift">0</parameter><parameter name="Charge">1</parameter><parameter name="Divisor">1</parameter><parameter name="Use Remainder of Kendrick mass">false</parameter></parameter>"#;
        assert_eq!(buffer, expected);
        Ok(())
    }
}
use mzbatch_generator::minimum_search_feature_resolver_module::*;

#[cfg(test)]
mod test{

    use super::*;

    #[test]
    fn ms1_to_ms2_precursor_tolerance_initialization(){
        let mut ms12pt_obj = Ms1Ms2PrecursorTolerance::new();
        assert_eq!(ms12pt_obj.get_name(), "MS1 to MS2 precursor tolerance (m/z)");
        assert_eq!(ms12pt_obj.get_parameter_length(), 0);
    }

    #[test]
    fn ms1_to_ms2_precursor_tolerance_add_get_parameter(){
        let mut ms12pt_obj = Ms1Ms2PrecursorTolerance::new();
        assert_eq!(ms12pt_obj.get_parameter_length(), 0);
        let mut parameter = AbsoluteTolerance::new();
        parameter.set_value(Some(1.1));
        ms12pt_obj.add_parameter(ToleranceParameters::AbsoluteTolerance(parameter));
        assert_eq!(ms12pt_obj.get_parameter_length(), 1);
        assert_eq!(*ms12pt_obj.get_parameter("Absolutetolerance").unwrap().get_value(), Some(1.1));
        let mut another_parameter = PpmTolerance::new();
        another_parameter.set_value(Some(2.2));
        ms12pt_obj.add_parameter(ToleranceParameters::PpmTolerance(another_parameter));
        assert_eq!(ms12pt_obj.get_parameter_length(), 2);
        assert_eq!(*ms12pt_obj.get_parameter("Ppmtolerance").unwrap().get_value(), Some(2.2));
    }

    #[test]
    fn ppm_tolerance_initialization(){
        let ppm = PpmTolerance::new();
        assert_eq!(*ppm.get_value(), None);
    }

    #[test]
    fn ppm_tolerance_set_get_value(){
        let mut ppm = PpmTolerance::new();
        assert_eq!(*ppm.get_value(), None);
        ppm.set_value(Some(123.3));
        assert_eq!(*ppm.get_value(), Some(123.3));
    }

    #[test]
    fn absolute_tolerance_initialization(){
        let ppm = AbsoluteTolerance::new();
        assert_eq!(*ppm.get_value(), None);
    }

    #[test]
    fn absolute_tolerance_set_get_value(){
        let mut ppm = AbsoluteTolerance::new();
        assert_eq!(*ppm.get_value(), None);
        ppm.set_value(Some(123.3));
        assert_eq!(*ppm.get_value(), Some(123.3));
    }

}
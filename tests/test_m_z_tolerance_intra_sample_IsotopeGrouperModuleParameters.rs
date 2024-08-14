use mzbatch_generator::isotope_grouper_module::*;

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn m_z_tolerance_intra_sample_initialization(){
        let mz_obj = MzToleranceIntraSample::new();
        assert_eq!(mz_obj.get_name(), "m/z tolerance (intra-sample)");
        assert_eq!(mz_obj.get_parameter_length(), 0);
    }

    #[test]
    fn m_z_tolerance_intra_sample_add_get_parameter(){
        let mut mz_obj = MzToleranceIntraSample::new();
        assert_eq!(mz_obj.get_parameter_length(), 0);
        mz_obj.add_parameter(MzToleranceIntraSampleParameters::AbsoluteTolerance(AbsoluteTolerance::new()));
        assert_eq!(mz_obj.get_parameter_length(), 1);
        mz_obj.add_parameter(MzToleranceIntraSampleParameters::PpmTolerance(PpmTolerance::new()));
        assert_eq!(mz_obj.get_parameter_length(), 2);
    }

    #[test]
    fn ppm_tolerance_initialization(){
        let ppm = PpmTolerance::new();
        assert_eq!(*ppm.get_value(), None);
    }

    #[test]
    fn ppm_tolerance_get_set_value(){
        let mut ppm = PpmTolerance::new();
        assert_eq!(*ppm.get_value(), None);
        ppm.set_value(Some(1.1));
        assert_eq!(*ppm.get_value(), Some(1.1))
    }

    #[test]
    fn absolute_tolerance_initialization(){
        let absolute = AbsoluteTolerance::new();
        assert_eq!(*absolute.get_value(), None);
    }

    #[test]
    fn absolute_tolerance_get_set_value(){
        let mut absolute = AbsoluteTolerance::new();
        assert_eq!(*absolute.get_value(), None);
        absolute.set_value(Some(1.1));
        assert_eq!(*absolute.get_value(), Some(1.1))
    }

}
use mzbatch_generator::isotope_grouper_module_parameters::MzToleranceIntraSample;
use mzbatch_generator::isotope_grouper_module_parameters::MzToleranceIntraSampleParameters;
use mzbatch_generator::isotope_grouper_module_parameters::AbsoluteTolerance;
use mzbatch_generator::isotope_grouper_module_parameters::PpmTolerance;

use mzbatch_generator::xml_serialization::*;

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

    #[test]
    fn m_z_tolerance_intra_sample_serialization() -> IoResult<()>{

        // Create a writer with an in-memory buffer
        let mut writer = Writer::new(Cursor::new(Vec::new()));

        let mut mz_tolerance = MzToleranceIntraSample::new();
        let mut absolute = AbsoluteTolerance::new();
        absolute.set_value(Some(0.0015));
        let mut ppm = PpmTolerance::new();
        ppm.set_value(Some(3.0));

        mz_tolerance.add_parameter(MzToleranceIntraSampleParameters::AbsoluteTolerance(absolute));
        mz_tolerance.add_parameter(MzToleranceIntraSampleParameters::PpmTolerance(ppm));

        assert_eq!(mz_tolerance.get_parameter_length(), 2);

        // Write the ScanTypes element
        mz_tolerance.write_element(&mut writer)?;

        // Convert buffer to string
        let result = writer.into_inner().into_inner();
        let result_str = String::from_utf8(result).expect("Failed to convert result to string");

        // Define the expected XML output
        let expected = r#"<parameter name="m/z tolerance (intra-sample)"><absolutetolerance>0.0015</absolutetolerance><ppmtolerance>3.0</ppmtolerance></parameter>"#;

        // Assert the result matches the expected output
        assert_eq!(result_str, expected);

        Ok(())
    }

}
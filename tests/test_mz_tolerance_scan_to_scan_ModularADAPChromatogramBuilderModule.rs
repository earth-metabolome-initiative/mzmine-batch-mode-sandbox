use mzbatch_generator::modular_adap_chromatogram_builder_module_parameters::MzToleranceScanToScan;
use mzbatch_generator::modular_adap_chromatogram_builder_module_parameters::PpmTolerance;
use mzbatch_generator::modular_adap_chromatogram_builder_module_parameters::AbsoluteTolerance;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mz_initialization(){
        let mz_obj = MzToleranceScanToScan::new();
        assert_eq!(mz_obj.get_name(), "m/z tolerance (scan-to-scan)");
        assert_eq!(*mz_obj.get_absolute_tolerance(), None);
        assert_eq!(*mz_obj.get_ppm_tolerance(), None)
    }

    #[test]
    fn test_mz_abs_tol_set_get_value(){
        let mut mz_obj = MzToleranceScanToScan::new();
        mz_obj.set_absolute_tolerance(Some(34.5));
        assert_eq!(*mz_obj.get_absolute_tolerance(), Some(34.5));
    }

    #[test]
    fn test_mz_ppm_tol_set_get_value(){
        let mut mz_obj = PpmTolerance::new();
        mz_obj.set_value(Some(25.5));
        assert_eq!(*mz_obj.get_value(), Some(25.5));
    }

    #[test]
    fn test_mz_abs_tol_set_value(){
        let mut mz_obj = AbsoluteTolerance::new();
        mz_obj.set_value(Some(12.6));
        assert_eq!(*mz_obj.get_value(), Some(12.6));
    }
}
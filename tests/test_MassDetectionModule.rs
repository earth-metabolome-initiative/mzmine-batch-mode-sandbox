use mzbatch_generator::mass_detection_module::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mass_detection_module_initialization(){
        let mass_detection_obj = MassDetectionModule::new();
        assert_eq!(mass_detection_obj.get_method(), "io.github.mzmine.modules.dataprocessing.featdet_massdetection.MassDetectionModule", "NOT right method name");
        assert_eq!(mass_detection_obj.get_parameter_version(), 1, "NOT parameter version 1");
        assert_eq!(mass_detection_obj.get_parameters_length(), 0, "NOT empty parameter vector initialization");
    }

    #[test]
    fn test_mass_detection_module_add_parameter(){
        let mut mass_detection_module_obj = MassDetectionModule::new();
        assert_eq!(mass_detection_module_obj.get_parameters_length(), 0, "NOT empty parameter vector initalization");
        mass_detection_module_obj.add_parameter(MassDetectionModuleParameter::MassDetector(MassDetector::new()));
        assert_eq!(mass_detection_module_obj.get_parameter_version(), 1, "NOT correct lenght of parameter vector after adding parameter");
    }
}
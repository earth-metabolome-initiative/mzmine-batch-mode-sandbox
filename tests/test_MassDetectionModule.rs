use mzbatch_generator::modules::MassDetectionModule;
use mzbatch_generator::modules::MassDetectionModuleParameter;
use mzbatch_generator::mass_detection_module_parameters::MassDetector;

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
        assert_eq!(mass_detection_module_obj.get_parameter_version(), 1, "NOT correct length of parameter vector after adding parameter");
    }

    #[test]
    fn mass_detection_module_get_parameter(){
        let mut mass_detection_module_obj = MassDetectionModule::new();
        assert_eq!(mass_detection_module_obj.get_parameters_length(), 0, "NOT empty parameter vector initalization");
        mass_detection_module_obj.add_parameter(MassDetectionModuleParameter::MassDetector(MassDetector::new()));
        let retrieved_parameter = mass_detection_module_obj.get_parameter("MassDetector");
        assert_eq!(retrieved_parameter.get_name().get_str_value(), "Mass detector");
    }
}
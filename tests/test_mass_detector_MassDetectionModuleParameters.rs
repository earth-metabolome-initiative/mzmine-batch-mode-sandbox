use mzbatch_generator::mass_detection_module::*;
use mzbatch_generator::all_spectral_data_import_module_parameters::Auto;
use mzbatch_generator::all_spectral_data_import_module_parameters::MSDetectorAdvancedModules;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mass_detector_initialization(){
        let mass_detector_obj = MassDetector::new();
        assert_eq!(mass_detector_obj.get_name(), "Mass detector");
        assert_eq!(mass_detector_obj.get_selected_item(), "Factor of lowest signal");
        assert_eq!(mass_detector_obj.get_modules_length(), 0);
    }

    #[test]
    fn test_mass_detector_add_get_module() {
        let mut mass_detector_obj = MassDetector::new();
        assert_eq!(mass_detector_obj.get_modules_length(), 0);
    
        let auto = Auto::new();
        mass_detector_obj.add_module(MSDetectorAdvancedModules::Auto(auto));
        assert_eq!(mass_detector_obj.get_modules_length(), 1);
    
        // Check if the added module is of type Auto
        if let Some(module) = mass_detector_obj.get_module("Auto") {
            if let MSDetectorAdvancedModules::Auto(_) = module {
                // Module is of type Auto, test passes
            } else {
                panic!("Module is not of type Auto");
            }
        } else {
            panic!("Module not found");
        }
    }
    

}
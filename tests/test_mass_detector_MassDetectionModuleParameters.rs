use mzbatch_generator::mass_detection_module_parameters::*;
use mzbatch_generator::all_spectral_data_import_module_parameters::Auto;
use mzbatch_generator::all_spectral_data_import_module_parameters::Centroid;
use mzbatch_generator::all_spectral_data_import_module_parameters::MSDetectorAdvancedModules;

use mzbatch_generator::prelude::*;

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

    // #[test]
    // fn mass_detector_serialization() -> IoResult<()> {

    //     // <module name="Auto">
    //     //     <parameter name="Noise level">1000.0</parameter>
    //     // </module>
    //     // <module name="Centroid">
    //     //     <parameter name="Noise level"/>
    //     // </module>
        
    //     // Create a writer with an in-memory buffer
    //     let mut writer = Writer::new(Cursor::new(Vec::new()));

    //     let mut scan_types = MassDetector::new();

    //     scan_types.add_module(MSDetectorAdvancedModules::Auto(Auto::new()));
    //     scan_types.add_module(MSDetectorAdvancedModules::Centroid(Centroid::new()));

    //     // Write the ScanTypes element
        

    //     // Convert buffer to string
    //     let result = writer.into_inner().into_inner();
    //     let result_str = String::from_utf8(result).expect("Failed to convert result to string");

    //     // Define the expected XML output
    //     let expected = r#"<parameter name="Scan number"></parameter>"#;

    //     // Assert the result matches the expected output
    //     assert_eq!(result_str, expected);

    //     Ok(())
    // }
    

}
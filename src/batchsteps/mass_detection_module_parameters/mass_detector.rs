use serde::{Serialize, Deserialize};

use crate::prelude::MSDetectorAdvancedModules;
// I don't know why but prelude::* or any other use crate:: wasn't working ... 
use crate::batchsteps::all_spectral_data_import_module_parameters::advanced_import::Auto;
use crate::batchsteps::all_spectral_data_import_module_parameters::advanced_import::Centroid;
use crate::batchsteps::all_spectral_data_import_module_parameters::advanced_import::ExactMass;
use crate::batchsteps::all_spectral_data_import_module_parameters::advanced_import::FactorOfLowestSignal;
use crate::batchsteps::all_spectral_data_import_module_parameters::advanced_import::LocalMaxima;
use crate::batchsteps::all_spectral_data_import_module_parameters::advanced_import::RecursiveThreshold;
use crate::batchsteps::all_spectral_data_import_module_parameters::advanced_import::WaveletTransform;

#[derive(Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
pub struct MassDetector {
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "@selected")]
    selected_item: String,
    
    modules: Vec<MSDetectorAdvancedModules>,
}

impl MassDetector {
    pub fn new() -> Self{
        MassDetector { 
            name: "Mass detector".to_owned(), 
            selected_item: "Factor of lowest signal".to_owned(),
            modules: Vec::new()
        }
    }

    pub fn add_module(&mut self, parameter:MSDetectorAdvancedModules){
        self.modules.push(parameter);
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mass_detector_initialization(){
        let mass_detector_obj = MassDetector::new();
        assert_eq!(mass_detector_obj.name, "Mass detector");
        assert_eq!(mass_detector_obj.selected_item, "Factor of lowest signal");
        assert_eq!(mass_detector_obj.modules.len(), 0);
    }

    #[test]
    fn test_mass_detector_add_module(){
        let mut mass_detector_obj = MassDetector::new();
        assert_eq!(mass_detector_obj.modules.len(), 0);
        let auto = Auto::new();
        mass_detector_obj.add_module(MSDetectorAdvancedModules::Auto(auto));
        assert_eq!(mass_detector_obj.modules.len(), 1);
        if let MSDetectorAdvancedModules::Auto(_) = &mass_detector_obj.modules[0] {
            ()
        } else {
            panic!("Incorrect module type");
        }
    }

}
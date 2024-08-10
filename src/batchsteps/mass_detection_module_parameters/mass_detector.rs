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

#[derive(Default, Serialize, Deserialize, PartialEq, Debug, Clone)]
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

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn get_selected_item(&self) -> &str{
        &self.selected_item
    }

    pub fn add_module(&mut self, parameter:MSDetectorAdvancedModules){
        self.modules.push(parameter);
    }

    pub fn get_modules_length(&self) -> usize{
        self.modules.len()
    }

    pub fn get_module(&self, target: &str) -> Option<&MSDetectorAdvancedModules> {
        for module in &self.modules {
            match module {
                MSDetectorAdvancedModules::Auto(_f) if target == "Auto" => return Some(module),
                MSDetectorAdvancedModules::Centroid(_f) if target == "Centroid" => return Some(module),
                MSDetectorAdvancedModules::ExactMass(_f) if target == "Exact Mass" => return Some(module),
                MSDetectorAdvancedModules::FactorOfLowestSignal(_f) if target == "Factor of lowest signal" => return Some(module),
                MSDetectorAdvancedModules::LocalMaxima(_f) if target == "Local maxima" => return Some(module),
                MSDetectorAdvancedModules::RecursiveThreshold(_f) if target == "Recursive threshold" => return Some(module),
                MSDetectorAdvancedModules::WaveletTransform(_f) if target == "Wavelet transform" => return Some(module),
                _ => continue,
            }
        }
        None
    }
    
}
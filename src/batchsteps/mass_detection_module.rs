use serde::{Serialize, Deserialize};
use crate::prelude::*;

#[derive(Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
pub struct MassDetectionModule {
    #[serde(rename = "@method")]
    method: String,

    #[serde(rename = "@parameter_version")]
    parameter_version: u8,

    parameters: Vec<Parameter>,
}

impl MassDetectionModule{
    pub fn new() -> Self{
        MassDetectionModule{
            method: "io.github.mzmine.modules.dataprocessing.featdet_massdetection.MassDetectionModule".to_owned(),
            parameter_version: 1,
            parameters: Vec::new(),
        }
    }

    pub fn add_parameter(&mut self, parameter: Parameter){
        self.parameters.push(parameter);
    }
}

#[derive(Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
enum Parameter{
    RawDataFiles(RawDataFiles),
    ScanFilters(ScanFilters),
    ScanTypes(ScanTypes),
    MassDetector(MassDetector),
    DenormalizeFragmentScanTraps(DenormalizeFragmentScanTraps)
}


#[cfg(test)]
mod tests {
    use crate::batchsteps::mass_detection_module_parameters;

    use super::*;

    #[test]
    fn test_mass_detection_module_initialization(){
        let mass_detection_obj = MassDetectionModule::new();
        assert_eq!(mass_detection_obj.method, "io.github.mzmine.modules.dataprocessing.featdet_massdetection.MassDetectionModule", "NOT right method name");
        assert_eq!(mass_detection_obj.parameter_version, 1, "NOT parameter version 1");
        assert_eq!(mass_detection_obj.parameters.len(), 0, "NOT empty parameter vector initialization");
    }

    #[test]
    fn test_mass_detection_module_add_parameter(){
        let mut mass_detection_module_obj = MassDetectionModule::new();
        assert_eq!(mass_detection_module_obj.parameters.len(), 0, "NOT empty parameter vector initalization");
        mass_detection_module_obj.add_parameter(Parameter::MassDetector(MassDetector::new()));
        assert_eq!(mass_detection_module_obj.parameters.len(), 1, "NOT correct lenght of parameter vector after adding parameter");
    }
}
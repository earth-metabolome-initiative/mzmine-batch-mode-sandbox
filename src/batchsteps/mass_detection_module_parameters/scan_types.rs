use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
pub struct ScanTypes {
    #[serde(rename = "@name")]
    name: String,
    
    parameters: Vec<ScanTypesParameter>,
}

impl ScanTypes {
    pub fn new() -> Self{
        ScanTypes { 
            name: "Scan types (IMS)".to_owned(), 
            parameters: Vec::new()
        }
    }

    pub fn add_parameter(&mut self, parameter:ScanTypesParameter){
        self.parameters.push(parameter);
    }
}

#[derive(Default, Serialize, Deserialize, PartialEq, Debug)]
#[serde(default, rename = "file", rename_all = "lowercase")]
struct ScanTypesParameter{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "$text")]
    parameter: String
}

impl ScanTypesParameter{
    fn new() -> Self{
        ScanTypesParameter{
            name: "parameter".to_owned(),
            parameter: "All scan types".to_owned()
        }
    }

    fn get_parameter(&self) -> String{
        self.parameter.clone()
    }

    fn set_parameter(&mut self, parameter:String){
        self.parameter = parameter;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    //Still need to test serialization

    #[test]
    fn test_scan_types_initialization(){
        let scan_type_obj = ScanTypes::new();
        assert_eq!(scan_type_obj.name, "Scan types (IMS)");
        assert_eq!(scan_type_obj.parameters.len(), 0);
    }

    #[test]
    fn test_scan_types_add_parameter(){
        let mut scan_type_obj = ScanTypes::new();
        assert_eq!(scan_type_obj.parameters.len(), 0);
        scan_type_obj.add_parameter(ScanTypesParameter::new());
        assert_eq!(scan_type_obj.parameters.len(), 1);
    }

    #[test]
    fn test_scan_types_parameter_initialization(){
        let scan_types_parameter = ScanTypesParameter::new();
        assert_eq!(scan_types_parameter.name, "parameter");
        assert_eq!(scan_types_parameter.parameter, "All scan types");
    }

    #[test]
    fn test_scan_types_parameter_get_value(){
        let mut scan_types_parameter = ScanTypesParameter::new();
        scan_types_parameter.parameter = "New parameter".to_owned();
        assert_eq!(scan_types_parameter.get_parameter(), "New parameter");
    }

    #[test]
    fn test_scan_types_parameter_set_parameter(){
        let mut scan_types_parameter = ScanTypesParameter::new();
        scan_types_parameter.set_parameter("New Parameter".to_owned());
        assert_eq!(scan_types_parameter.parameter, "New Parameter");
    }

}
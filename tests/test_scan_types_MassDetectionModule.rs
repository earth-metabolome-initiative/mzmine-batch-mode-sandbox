use mzbatch_generator::mass_detection_module::*;

#[cfg(test)]
mod tests {
    use super::*;

    //Still need to test serialization

    #[test]
    fn test_scan_types_initialization(){
        let scan_type_obj = ScanTypes::new();
        assert_eq!(scan_type_obj.get_name(), "Scan types (IMS)");
        assert_eq!(scan_type_obj.get_parameters_length(), 0);
    }

    #[test]
    fn test_scan_types_add_parameter(){
        let mut scan_type_obj = ScanTypes::new();
        assert_eq!(scan_type_obj.get_parameters_length(), 0);
        scan_type_obj.add_parameter(ScanTypesParameter::new());
        assert_eq!(scan_type_obj.get_parameters_length(), 1);
    }

    #[test]
    fn test_scan_types_parameter_initialization(){
        let scan_types_parameter = ScanTypesParameter::new();
        assert_eq!(scan_types_parameter.get_name(), "parameter");
        assert_eq!(scan_types_parameter.get_parameter(), "All scan types");
    }

    #[test]
    fn test_scan_types_parameter_set_get_value(){
        let mut scan_types_parameter = ScanTypesParameter::new();
        scan_types_parameter.set_parameter("New parameter");
        assert_eq!(scan_types_parameter.get_parameter(), "New parameter");
    }
}
use mzbatch_generator::mass_detection_module_parameters::ScanTypes;

#[cfg(test)]
mod tests {
    use super::*;
    use quick_xml::writer::Writer;
    use std::io::{Cursor, Result as IoResult};

    //Still need to test serialization

    #[test]
    fn test_scan_types_initialization(){
        let scan_type_obj = ScanTypes::new();
        assert_eq!(scan_type_obj.get_name(), "Scan types (IMS)");
        assert_eq!(scan_type_obj.get_value(), "");
    }

    #[test]
    fn test_scan_types_add_parameter(){
        let mut scan_type_obj = ScanTypes::new();
        assert_eq!(scan_type_obj.get_value(), "");
        scan_type_obj.set_value("value");
        assert_eq!(scan_type_obj.get_value(), "value");
    }

    #[test]
    fn scan_types_serialization() -> IoResult<()> {
        // Create a writer with an in-memory buffer
        let mut writer = Writer::new(Cursor::new(Vec::new()));

        let mut scan_types = ScanTypes::new();
        scan_types.set_value("All scan types");

        // Write the ScanTypes element
        scan_types.write_element(&mut writer)?;

        // Convert buffer to string
        let result = writer.into_inner().into_inner();
        let result_str = String::from_utf8(result).expect("Failed to convert result to string");

        // Define the expected XML output
        let expected = r#"<parameter name="Scan types (IMS)">All scan types</parameter>"#;

        // Assert the result matches the expected output
        assert_eq!(result_str, expected);

        Ok(())
    }
}
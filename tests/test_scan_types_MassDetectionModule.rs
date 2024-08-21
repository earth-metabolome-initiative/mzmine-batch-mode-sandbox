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
    fn scan_types_serialization() -> Result<(), Box<dyn std::error::Error>> {
        let mut buffer = "".to_owned();

        let mut scan_types = ScanTypes::new();
        scan_types.set_value("All scan types");

        let expected = r#"<parameter name="Scan types (IMS)">All scan types</parameter>"#;

        quick_xml::se::to_writer(&mut buffer, &scan_types)?;
        assert_eq!(buffer, expected);

        Ok(())
    }
}
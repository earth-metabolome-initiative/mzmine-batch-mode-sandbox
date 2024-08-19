use mzbatch_generator::mass_detection_module_parameters::RawDataFiles;

#[cfg(test)]
mod tests {
    use super::*;

    use quick_xml::writer::Writer;
    use std::{io::{Cursor, Result as IoResult}, iter::Scan};

    #[test]
    fn raw_data_initialization(){
        let raw_data_obj = RawDataFiles::new();
        assert_eq!(raw_data_obj.get_name(), "Raw data files".to_owned(), "NOT correct object name");
        assert_eq!(raw_data_obj.get_type(), "BATCH_LAST_FILES".to_owned(), "NOT correct type name");
    }


    #[test]
    fn raw_data_set_type(){
        let mut raw_data_obj = RawDataFiles::new();
        assert_eq!(raw_data_obj.get_type(), "BATCH_LAST_FILES".to_owned(), "NOT correct type name");
        raw_data_obj.set_type("New type".to_owned());
        assert_eq!(raw_data_obj.get_type(), "New type", "Type not changed correctly");
    }   

    #[test]
    fn raw_data_serialization() -> IoResult<()> {
        // Create a writer with an in-memory buffer
        let mut writer = Writer::new(Cursor::new(Vec::new()));

        let mut scan_types = RawDataFiles::new();

        // Write the ScanTypes element
        scan_types.write_element(&mut writer)?;

        // Convert buffer to string
        let result = writer.into_inner().into_inner();
        let result_str = String::from_utf8(result).expect("Failed to convert result to string");

        // Define the expected XML output
        let expected = r#"<parameter name="Raw data files" type="BATCH_LAST_FILES"></parameter>"#;

        // Assert the result matches the expected output
        assert_eq!(result_str, expected);

        Ok(())
    }
}
use mzbatch_generator::mass_detection_module_parameters::RawDataFiles;

#[cfg(test)]
mod tests {
    use super::*;

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
    fn raw_data_serialization() -> Result<(), Box<dyn std::error::Error>> {
        let mut buffer = "".to_owned();

        let raw_data = RawDataFiles::new();

        quick_xml::se::to_writer(&mut buffer, &raw_data)?;

        let expected = r#"<parameter name="Raw data files" type="BATCH_LAST_FILES"/>"#;

        assert_eq!(buffer, expected);

        Ok(())
    }
}
use mzbatch_generator::all_spectral_data_import_module_parameters::*;
use mzbatch_generator::xml_serialization::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_names_new() {
        let file_names = FileNames::new();
        assert_eq!(file_names.get_name(), "File names");
        assert_eq!(file_names.files_length(), 0);
    }

    #[test]
    fn test_add_remove_file_to_file_names() {
        let mut file_names = FileNames::new();
        assert_eq!(file_names.files_length(), 0);                      // test it was empty
    
        let mut new_file = InputFile::new();
        new_file.set_name("This");
        
        file_names.add_file_name(new_file);
        
        assert_eq!(file_names.files_length(), 1);                      // test it's been added
    
        let retrieved_file = file_names.get_file("This");
        assert!(retrieved_file.is_some());                              // test it was found
        assert_eq!(retrieved_file.unwrap().get_name(), "This");        // test it is the correct InputFile
    }

    #[test]
    fn input_file_initialization(){
        let input_file = InputFile::new();
        assert_eq!(input_file.get_name(), "")
    }

    #[test]
    fn input_file_get_set_name(){
        let mut input_file = InputFile::new();
        assert_eq!(input_file.get_name(), "");
        input_file.set_name("Changed");
        assert_eq!(input_file.get_name(), "Changed");
    }

    #[test]
    fn file_names_serialization() -> IoResult<()>{
        // Create a writer with an in-memory buffer
        let mut writer = Writer::new(Cursor::new(Vec::new()));

        let mut file_names: FileNames = FileNames::new();

        let mut file1 = InputFile::new();
        file1.set_name("file1");
        file_names.add_file_name(file1);

        let mut file2 = InputFile::new();
        file2.set_name("file2");
        file_names.add_file_name(file2);

        let mut file3 = InputFile::new();
        file3.set_name("file3");
        file_names.add_file_name(file3);

        // Write the ScanTypes element
        file_names.write_element(&mut writer)?;

        // Convert buffer to string
        let result = writer.into_inner().into_inner();
        let result_str = String::from_utf8(result).expect("Failed to convert result to string");

        // Define the expected XML output
        let expected = r#"<parameter name="File names"><file>file1</file><file>file2</file><file>file3</file></parameter>"#;

        // Assert the result matches the expected output
        assert_eq!(result_str, expected);

        Ok(())
    }
}
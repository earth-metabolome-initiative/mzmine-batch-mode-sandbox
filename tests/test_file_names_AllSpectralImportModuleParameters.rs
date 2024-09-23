use mzbatch_generator::all_spectral_data_import_module_parameters::*;

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
    
        let mut new_file = InputFile::default();
        new_file.set_name("This");
        
        file_names.add_file_name(new_file);
        
        assert_eq!(file_names.files_length(), 1);                      // test it's been added
    
        let retrieved_file = file_names.get_file("This");
        assert!(retrieved_file.is_some());                              // test it was found
        assert_eq!(retrieved_file.unwrap().get_name(), "This");        // test it is the correct InputFile
    }

    #[test]
    fn file_names_serialization() ->  Result<(), Box<dyn std::error::Error>>{
        let mut file_names = FileNames::new();

        let mut file_1 = InputFile::default();
        file_1.set_name("file1");

        let mut file_2 = InputFile::default();
        file_2.set_name("file2");

        file_names.add_file_name(file_1);
        file_names.add_file_name(file_2);
    
        let mut buffer = "".to_owned(); // Create the string buffer for the XML content
        quick_xml::se::to_writer(&mut buffer, &file_names)?;
        
        let expected = r#"<parameter name="File names"><file>file1</file><file>file2</file></parameter>"#;
        
        assert_eq!(buffer, expected);
        
        Ok(())
    }

    #[test]
    fn file_names_generate_serialization() ->  Result<(), Box<dyn std::error::Error>>{
        let file_names = FileNames::generate(vec!["file1".to_string(), "file2".to_string()]);
    
        let mut buffer = "".to_owned(); // Create the string buffer for the XML content
        quick_xml::se::to_writer(&mut buffer, &file_names)?;
        
        let expected = r#"<parameter name="File names"><file>file1</file><file>file2</file></parameter>"#;
        
        assert_eq!(buffer, expected);
        
        Ok(())
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
    fn input_serialization() ->  Result<(), Box<dyn std::error::Error>>{
        let mut input_file = InputFile::new();
        input_file.set_name("this_file");
    
        let mut buffer = "".to_owned(); // Create the string buffer for the XML content
        quick_xml::se::to_writer(&mut buffer, &input_file)?;
        
        // IMPORTANT
        // serializer print int if float is .0
        let expected = r#"<file>this_file</file>"#;
        
        assert_eq!(buffer, expected);
        
        Ok(())
    }
}
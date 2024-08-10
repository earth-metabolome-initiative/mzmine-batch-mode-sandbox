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
    
        let mut new_file = InputFile::new();
        new_file.set_name("This".to_owned());
        
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
        input_file.set_name("Changed".to_owned());
        assert_eq!(input_file.get_name(), "Changed");
    }
}
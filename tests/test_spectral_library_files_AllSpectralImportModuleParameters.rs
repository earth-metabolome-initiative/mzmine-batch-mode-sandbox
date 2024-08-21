use mzbatch_generator::all_spectral_data_import_module_parameters::*;
use mzbatch_generator::xml_serialization::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spectral_library_initialization(){
        let spectral_library_obj = SpectralLibrary::new();
        assert_eq!(spectral_library_obj.get_name(), "Spectral library files");
        assert_eq!(spectral_library_obj.get_files_length(), 0);
    }

    #[test]
    fn test_spectral_library_add_file(){
        let mut spectral_library_obj = SpectralLibrary::new();
        assert_eq!(spectral_library_obj.get_files_length(), 0);
        spectral_library_obj.add_file(SpectralLibraryFile::new());
        assert_eq!(spectral_library_obj.get_files_length(), 1);
    }

    #[test]
    fn test_spectral_library_file_initialization(){
        let spectral_library_file_obj = SpectralLibraryFile::new();
        assert_eq!(spectral_library_file_obj.get_name(), "Spectral library files");
        assert_eq!(spectral_library_file_obj.get_file_name(), "File name")
    }

    #[test]
    fn test_spectral_library_file_change_file_name(){
        let mut new_file = SpectralLibraryFile::new();
        assert_eq!(new_file.get_file_name(), "File name");
        new_file.change_file_name("This".to_owned());
        assert_eq!(new_file.get_file_name(), "This");
    }

    #[test]
    fn spectral_library_empty_serialization() -> Result<(), Box<dyn std::error::Error>>{

        //      TODO 
        // check what happens if we add spectral files

        let spectral = SpectralLibrary::new();

        let mut buffer = "".to_owned();

        quick_xml::se::to_writer(&mut buffer, &spectral)?;

        // Define the expected XML output
        let expected = r#"<parameter name="Spectral library files"/>"#;

        // Assert the result matches the expected output
        assert_eq!(buffer, expected);

        Ok(())
    }
}
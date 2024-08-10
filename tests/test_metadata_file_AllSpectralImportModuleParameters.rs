use mzbatch_generator::all_spectral_data_import_module_parameters::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_MetaData_initialization(){
        let mut metadata_obj = MetaData::new();
        assert_eq!(metadata_obj.get_name(), "Metadata file", "NOT correct name");
        assert_eq!(metadata_obj.is_selected(), true, "NOT selected=true");
        assert_eq!(metadata_obj.get_current_file(), MetaDataFile::new(), "NOT MetaDataFile type");
        assert_eq!(metadata_obj.last_files_length(), 0, "NOT empty vector");
    }

    #[test]
    fn test_MetaData_exchange_current_file(){
        let mut metadata_obj = MetaData::new();
        let mut new_exchange_file: MetaDataFile = MetaDataFile::new();
        new_exchange_file.set_name("New Name".to_owned());
        metadata_obj.set_current_file(new_exchange_file);
        assert_eq!(metadata_obj.get_current_file().get_name(), "New Name");
    }

    #[test]
    fn test_add_last_file_name(){
        let mut metadata_obj = MetaData::new();
        let mut new_metadata_file = MetaDataFile::new();
        assert_eq!(metadata_obj.last_files_length(), 0);
        new_metadata_file.set_name("Added File".to_owned());
        metadata_obj.add_last_file_name(new_metadata_file);
        assert_eq!(metadata_obj.last_files_length(), 1);
        assert!(metadata_obj.get_last_file("Added File").is_ok());
    }

    // #[test]
    // fn test_remove_last_file_name(){
    //     let mut metadata_obj = MetaData::new();
    //     let mut new_metadata_file = MetaDataFile::new();
    //     new_metadata_file.set_name("Added File".to_owned());
    //     metadata_obj.add_last_file_name(new_metadata_file);
    //     assert_eq!(metadata_obj.last_files[0].name, "Added File");
    //     assert_eq!(metadata_obj.last_files.len(), 1);
    //     metadata_obj.remove_last_file_name(0);
    //     assert_eq!(metadata_obj.last_files.len(), 0);
    // }

    // #[test]
    // fn test_MetaDataFile_creation(){
    //     let metadatafile_obj = MetaDataFile::new();
    //     assert_eq!(metadatafile_obj.name, "Metadata file name");
    // }

    // fn test_MetaDataFile_set_name(){
    //     let mut metadatafile_obj = MetaDataFile::new();
    //     assert_eq!(metadatafile_obj.name, "Metadata file name");
    //     metadatafile_obj.set_name("New Name".to_owned());
    //     assert_eq!(metadatafile_obj.name, "New Name");
    // }
}
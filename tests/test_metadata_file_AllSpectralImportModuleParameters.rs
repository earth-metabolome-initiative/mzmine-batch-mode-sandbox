use mzbatch_generator::all_spectral_data_import_module_parameters::MetaData;
use mzbatch_generator::all_spectral_data_import_module_parameters::MetaDataFile;

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn meta_data_initialization(){
        let metadata_obj = MetaData::new();
        assert_eq!(metadata_obj.get_name(), "Metadata file", "NOT correct name");
        assert_eq!(metadata_obj.is_selected(), true, "NOT selected=true");
        assert_eq!(*metadata_obj.get_current_file(), MetaDataFile::new(), "NOT MetaDataFile type");
        assert_eq!(metadata_obj.last_files_length(), 0, "NOT empty vector");
    }

    #[test]
    fn meta_data_exchange_current_file(){
        let mut metadata_obj = MetaData::new();
        let new_exchange_file: MetaDataFile = MetaDataFile::new();
        metadata_obj.set_current_file(new_exchange_file);
        assert_eq!(metadata_obj.get_current_file().get_name(), "");
    }

    #[test]
    fn add_last_file_name(){
        let mut metadata_obj = MetaData::new();
        let mut new_metadata_file = MetaDataFile::new();
        assert_eq!(metadata_obj.last_files_length(), 0);
        new_metadata_file.set_name("Added File");
        metadata_obj.add_last_file_name(new_metadata_file);
        assert_eq!(metadata_obj.last_files_length(), 1);
        assert_eq!(metadata_obj.get_last_file("Added File").get_name(), "Added File");
    }

    #[test]
    fn remove_last_file_name(){
        let mut metadata_obj = MetaData::new();
        let mut new_metadata_file = MetaDataFile::new();
        new_metadata_file.set_name("Added File");
        metadata_obj.add_last_file_name(new_metadata_file);
        assert_eq!(metadata_obj.get_last_files()[0].get_name(), "Added File");
        assert_eq!(metadata_obj.last_files_length(), 1);
        metadata_obj.remove_last_file_name(0);
        assert_eq!(metadata_obj.last_files_length(), 0);
    }

    #[test]
    fn meta_data_file_initialization(){
        let metadatafile_obj = MetaDataFile::new();
        assert_eq!(metadatafile_obj.get_name(), "");
    }
    
    #[test]
    fn meta_data_file_set_name(){
        let mut metadatafile_obj = MetaDataFile::new();
        metadatafile_obj.set_name("New Name");
        assert_eq!(metadatafile_obj.get_name(), "New Name");
    }

    #[test]
    fn metadata_serialization() -> Result<(), Box<dyn std::error::Error>>{
        let mut metadata = MetaData::new();
        metadata.select();

        let mut current = MetaDataFile::new();
        current.set_name("current");
        metadata.set_current_file(current);

        let mut file1 = MetaDataFile::new();
        file1.set_name("file1");

        let mut file2 = MetaDataFile::new();
        file2.set_name("file2");

        metadata.add_last_file_name(file1);
        metadata.add_last_file_name(file2);


        let mut buffer = "".to_owned();

        quick_xml::se::to_writer(&mut buffer, &metadata)?;

        let expected = r#"<parameter name="Metadata file" selected="true"><current_file>current</current_file><last_file>file1</last_file><last_file>file2</last_file></parameter>"#;

        assert_eq!(buffer, expected);
        Ok(())
    }

}
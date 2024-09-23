use mzbatch_generator::all_spectral_data_import_module_parameters::MetaData;
use mzbatch_generator::all_spectral_data_import_module_parameters::MetaDataFile;

#[cfg(test)]
mod tests {

    use std::vec;

    use super::*;

    #[test]
    fn meta_data_initialization(){
        let metadata_obj = MetaData::default();
        assert_eq!(metadata_obj.get_name(), "Metadata file", "NOT correct name");
        assert_eq!(*metadata_obj.is_selected(), Some(true), "NOT selected=true");
        assert_eq!(*metadata_obj.get_current_file(), MetaDataFile::default(), "NOT MetaDataFile type");
        assert_eq!(metadata_obj.last_files_length(), 0, "NOT empty vector");
    }

    #[test]
    fn meta_data_exchange_current_file(){
        let mut metadata_obj = MetaData::default();
        let new_exchange_file: MetaDataFile = MetaDataFile::default();
        metadata_obj.set_current_file(new_exchange_file);
        assert_eq!(metadata_obj.get_current_file().get_name(), "");
    }

    #[test]
    fn add_last_file_name(){
        let mut metadata_obj = MetaData::default();
        let mut new_metadata_file = MetaDataFile::new("Added File");
        assert_eq!(metadata_obj.last_files_length(), 0);
        metadata_obj.add_last_file_name(new_metadata_file);
        assert_eq!(metadata_obj.last_files_length(), 1);
        assert_eq!(metadata_obj.get_last_file("Added File").get_name(), "Added File");
    }

    #[test]
    fn remove_last_file_name(){
        let mut metadata_obj = MetaData::default();
        let mut new_metadata_file = MetaDataFile::default();
        new_metadata_file.set_name("Added File");
        metadata_obj.add_last_file_name(new_metadata_file);
        assert_eq!(metadata_obj.get_last_files()[0].get_name(), "Added File");
        assert_eq!(metadata_obj.last_files_length(), 1);
        metadata_obj.remove_last_file_name(0);
        assert_eq!(metadata_obj.last_files_length(), 0);
    }

    #[test]
    fn meta_data_file_initialization(){
        let metadatafile_obj = MetaDataFile::default();
        assert_eq!(metadatafile_obj.get_name(), "");
    }
    
    #[test]
    fn meta_data_file_set_name(){
        let mut metadatafile_obj = MetaDataFile::default();
        metadatafile_obj.set_name("New Name");
        assert_eq!(metadatafile_obj.get_name(), "New Name");
    }

    #[test]
    fn metadata_serialization() -> Result<(), Box<dyn std::error::Error>>{
        let mut metadata = MetaData::default();
        metadata.select();

        let mut current = MetaDataFile::default();
        current.set_name("current");
        metadata.set_current_file(current);

        let mut file1 = MetaDataFile::default();
        file1.set_name("file1");

        let mut file2 = MetaDataFile::default();
        file2.set_name("file2");

        metadata.add_last_file_name(file1);
        metadata.add_last_file_name(file2);


        let mut buffer = "".to_owned();

        quick_xml::se::to_writer(&mut buffer, &metadata)?;

        let expected = r#"<parameter name="Metadata file" selected="true"><current_file>current</current_file><last_file>file1</last_file><last_file>file2</last_file></parameter>"#;

        assert_eq!(buffer, expected);
        Ok(())
    }

    #[test]
    fn metadata_generate_serialization() -> Result<(), Box<dyn std::error::Error>>{
        let metadata = MetaData::generate(vec!["current".to_string(), "file1".to_string(), "file2".to_string()]);

        let mut buffer = "".to_owned();

        quick_xml::se::to_writer(&mut buffer, &metadata)?;

        let expected = r#"<parameter name="Metadata file" selected="true"><current_file>current</current_file><last_file>file1</last_file><last_file>file2</last_file></parameter>"#;

        assert_eq!(buffer, expected);
        Ok(())
    }

}
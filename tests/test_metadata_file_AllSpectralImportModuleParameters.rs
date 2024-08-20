use mzbatch_generator::all_spectral_data_import_module_parameters::MetaData;
use mzbatch_generator::all_spectral_data_import_module_parameters::MetaDataFile;
use mzbatch_generator::xml_serialization::*;

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
    fn metadata_serialization() -> IoResult<()>{
        // Create a writer with an in-memory buffer
        let mut writer = Writer::new(Cursor::new(Vec::new()));

        let mut metadata: MetaData = MetaData::new();

        let mut current_file = MetaDataFile::new();
        current_file.set_name("/home/jopitim/mzmine.tsv");

        metadata.set_current_file(current_file);

        let mut last_file1 = MetaDataFile::new();
        last_file1.set_name("/home/allardpm1.tsv");

        metadata.add_last_file_name(last_file1);

        let mut last_file2 = MetaDataFile::new();
        last_file2.set_name("/home/allardpm2.tsv");

        metadata.add_last_file_name(last_file2);

        let mut last_file3 = MetaDataFile::new();
        last_file3.set_name("/home/allardpm3.tsv");

        metadata.add_last_file_name(last_file3);

        // Write the ScanTypes element
        metadata.write_element(&mut writer)?;

        // Convert buffer to string
        let result = writer.into_inner().into_inner();
        let result_str = String::from_utf8(result).expect("Failed to convert result to string");

        // Define the expected XML output
        let expected = r#"<parameter name="Metadata file" selected="true"><current_file>/home/jopitim/mzmine.tsv</current_file><last_file>/home/allardpm1.tsv</last_file><last_file>/home/allardpm2.tsv</last_file><last_file>/home/allardpm3.tsv</last_file></parameter>"#;

        // Assert the result matches the expected output
        assert_eq!(result_str, expected);

        Ok(())
    }

}
use mzbatch_generator::mass_detection_module::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn raw_data_initialization(){
        let raw_data_obj = RawDataFiles::new();
        assert_eq!(raw_data_obj.get_name(), "Raw data files".to_owned(), "NOT correct object name");
        assert_eq!(raw_data_obj.get_type(), "BATCH_LAST_FILES".to_owned(), "NOT correct type name");
        assert_eq!(raw_data_obj.get_files_length(), 0, "NOT 0 length files vector initialization");
    }

    #[test]
    fn raw_data_add_file(){
        let mut raw_data_obj = RawDataFiles::new();
        assert_eq!(raw_data_obj.get_files_length(), 0, "NOT 0 length files vector initialization");
        raw_data_obj.add_file(RDFiles::new());
        assert_eq!(raw_data_obj.get_files_length(), 1, "NOT correct lenght after file added to vector files");
    }

    #[test]
    fn raw_data_set_type(){
        let mut raw_data_obj = RawDataFiles::new();
        assert_eq!(raw_data_obj.get_type(), "BATCH_LAST_FILES".to_owned(), "NOT correct type name");
        raw_data_obj.set_type("New type".to_owned());
        assert_eq!(raw_data_obj.get_type(), "New type", "Type not changed correctly");
    }   

    #[test]
    fn raw_data_get_rd_file(){
        let mut raw_data_obj = RawDataFiles::new();
        assert_eq!(raw_data_obj.get_files_length(), 0, "NOT 0 length files vector initialization");

        let mut rd_file = RDFiles::new();
        rd_file.set_file_name("This".to_owned());
        raw_data_obj.add_file(rd_file);


        let mut other_rd_file = RDFiles::new();
        other_rd_file.set_file_name("That".to_owned());
        raw_data_obj.add_file(other_rd_file);

        assert_eq!(raw_data_obj.get_files_length(), 2);

        let this_file = raw_data_obj.get_file("This").expect("File 'This' not found");
        let that_file = raw_data_obj.get_file("That").expect("File 'That' not found");
    
        assert_eq!(this_file.get_file_name(), "This");
        assert_eq!(that_file.get_file_name(), "That");

    }
}
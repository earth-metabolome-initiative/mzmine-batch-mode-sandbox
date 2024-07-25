use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
pub struct MetaData {
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "@selected")]
    selected: bool,

    #[serde(rename = "@current_file")]
    currentFile: MetaDataFile,
    
    last_files: Vec<MetaDataFile>,
}

impl MetaData {
    pub fn new() -> Self {
        MetaData {
            name: "Metadata file".to_owned(),
            selected: true,
            currentFile: MetaDataFile::new(),
            last_files: Vec::new(),
        }
    }

    pub fn set_name(&mut self, name:String){
        self.name = name;
    }

    pub fn get_name(&self) -> String{
        self.name.clone()
    }


    pub fn exchange_current_file(&mut self, file: MetaDataFile){
        self.currentFile = file;
    }

    pub fn add_last_file_name(&mut self, element: MetaDataFile){
        self.last_files.push(element);
    }

    pub fn remove_last_file_name(&mut self, index: usize){
        self.last_files.remove(index);
    }


}

#[derive(Default, Serialize, Deserialize, PartialEq, Debug)]
#[serde(default, rename = "last_file", rename_all = "lowercase")]
pub struct MetaDataFile {
    #[serde(rename = "$text")]
    name: String
}

impl MetaDataFile {
    pub fn new() -> Self {
        MetaDataFile {
            name: "Metadata file name".to_owned(),
        }
    }

    pub fn set_name(&mut self, file_name: String){
        self.name = file_name;
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_MetaData_creation(){
        let metadata_obj = MetaData::new();
        assert_eq!(metadata_obj.name, "Metadata file", "NOT correct name");
        assert_eq!(metadata_obj.selected, true, "NOT selected=true");
        assert_eq!(metadata_obj.currentFile, MetaDataFile::new(), "NOT MetaDataFile type");
        assert_eq!(metadata_obj.last_files.len(), 0, "NOT empty vector");
    }

    #[test]
    fn test_MetaData_exchange_current_file(){
        let mut metadata_obj = MetaData::new();
        let mut new_exchange_file: MetaDataFile = MetaDataFile::new();
        new_exchange_file.set_name("New Name".to_owned());
        metadata_obj.exchange_current_file(new_exchange_file);
        assert_eq!(metadata_obj.currentFile.name, "New Name");
    }

    #[test]
    fn test_add_last_file_name(){
        let mut metadata_obj = MetaData::new();
        let mut new_metadata_file = MetaDataFile::new();
        assert_eq!(metadata_obj.last_files.len(), 0);
        new_metadata_file.set_name("Added File".to_owned());
        metadata_obj.add_last_file_name(new_metadata_file);
        assert_eq!(metadata_obj.last_files.len(), 1);
        assert_eq!(metadata_obj.last_files[0].name, "Added File");
    }

    #[test]
    fn test_remove_last_file_name(){
        let mut metadata_obj = MetaData::new();
        let mut new_metadata_file = MetaDataFile::new();
        new_metadata_file.set_name("Added File".to_owned());
        metadata_obj.add_last_file_name(new_metadata_file);
        assert_eq!(metadata_obj.last_files[0].name, "Added File");
        assert_eq!(metadata_obj.last_files.len(), 1);
        metadata_obj.remove_last_file_name(0);
        assert_eq!(metadata_obj.last_files.len(), 0);
    }

    #[test]
    fn test_MetaDataFile_creation(){
        let metadatafile_obj = MetaDataFile::new();
        assert_eq!(metadatafile_obj.name, "Metadata file name");
    }

    fn test_MetaDataFile_set_name(){
        let mut metadatafile_obj = MetaDataFile::new();
        assert_eq!(metadatafile_obj.name, "Metadata file name");
        metadatafile_obj.set_name("New Name".to_owned());
        assert_eq!(metadatafile_obj.name, "New Name");
    }
}
use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
pub struct RawDataFiles {
    #[serde(rename = "@name")]
    name: String,

    //needs the _ because type is a reserved keyword
    #[serde(rename = "@type")]
    _type: String,
    
    files: Vec<RDFiles>,
}

impl RawDataFiles {
    pub fn new() -> Self{
        RawDataFiles { 
            name: "Raw data files".to_owned(), 
            _type: "BATCH_LAST_FILES".to_owned(),
            files: Vec::new()
        }
    }

    pub fn add_file(&mut self, file:RDFiles){
        self.files.push(file);
    }

    pub fn set_type(&mut self, _type: String){
        self._type = _type;
    }
}

//still not used but supposed to look something like this if will ever be the case
 #[derive(Default, Serialize, Deserialize, PartialEq, Debug)]
 #[serde(default, rename = "file", rename_all = "lowercase")]
 struct RDFiles{
     #[serde(rename = "@name")]
     name: String,
 
     #[serde(rename = "$text")]
     file_name: String
 }
 
 impl RDFiles{
     fn new() -> Self{
         RDFiles{
             name: "file".to_owned(),
             file_name: "file_name".to_owned()
         }
     }
 
     fn set_file_name(&mut self, name: String){
         self.file_name = name;
     }
 }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_raw_data_initialization(){
        let raw_data_obj = RawDataFiles::new();
        assert_eq!(raw_data_obj.name, "Raw data files".to_owned(), "NOT correct object name");
        assert_eq!(raw_data_obj._type, "BATCH_LAST_FILES".to_owned(), "NOT correct type name");
        assert_eq!(raw_data_obj.files.len(), 0, "NOT 0 length files vector initialization");
    }

    #[test]
    fn test_raw_data_add_file(){
        let mut raw_data_obj = RawDataFiles::new();
        assert_eq!(raw_data_obj.files.len(), 0, "NOT 0 length files vector initialization");
        raw_data_obj.add_file(RDFiles::new());
        assert_eq!(raw_data_obj.files.len(), 1, "NOT correct lenght after file added to vector files");
    }

    #[test]
    fn test_raw_data_set_type(){
        let mut raw_data_obj = RawDataFiles::new();
        assert_eq!(raw_data_obj._type, "BATCH_LAST_FILES".to_owned(), "NOT correct type name");
        raw_data_obj.set_type("New type".to_owned());
        assert_eq!(raw_data_obj._type, "New type", "Type not changed correctly");
    }   
}
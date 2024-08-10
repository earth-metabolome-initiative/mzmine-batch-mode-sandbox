use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize, PartialEq, Clone)]
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

    pub fn get_name(&self) -> String{
        self.name.clone()
    }

    pub fn get_type(&self) -> String{
        self._type.clone()
    }

    pub fn set_type(&mut self, _type: String){
        self._type = _type;
    }

    pub fn add_file(&mut self, file:RDFiles){
        self.files.push(file);
    }

    pub fn get_file(&self, target: &str) -> Result<&RDFiles, &'static str>{
        for file in &self.files{
            if file.get_file_name() == target{
                return Ok(file);
            }
        }
        Err("File not found")
    }   

    pub fn get_files_length(&self) -> usize{
        self.files.len()
    }
}

//still not used but supposed to look something like this if will ever be the case
 #[derive(Default, Serialize, Deserialize, PartialEq, Debug, Clone)]
 #[serde(default, rename = "file", rename_all = "lowercase")]
 pub struct RDFiles{
     #[serde(rename = "@name")]
     name: String,
 
     #[serde(rename = "$text")]
     file_name: String
 }
 
 impl RDFiles{
    pub fn new() -> Self{
         RDFiles{
             name: "file".to_owned(),
             file_name: "file_name".to_owned()
         }
     }

    pub fn get_name(&self) -> &str{
        &self.name
    }
 
    pub fn get_file_name(&self) -> &str{
        &self.file_name
    }

    pub fn set_file_name(&mut self, name: String){
         self.file_name = name;
    }
 }
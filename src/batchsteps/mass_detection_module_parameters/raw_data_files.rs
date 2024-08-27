use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq, Clone, Debug)]
#[serde(default, rename_all = "lowercase", rename = "parameter")]
pub struct RawDataFiles {
    #[serde(rename = "@name")]
    name: String,

    //needs the _ because type is a reserved keyword
    #[serde(rename = "@type")]
    _type: String,
}

impl Default for RawDataFiles{
    fn default() -> Self{
        RawDataFiles { 
            name: "Raw data files".to_owned(), 
            _type: "BATCH_LAST_FILES".to_owned(),
        }
    }
}

impl RawDataFiles {
    pub fn new() -> Self{
        RawDataFiles { 
            name: "Raw data files".to_owned(), 
            _type: "".to_owned(),
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn get_type(&self) -> &str{
        &self._type
    }

    pub fn set_type(&mut self, _type: String){
        self._type = _type;
    }
}
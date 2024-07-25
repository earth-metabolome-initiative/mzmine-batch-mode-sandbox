use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
pub struct FileNames {
    #[serde(rename = "@name")]
    name: String,
    
    files: Vec<InputFile>,
}

impl Default for FileNames {
    fn default() -> Self {
        FileNames {
            name: "File names".to_owned(),
            files: vec![InputFile::default(), InputFile::default()],
        }
    }
}

impl FileNames{
    pub fn new() -> Self {
        FileNames {
            name: "NoName".to_owned(),
            files: vec![],
        }
    }

    pub fn add_file_name(&mut self, element: InputFile){
        self.files.push(element);
    }

    pub fn remove_file_name(&mut self, index: usize){
        self.files.remove(index);
    }
}

#[derive(Serialize, Deserialize, PartialEq, Debug)]
#[serde(default, rename = "file", rename_all = "lowercase")]
pub struct InputFile {
    #[serde(rename = "$text")]
    name: String
}

impl Default for InputFile {
    fn default() -> Self {
        InputFile {
            name: "File_name".to_owned(),
        }
    }
}

impl InputFile{
    fn set_name(&mut self, file_name: String){
        self.name = file_name;
    }
}




#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_file_names_new() {
        let file_names = FileNames::new();
        assert_eq!(file_names.name, "NoName");
        assert_eq!(file_names.files.len(), 0);
    }

    #[test]
    fn test_add_file_to_file_names(){
        let mut file_names = FileNames::new();
        assert_eq!(file_names.files.len(), 0);                      //test it was empty
        file_names.add_file_name(InputFile::default());
        assert_eq!(file_names.files.len(), 1);                      //test it's been added
        assert_eq!(file_names.files[0], InputFile::default());      //test it is an InputFile
    }

    #[test]
    fn test_remove_file(){
        let mut file_names = FileNames::new();
        file_names.add_file_name(InputFile::default());
        assert_eq!(file_names.files.len(), 1);
        file_names.remove_file_name(0);
        assert_eq!(file_names.files.len(), 0);
    }

    #[test]
    fn test_input_file_default_name(){
        let input_file = InputFile::default();
        assert_eq!(input_file.name, "File_name")
    }

    #[test]
    fn test_input_file_set_name(){
        let mut input_file = InputFile::default();
        input_file.set_name("Changed".to_owned());
        assert_eq!(input_file.name, "Changed");
    }
}
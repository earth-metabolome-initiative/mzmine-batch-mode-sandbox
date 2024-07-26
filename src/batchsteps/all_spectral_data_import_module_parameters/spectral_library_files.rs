use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
pub struct SpectralLibrary {
    #[serde(rename = "@name")]
    name: String,

    files: Vec<SpectralLibraryFile>
}

impl SpectralLibrary{
    pub fn new() -> Self {
        SpectralLibrary {
            name: "Spectral library files".to_owned(),
            files: Vec::new(),
        }
    }

    pub fn add_file(&mut self, element: SpectralLibraryFile){
        self.files.push(element);
    }

    pub fn remove_file_name(&mut self, index: usize){
        self.files.remove(index);
    }
}

#[derive(Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase", rename = "file")]
struct SpectralLibraryFile{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "$text")]
    file_name: String,
}

impl SpectralLibraryFile{
    pub fn new() -> Self{
        SpectralLibraryFile{
            name: "Spectral library files".to_owned(),
            file_name: "File name".to_owned(),
        }
    }

    pub fn change_file_name(&mut self, name: String){
        self.file_name = name;
    }
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spectral_library_initialization(){
        let spectral_library_obj = SpectralLibrary::new();
        assert_eq!(spectral_library_obj.name, "Spectral library files");
        assert_eq!(spectral_library_obj.files.len(), 0);
    }

    #[test]
    fn test_spectral_library_add_file(){
        let mut spectral_library_obj = SpectralLibrary::new();
        assert_eq!(spectral_library_obj.files.len(), 0);
        spectral_library_obj.add_file(SpectralLibraryFile::new());
        assert_eq!(spectral_library_obj.files.len(), 1);
    }

    #[test]
    fn test_spectral_library_file_initialization(){
        let spectral_library_file_obj = SpectralLibraryFile::new();
        assert_eq!(spectral_library_file_obj.name, "Spectral library files");
        assert_eq!(spectral_library_file_obj.file_name, "File name")
    }

    #[test]
    fn test_spectral_library_file_change_file_name(){
        let mut new_file = SpectralLibraryFile::new();
        assert_eq!(new_file.file_name, "File name");
        new_file.change_file_name("This".to_owned());
        assert_eq!(new_file.file_name, "This");
    }
}
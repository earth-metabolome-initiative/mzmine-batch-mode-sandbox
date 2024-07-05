use simple_xml_builder::XMLElement;


pub struct FileList{
    pub files: Vec<String>,
    pub number_of_files: usize,
}

impl FileList{
    pub fn new(files: Vec<String>) -> Self{
        Self {
            files: files.clone(),
            number_of_files: files.len(),
        }
    }

    pub fn gen_XML_element(&self) -> XMLElement{
        let mut fileNames = XMLElement::new("batchstep");
        fileNames.add_attribute("method", "io.github.mzmine.modules.io.import_rawdata_all.AllSpectralDataImportModule");
        fileNames.add_attribute("parameter_version=", "1");        

        for file in self.files.iter(){
            let mut parameter = XMLElement::new("file");
            parameter.add_text(file);
            fileNames.add_child(parameter);
        }
        
        fileNames
    }
}



// // vector of file names to be fed to the parameters file
// let mut input_files:Vec<String> = Vec::new();
// 
// for entry in fs::read_dir("/home/jopitim/mzmine-batch-mode-sandbox/data/mzml/")? {
//     let dir = entry?;
//     input_files.push(dir.path().into_os_string().into_string().unwrap());
// }

// let mut fileNames = XMLElement::new("batchstep");
// fileNames.add_attribute("method", "io.github.mzmine.modules.io.import_rawdata_all.AllSpectralDataImportModule");
// fileNames.add_attribute("parameter_version=", "1");
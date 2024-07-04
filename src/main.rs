use std::fs::File;
use std::io::Result;

use simple_xml_builder::XMLElement;

use std::fs;

// local batchsteps module definition
mod batchsteps;

mod io {
    pub mod export_features_gnps;
}

mod export_features_gnps {
    pub mod fbmn;
}

mod fbmn {
    pub mod GnpsFBmnExportAndSubmitModule;
}


struct Batchstep{
    //The XML file is constructed of multiple batchsteps, in which we find parameters and other attributes
    name: String,
    parameters: Vec<String>,
}

impl Batchstep{
    fn new(name: &str) -> Self{
        Self{
            name: String::from(name),
            parameters : Vec::new(),
        }
    }
}

struct XMLFileInput{
    // Define at beginning the mzmine version (eventually other important general infos), 
    // Than store in a vector *.mzML files provided for analysis,
    // Define at priori the batchsteps needed

    output_file_name: String,
    mzmine_version: String,
    input_raw_files: Vec<String>,
    batchsteps: Vec<Batchstep>,
}

impl XMLFileInput{
    //methods to fill XML file with input informations

    fn new(output_file_name: &str, mz_version: &str, input_raw_files: Vec<String>, batchsteps:Vec<Batchstep>) -> Self{
        Self{
            output_file_name: String::from(output_file_name),
            mzmine_version: String::from(mz_version),
            input_raw_files,
            batchsteps,
        }
    }

}

fn main() -> Result<()> {

    batchsteps::io::export_features_gnps::fbmn::GnpsFBmnExportAndSubmitModule::greet();

    //Define input_files path in a String vector

    // vector of file names to be fed to the parameters file
    let mut input_files:Vec<String> = Vec::new();

    for entry in fs::read_dir("/home/jopitim/mzmine-batch-mode-sandbox/data/mzml/")? {
        let dir = entry?;
        input_files.push(dir.path().into_os_string().into_string().unwrap());
    }

    // list all the batchsteps necessary
    let batchsteps_elements = vec![
        String::from("bachstep1"),
        String::from("batchstep2"),
    ];

    let mut batch_steps_vector: Vec<Batchstep> = Vec::new();
    for element in batchsteps_elements.iter() {
        batch_steps_vector.push(Batchstep::new(element));
    }

    let File_content = XMLFileInput::new("gen_parameters.mzbatch", "4.1.0", input_files, batch_steps_vector);

// - - - - - - - - - - - - Write XML - - - - - - - - - - - - - - - - - - - - -

    // Create XML file
    let mut file = File::create(File_content.output_file_name)?;

    // Create the batch (root) element
    let mut batch = XMLElement::new("batch");
    batch.add_attribute("mzmine_version", File_content.mzmine_version);

    let mut fileNames = XMLElement::new("batchstep");
    fileNames.add_attribute("method", "io.github.mzmine.modules.io.import_rawdata_all.AllSpectralDataImportModule");
    fileNames.add_attribute("parameter_version=", "1");

    let mut fileNamesParameter = XMLElement::new("parameter");
    fileNamesParameter.add_attribute("name", "File names");

    for file in File_content.input_raw_files.iter() {
        let mut tmp_file = XMLElement::new("file");
        tmp_file.add_text(file);
        fileNames.add_child(tmp_file);
    }


    // create child strucure

    fileNames.add_child(fileNamesParameter);

    batch.add_child(fileNames);

    for element in File_content.batchsteps.iter() {
        let batchstep = XMLElement::new(element.name.clone());
        batch.add_child(batchstep)
    }

    batch.write(&mut file)?;

    Ok(())

}
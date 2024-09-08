use crate::modules::*;
use serde::{Deserialize, Serialize}; // Import serde macros

/// Enum of all available modules (type defined)
#[derive(Serialize, Deserialize, PartialEq)]
#[serde(untagged)] // uses only content of the batchstep themselves and not serializes this enum
pub enum Batchstep {
    AllSpectralDataImportModule(AllSpectralDataImportModule),
    MassDetectionModule(Vec<MassDetectionModule>),
    ModularADAPChromatogramBuilderModule(ModularADAPChromatogramBuilderModule),
    SmoothinModule(SmoothingModule),
    MinimumSearchFeatureResolverModule(MinimumSearchFeatureResolverModule),
    IsotopeGrouper(IsotopeGrouper),
    RowsFilterModule(RowsFilterModule),
    IsotopeFinderModule(IsotopeFinderModule),
    GnpsFbmnExportAndSubmitModule(GnpsFbmnExportAndSubmitModule),
    SiriusExportModule(SiriusExportModule)
}

/// (Each?) batch contains multiple batchsteps, which are stored in an array of Modules
#[derive(Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename = "batch", rename_all = "lowercase")]
pub struct Batch {
    #[serde(rename = "@mzmine_version")]
    mzmine_version: String,
    batchsteps: Vec<Batchstep>,
}

impl Batch {
    pub fn new(mzmine_version: String, batchsteps: Vec<Batchstep>) -> Self {
        Batch {
            mzmine_version,
            batchsteps,
        }
    }

    pub fn get_mzmine_version(&self) -> &str{
        &self.mzmine_version
    }

    pub fn set_version(&mut self, mzmine_version: String) {
        self.mzmine_version = mzmine_version;
    }

    pub fn add_batchstep(&mut self, batchstep: Batchstep){
        self.batchsteps.push(batchstep)
    }

    pub fn get_batchsteps_length(&self) -> usize{
        self.batchsteps.len()
    }
}

// fn main() -> Result<(), Box<dyn std::error::Error>> {
// 
//     let header: String = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>".to_owned();
// 
//     let mut batch = Batch::default();       // creiamo Batch
//     batch.mzmine_version = "4.1.0".to_owned();      // filliamo Batch
// 
//     batch.batchstep = vec![
//         Modules::AllSpectralDataImportModule(AllSpectralDataImportModule::default())
//     ];  // filliamo con i nostri batchstep di test -> verrà fillato dal nostro enums di batchsteps
// 
//     let mut buffer = header;     // crea la stringa in cui caricare (?) il nostro contenuto XML
//     quick_xml::se::to_writer(&mut buffer, &batch)?;
//     
//     println!("{}", buffer);
// 
//     // Open a file in write mode
//     let mut file = File::create("parameters.mzbatch")?;
// 
//     // Write the header string to the file
//     file.write_all(buffer.as_bytes())?;
// 
//     Ok(())
// }
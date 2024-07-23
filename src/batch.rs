use std::fs::File;
use std::io::Write;

use crate::prelude::*;
use serde::{Deserialize, Serialize}; // Import serde macros

/// Enum of all available modules (type defined)
#[derive(Serialize, Deserialize, PartialEq)]
#[serde(untagged)] // uses only content of the batchstep themselves and not serializes this enum
pub enum Modules {
    AllSpectralDataImportModule(AllSpectralDataImportModule),
    // MassDetectionModule(Vec<batchsteps::mass_detection_module::MassDetectionModule>),
    // ModularADAPChromatogramBuilderModule(batchsteps::modular_adap_chromatogram_builder_module::ModularADAPChromatogramBuilderModule),
    // SmoothinModule(batchsteps::smoothing_module::SmoothingModule),
    // MinimumSearchFeatureResolverModule(batchsteps::minimum_search_feature_resolver_module::MinimumSearchFeatureResolverModule),
    // IsotopeGrouper(batchsteps::isotope_grouper_module::IsotopeGrouper),
    // RowsFilterModule(batchsteps::rows_filter_module::RowsFilterModule),
    // RowsFilterModule2(batchsteps::rows_filter_module_2::RowsFilterModule2),
    // GnpsFbmnExportAndSubmitModule(batchsteps::gnps_fbmn_export_and_submit_module::GnpsFbmnExportAndSubmitModule),
    // SiriusExportModule(batchsteps::sirius_export_module::SiriusExportModule)
}

/// (Each?) batch contains multiple batchsteps, which are stored in an array of Modules
#[derive(Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename = "batch", rename_all = "lowercase")]
pub struct Batch {
    #[serde(rename = "@mzmine_version")]
    mzmine_version: String,
    batchstep: Vec<Modules>,
}

impl Batch {
    fn set_name(&mut self, mzmine_version: String) {
        self.mzmine_version = mzmine_version;
    }

    fn add_batchstep(&mut self, module: Modules){
        self.batchstep.push(module)
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

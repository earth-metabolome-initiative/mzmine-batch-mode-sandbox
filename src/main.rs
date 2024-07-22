use batchsteps::all_spectral_data_import_module::AllSpectralDataImportModule;
use serde::{Deserialize, Serialize}; // Import serde macros

mod batchsteps;

/// Enum of all available modules (type defined)
#[derive(Serialize, Deserialize, PartialEq)]
#[serde(untagged)] // uses only content of the batchstep themselves and not serializes this enum
pub enum Modules {
    AllSpectralDataImportModule(AllSpectralDataImportModule),

    // A vector since we have at least two (idk if more) -> MS1, MS2?
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

/// Each(?) batch contains multiple batchsteps, which are stored in an array of Modules
#[derive(Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename = "batch", rename_all = "lowercase")]
struct Batch {

    #[serde(rename = "@mzmine_version")]    // attributo da inserire tipo parameters/method/ ...
    mzmine_version: String,

    //#[serde(rename = "@batchstep")]              // testo da inserire tipo path/to/file/ / ANY / 1000.0
    batchstep: Vec<Modules>,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut batch = Batch::default();       // creiamo Batch
    batch.mzmine_version = "4.1.0".to_owned();      // filliamo Batch

    batch.batchstep = vec![
        Modules::AllSpectralDataImportModule(AllSpectralDataImportModule::default()),
        Modules::AllSpectralDataImportModule(AllSpectralDataImportModule::default())
    ];  // filliamo con i nostri batchstep di test -> verrà fillato dal nostro enums di batchsteps

    let mut buffer = String::new();     // crea la stringa in cui caricare (?) il nostro contenuto XML
    quick_xml::se::to_writer(&mut buffer, &batch)?;
    
    println!("{}", buffer);

    Ok(())
}

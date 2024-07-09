use std::fs::File;
use quick_xml;
use serde::{Serialize, Deserialize};
use simple_xml_builder::XMLElement;

mod batchsteps;

const MZMINE_VERSION: &str = "4.1.0";

/// Enum of all available modules (type defined)
#[derive(Serialize, Deserialize)]
pub enum Modules{
    AllSpectralDataImportModule(batchsteps::all_spectral_data_import_module::AllSpectralDataImportModule),

    // A vector since we have at least two (idk if more) -> MS1, MS2?
    MassDetectionModule(Vec<batchsteps::mass_detection_module::MassDetectionModule>),

    ModularADAPChromatogramBuilderModule(batchsteps::modular_adap_chromatogram_builder_module::ModularADAPChromatogramBuilderModule),

    SmoothinModule(batchsteps::smoothing_module::SmoothingModule),

    MinimumSearchFeatureResolverModule(batchsteps::minimum_search_feature_resolver_module::MinimumSearchFeatureResolverModule),

    IsotopeGrouper(batchsteps::isotope_grouper_module::IsotopeGrouper),

    RowsFilterModule(batchsteps::rows_filter_module::RowsFilterModule),

    RowsFilterModule2(batchsteps::rows_filter_module_2::RowsFilterModule2),

    GnpsFbmnExportAndSubmitModule(batchsteps::gnps_fbmn_export_and_submit_module::GnpsFbmnExportAndSubmitModule),

    SiriusExportModule(batchsteps::sirius_export_module::SiriusExportModule)
}

/// Each(?) batch contains multiple batchsteps, which are stored in an array of Modules
#[derive(Default, Serialize, Deserialize)]
pub struct Batch{
    mzmine_version: &'static str,
    batchsteps: Vec<Modules>,
}

/// Create batch XML element which contains all batchsteps selected
fn batch_builder(batch: &mut Batch, element: Modules){
    batch.batchsteps.push(element);
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_add() {
    //     assert_eq!(add(2, 3), 5);
    // }
}

fn main() {
    println!("You called the main function :D")
}
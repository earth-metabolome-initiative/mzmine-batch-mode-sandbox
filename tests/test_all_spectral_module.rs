use mzbatch_generator::modules::AllSpectralDataImportModule;
use mzbatch_generator::all_spectral_data_import_module_parameters::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialization(){
        let all_spectral_data_import_module_obj = AllSpectralDataImportModule::new();
        assert_eq!(all_spectral_data_import_module_obj.method, "io.github.mzmine.modules.io.import_rawdata_all.AllSpectralDataImportModule");
        assert_eq!(all_spectral_data_import_module_obj.parameter_version, 1);
        assert_eq!(all_spectral_data_import_module_obj.parameters.len(), 0);
    }

    #[test]
    fn add_parameter(){
        let mut all_spectral_data_import_module_obj = AllSpectralDataImportModule::new();
        assert_eq!(all_spectral_data_import_module_obj.parameters.len(), 0);
        all_spectral_data_import_module_obj.add_parameter(Parameter::FileNames(FileNames::new()));
        assert_eq!(all_spectral_data_import_module_obj.parameters.len(), 1);       
    }
}
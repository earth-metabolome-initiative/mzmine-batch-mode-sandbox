use mzbatch_generator::all_spectral_data_import_module_parameters::*;

#[cfg(test)]
mod tests {
    use super::*;

    //Still need to test serialization

    #[test]
    fn advanced_import_initilization(){
        let advanced_obj = AdvancedImport::new();
        assert_eq!(advanced_obj.get_name(), "Advanced import");
        assert_eq!(advanced_obj.get_parameters_length(), 0);
        assert_eq!(advanced_obj.is_selected(), false);
    }

    #[test]
    fn advanced_import_add_parameter(){
        let mut advanced_obj = AdvancedImport::new();
        assert_eq!(advanced_obj.get_parameters_length(), 0);
        advanced_obj.add_parameter(AdvancedImportParameters::ScanFilter(ScanFilter::new()));
        assert_eq!(advanced_obj.get_parameters_length(), 1);
    }

    fn advanced_import_invert_selected(){
        let mut advanced_obj = AdvancedImport::new();
        assert_eq!(advanced_obj.is_selected(), false);
        advanced_obj.invert_selected();
        assert_eq!(advanced_obj.is_selected(), true);
    }

    
}
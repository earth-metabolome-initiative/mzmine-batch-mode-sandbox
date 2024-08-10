use mzbatch_generator::all_spectral_data_import_module_parameters::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn scan_filter_initialization(){
        let scan_filter_obj = ScanFilter::new();
        assert_eq!(scan_filter_obj.get_name(), "Scan filters");
        assert_eq!(scan_filter_obj.is_selected(), true);
        assert_eq!(scan_filter_obj.get_parameters_length(), 0);
    }

    #[test]
    fn scan_filter_add_parameter(){
        let mut scan_filter_obj = ScanFilter::new();
        assert_eq!(scan_filter_obj.get_parameters_length(), 0);
        let mut new_parameter = ScanFilterParameters::Mobility;
        scan_filter_obj.add_parameter(ScanFilterParameters::Mobility(Mobility::new()));
    }
}
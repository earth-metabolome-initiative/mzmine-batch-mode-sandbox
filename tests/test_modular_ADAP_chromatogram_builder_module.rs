pub use mzbatch_generator::modular_adap_chromatogram_builder_module::*;

use mzbatch_generator::prelude::RawDataFiles;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_m_adap_chr_builder_module_initialization(){
        let obj = ModularADAPChromatogramBuilderModule::new();
        assert_eq!(obj.get_method(), "io.github.mzmine.modules.dataprocessing.featdet_adapchromatogrambuilder.ModularADAPChromatogramBuilderModule");
        assert_eq!(*obj.get_parameter_version(), 1u8);
        assert_eq!(obj.get_parameters_length(), 0);
    }

    #[test]
    fn test_m_adap_chr_builder_module_add_parameter(){
        let mut obj = ModularADAPChromatogramBuilderModule::new();
        assert_eq!(obj.get_parameters_length(), 0);
        obj.add_parameter(ModularADAPChromatogramBuilderModuleParameter::RawDataFiles(RawDataFiles::new()));
        assert_eq!(obj.get_parameters_length(), 1);
    }
}
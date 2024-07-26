use mzbatch_generator::prelude::*;

// fare test serializzazione!!
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mzmine_version_setter() {
        let mut batch = Batch::new("".to_string(), vec![]);
        batch.set_version("1.0.0".to_string());
        assert_eq!("1.0.0", batch.mzmine_version);
    }

    #[test]
    fn test_add_batchsteps(){
        let mut batch = Batch::new("1.0.1".to_string(), vec![]);
        batch.add_batchstep(Modules::AllSpectralDataImportModule);
        assert!(batch.batchsteps > 0);
    }

    #[test]
    fn test_file_names_creation() {
        let batch = Batch::default();
    }

}
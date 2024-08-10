use mzbatch_generator::prelude::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mzmine_version_setter() {
        let mut batch = Batch::new("".to_string(), vec![]);
        batch.set_version("1.0.0".to_string());
        assert_eq!("1.0.0", batch.mzmine_version);
    }

    #[test]
    fn add_batchsteps() {
        let mut batch = Batch::new("1.0.1".to_string(), vec![]);
        batch.add_batchstep(Modules::AllSpectralDataImportModule(AllSpectralDataImportModule::default()));
        assert_eq!(batch.batchstep.len(), 1);
    }

    #[test]
    fn file_names_creation() {
        let batch = Batch::default();
        assert_eq!(batch.mzmine_version, "");
        assert!(batch.batchstep.is_empty());
    }
}
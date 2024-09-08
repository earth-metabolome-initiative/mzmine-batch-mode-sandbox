use mzbatch_generator::prelude::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn mzmine_version_setter() {
        let mut batch = Batch::new("".to_string(), vec![]);
        batch.set_version("1.0.0".to_string());
        assert_eq!(batch.get_mzmine_version(), "1.0.0");
    }

    #[test]
    fn add_batchsteps() {
        let mut batch = Batch::new("1.0.1".to_string(), vec![]);
        batch.add_batchstep(Batchstep::AllSpectralDataImportModule(AllSpectralDataImportModule::default()));
        assert_eq!(batch.get_batchsteps_length(), 1);
    }

    #[test]
    fn file_names_creation() {
        let batch = Batch::default();
        assert_eq!(batch.get_mzmine_version(), "");
        assert_eq!(batch.get_batchsteps_length(), 0);
    }
}
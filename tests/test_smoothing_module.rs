use mzbatch_generator::smoothing_module::*;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoothing_module_initialization(){
        let smooth_obj = SmoothingModule::new();
        assert_eq!(smooth_obj.get_method(), "io.github.mzmine.modules.dataprocessing.featdet_smoothing.SmoothingModule");
        assert_eq!(*smooth_obj.get_parameter_version(), 1u8);
        assert_eq!(smooth_obj.get_parameters_lenght(), 0);
    }

    #[test]
    fn smoothing_module_get_paramater(){
        let mut smooth_obj = SmoothingModule::new();
        assert_eq!(smooth_obj.get_parameters_lenght(), 0);
        smooth_obj.add_parameter(SmoothingModuleParameters::FeatureList(FeatureLists::new()));
        assert_eq!(smooth_obj.get_parameters_lenght(), 1);
        let module = smooth_obj.get_parameter("Feature list").unwrap();
        assert_eq!(module.get_name(), "Feature lists")
    }
}
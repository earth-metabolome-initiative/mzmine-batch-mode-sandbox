use mzbatch_generator::smoothing_module_parameters::SmoothingAlgorithm;
use mzbatch_generator::smoothing_module_parameters::SmoothingAlgorithmModule;

use mzbatch_generator::smoothing_module_parameters::SavitzkyGolay;
use mzbatch_generator::smoothing_module_parameters::SavitzkyGolayParameter;
use mzbatch_generator::smoothing_module_parameters::RetentionTimeSmoothing;
use mzbatch_generator::smoothing_module_parameters::MobilitySmoothing;

use mzbatch_generator::smoothing_module_parameters::LoessSmoothing;
use mzbatch_generator::smoothing_module_parameters::LoessSmoothingParameter;
use mzbatch_generator::smoothing_module_parameters::RetentionTimeWidth;
use mzbatch_generator::smoothing_module_parameters::MobilityWidth;



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn smoothing_algorithm_initialization(){
        let smooth_algo_obj = SmoothingAlgorithm::new();
        assert_eq!(smooth_algo_obj.get_name(), "Smoothing algorithm");
        assert_eq!(smooth_algo_obj.get_selected_item(), "");
        assert_eq!(smooth_algo_obj.get_modules_length(), 0);
    }

    #[test] 
    fn smoothing_algorithm_add_module(){
        let mut smooth_algo_obj = SmoothingAlgorithm::new();
        assert_eq!(smooth_algo_obj.get_modules_length(), 0);
        smooth_algo_obj.add_module(SmoothingAlgorithmModule::SavitzkyGolay(SavitzkyGolay::new()));
        assert_eq!(smooth_algo_obj.get_modules_length(), 1);
    }

    #[test]
    fn smoothing_algorithm_get_module(){
        let mut smooth_algo_obj = SmoothingAlgorithm::new();
        assert_eq!(smooth_algo_obj.get_modules_length(), 0);
        smooth_algo_obj.add_module(SmoothingAlgorithmModule::SavitzkyGolay(SavitzkyGolay::new()));
        assert_eq!(smooth_algo_obj.get_modules_length(), 1);
        assert_eq!(smooth_algo_obj.get_module("Savitzky golay").unwrap().get_name(), "Savitzky Golay");
        smooth_algo_obj.add_module(SmoothingAlgorithmModule::LoessSmoothing(LoessSmoothing::new()));
        assert_eq!(smooth_algo_obj.get_modules_length(), 2);
        assert_eq!(smooth_algo_obj.get_module("Loess smoothing").unwrap().get_name(), "Loess smoothing");
    }

    #[test]
    fn savitzky_golay_initialization(){
        let sg_obj = SavitzkyGolay::new();
        assert_eq!(sg_obj.get_name(), "Savitzky Golay");
        assert_eq!(sg_obj.is_selected(), false);
        assert_eq!(sg_obj.get_parameters_length(), 0);
    }

    #[test]
    fn savitzky_golay_selection(){
        let mut sg_obj = SavitzkyGolay::new();
        assert_eq!(sg_obj.is_selected(), false);
        sg_obj.select();
        assert_eq!(sg_obj.is_selected(), true);
        sg_obj.deselect();
        assert_eq!(sg_obj.is_selected(), false);
        sg_obj.invert_selected();
        assert_eq!(sg_obj.is_selected(), true);
    }

    #[test]
    fn savitzky_golay_add_parameter(){
        let mut sg_obj = SavitzkyGolay::new();
        assert_eq!(sg_obj.get_parameters_length(), 0);
        sg_obj.add_parameter(SavitzkyGolayParameter::RetentionTimeSmoothing(RetentionTimeSmoothing::new()));
        assert_eq!(sg_obj.get_parameters_length(), 1);
        assert_eq!(sg_obj.get_parameter("Retention time smoothing").unwrap().get_name(), "Retention time smoothing");
        sg_obj.add_parameter(SavitzkyGolayParameter::MobilitySmoothing(MobilitySmoothing::new()));
        assert_eq!(sg_obj.get_parameters_length(), 2);
        assert_eq!(sg_obj.get_parameter("Mobility smoothing").unwrap().get_name(), "Mobility smoothing");
    }

    #[test]
    fn retention_time_initialization(){
        let rts_obj = RetentionTimeSmoothing::new();
        assert_eq!(rts_obj.get_name(), "Retention time smoothing");
        assert_eq!(rts_obj.is_selected(), false);
        assert_eq!(*rts_obj.get_value(), None);
    }

    #[test]
    fn retention_time_selection(){
        let mut rts_obj = RetentionTimeSmoothing::new();
        assert_eq!(rts_obj.is_selected(), false);
        rts_obj.select();
        assert_eq!(rts_obj.is_selected(), true);
        rts_obj.deselect();
        assert_eq!(rts_obj.is_selected(), false);
        rts_obj.invert_selected();
        assert_eq!(rts_obj.is_selected(), true);
    }

    #[test]
    fn retention_time_set_get_value(){
        let mut rts_obj = RetentionTimeSmoothing::new();
        assert_eq!(*rts_obj.get_value(), None);
        rts_obj.set_value(Some(34.3));
        assert_eq!(*rts_obj.get_value(), Some(34.3));
    }

    #[test]
    fn mobility_smoothing_initialization(){
        let ms_obj = MobilitySmoothing::new();
        assert_eq!(ms_obj.get_name(), "Mobility smoothing");
        assert_eq!(ms_obj.is_selected(), false);
        assert_eq!(*ms_obj.get_value(), None);
    }

    #[test]
    fn mobility_smoothing_selection(){
        let mut ms_obj = MobilitySmoothing::new();
        assert_eq!(ms_obj.is_selected(), false);
        ms_obj.select();
        assert_eq!(ms_obj.is_selected(), true);
        ms_obj.deselect();
        assert_eq!(ms_obj.is_selected(), false);
        ms_obj.invert_selected();
        assert_eq!(ms_obj.is_selected(), true);
    }

    #[test]
    fn mobility_smoothing_set_get_value(){
        let mut ms_obj = MobilitySmoothing::new();
        assert_eq!(*ms_obj.get_value(), None);
        ms_obj.set_value(Some(12.4));
        assert_eq!(*ms_obj.get_value(), Some(12.4));
    }

    #[test]
    fn loess_smoothing_initialization(){
        let ls_obj = LoessSmoothing::new();
        assert_eq!(ls_obj.get_name(), "Loess smoothing");
        assert_eq!(ls_obj.is_selected(), false);
        assert_eq!(ls_obj.get_parameters_length(), 0);
    }

    #[test]
    fn loess_smoothing_selection(){
        let mut ls_obj = LoessSmoothing::new();
        assert_eq!(ls_obj.is_selected(), false);
        ls_obj.select();
        assert_eq!(ls_obj.is_selected(), true);
        ls_obj.deselect();
        assert_eq!(ls_obj.is_selected(), false);
        ls_obj.invert_selected();
        assert_eq!(ls_obj.is_selected(), true);
    }

    #[test]
    fn loess_smoothing_add_parameter(){
        let mut ls_obj = LoessSmoothing::new();
        assert_eq!(ls_obj.get_parameters_length(), 0);
        ls_obj.add_parameter(LoessSmoothingParameter::MobilityWidth(MobilityWidth::new()));
        assert_eq!(ls_obj.get_parameters_length(), 1);
        ls_obj.add_parameter(LoessSmoothingParameter::RetentionTimeWidth(RetentionTimeWidth::new()));
        assert_eq!(ls_obj.get_parameters_length(), 2);
    }

    #[test]
    fn loess_smoothing_get_parameter_and_parameter_name(){
        let mut ls_obj = LoessSmoothing::new();
        assert_eq!(ls_obj.get_parameters_length(), 0);
        ls_obj.add_parameter(LoessSmoothingParameter::MobilityWidth(MobilityWidth::new()));
        assert_eq!(ls_obj.get_parameters_length(), 1);
        ls_obj.add_parameter(LoessSmoothingParameter::RetentionTimeWidth(RetentionTimeWidth::new()));
        assert_eq!(ls_obj.get_parameters_length(), 2);

        assert_eq!(ls_obj.get_parameter("Mobility width").unwrap().get_name(), "Mobility width (scans)");
        assert_eq!(ls_obj.get_parameter("Retention time width").unwrap().get_name(), "Retention time width (scans)");

    }

    #[test]
    fn loess_smoothing_set_get_parameter_value(){
        let mut ls_obj = LoessSmoothing::new();
        assert_eq!(ls_obj.get_parameters_length(), 0);
        ls_obj.add_parameter(LoessSmoothingParameter::MobilityWidth(MobilityWidth::new()));
        assert_eq!(ls_obj.get_parameters_length(), 1);
        ls_obj.add_parameter(LoessSmoothingParameter::RetentionTimeWidth(RetentionTimeWidth::new()));
        assert_eq!(ls_obj.get_parameters_length(), 2);

        let mut mobility_parameter = ls_obj.get_parameter("Mobility width").unwrap();
        mobility_parameter.set_value(Some(1.0));

        let mut retention_parameter = ls_obj.get_parameter("Retention time width").unwrap();
        retention_parameter.set_value(Some(2.0));

        assert_eq!(*ls_obj.get_parameter("Mobility width").unwrap().get_value(), Some(1.0));
        assert_eq!(*ls_obj.get_parameter("Retention time width").unwrap().get_value(), Some(2.0));
    }

    #[test]
    fn mobility_width_initialization(){
        let mw_obj = MobilityWidth::new();
        assert_eq!(mw_obj.get_name(), "Mobility width (scans)");
        assert_eq!(mw_obj.is_selected(), true);
        assert_eq!(*mw_obj.get_value(), None);
    }

    #[test]
    fn mobility_width_selection(){
        let mut mw_obj = MobilityWidth::new();
        assert_eq!(mw_obj.is_selected(), true);
        mw_obj.deselect();
        assert_eq!(mw_obj.is_selected(), false);
        mw_obj.select();
        assert_eq!(mw_obj.is_selected(), true);
        mw_obj.invert_selected();
        assert_eq!(mw_obj.is_selected(), false);
    }

    #[test]
    fn mobility_width_set_get_value(){
        let mut mw_obj = MobilitySmoothing::new();
        assert_eq!(*mw_obj.get_value(), None);
        mw_obj.set_value(Some(1.1));
        assert_eq!(*mw_obj.get_value(), Some(1.1));
    }

    #[test]
    fn retention_time_width_initialization(){
        let rtw_obj = RetentionTimeWidth::new();
        assert_eq!(rtw_obj.get_name(), "Retention time width (scans)");
        assert_eq!(rtw_obj.is_selected(), true);
        assert_eq!(*rtw_obj.get_value(), None);
    }

    #[test]
    fn retention_time_width_selection(){
        let mut rtw_obj = RetentionTimeWidth::new();
        assert_eq!(rtw_obj.is_selected(), true);
        rtw_obj.deselect();
        assert_eq!(rtw_obj.is_selected(), false);
        rtw_obj.select();
        assert_eq!(rtw_obj.is_selected(), true);
        rtw_obj.invert_selected();
        assert_eq!(rtw_obj.is_selected(), false);
    }

    #[test]
    fn retention_time_width_set_get_value(){
        let mut rtw_obj = MobilitySmoothing::new();
        assert_eq!(*rtw_obj.get_value(), None);
        rtw_obj.set_value(Some(1.1));
        assert_eq!(*rtw_obj.get_value(), Some(1.1));
    }
}
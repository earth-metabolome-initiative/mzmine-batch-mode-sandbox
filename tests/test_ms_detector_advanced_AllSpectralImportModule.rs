use mzbatch_generator::all_spectral_data_import_module_parameters::*;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ms_detector_advanced_initialization(){
        let ms_detector_obj = MSDetectorAdvanced::new();
        assert_eq!(ms_detector_obj.get_name(), "");
        assert_eq!(ms_detector_obj.is_selected(), true);
        assert_eq!(ms_detector_obj.module_length(), 0)
    }

    #[test]
    fn ms_detector_advanced_selection(){
        let mut ms_detector_obj = MSDetectorAdvanced::new();
        assert_eq!(ms_detector_obj.is_selected(), true);
        ms_detector_obj.invert_selected();
        assert_eq!(ms_detector_obj.is_selected(), false);
        ms_detector_obj.select();
        assert_eq!(ms_detector_obj.is_selected(), true);
        ms_detector_obj.deselect();
        assert_eq!(ms_detector_obj.is_selected(), false);
    }

    #[test]
    fn ms_detector_advanced_add_module(){
        let modules: Vec<MSDetectorAdvancedModules> = vec![
            MSDetectorAdvancedModules::Auto(Auto::new()),
            MSDetectorAdvancedModules::Centroid(Centroid::new()),
            MSDetectorAdvancedModules::ExactMass(ExactMass::new()),
            MSDetectorAdvancedModules::FactorOfLowestSignal(FactorOfLowestSignal::new(Some(32.0))),
            MSDetectorAdvancedModules::LocalMaxima(LocalMaxima::new()),
            MSDetectorAdvancedModules::RecursiveThreshold(RecursiveThreshold::new()),
            MSDetectorAdvancedModules::WaveletTransform(WaveletTransform::new())
        ];

        let mut ms_detector_obj = MSDetectorAdvanced::new();
        let mut length = 0;

        for module in modules{
            ms_detector_obj.add_module(module);
            length += 1;
            assert_eq!(ms_detector_obj.module_length(), length);
        }
    }

    #[test]
    fn ms1_content(){
        let mut ms_detector = MSDetectorAdvanced::new();
        assert_eq!(ms_detector.get_name(), "");                                                                                       //test it has been initialized correctly
        ms_detector.set_ms1(Some(0.0));
        assert_eq!(ms_detector.get_name(), "MS1 detector (Advanced)");                                                                //test it has the correct name
        ms_detector.add_module(MSDetectorAdvancedModules::FactorOfLowestSignal(FactorOfLowestSignal::new(Some(0.0)))); 
        assert_eq!(ms_detector.module_length(), 1);                                                                                   //test something has been inserted
        assert_eq!(ms_detector.get_module(0), MSDetectorAdvancedModules::FactorOfLowestSignal(FactorOfLowestSignal::new(Some(0.0)))); //test that it is in fact this type of object
        ms_detector.set_ms1(Some(7.0));
        assert_eq!(*ms_detector.get_module(0).get_value().unwrap(), Some(7.0));
    }

    #[test]
    fn ms2_content(){
        let mut ms_detector = MSDetectorAdvanced::new();
        assert_eq!(ms_detector.get_name(), "", "NOT empty");                                                                                    //test it has been initialized correctly
        ms_detector.set_ms2(Some(0.0));
        assert_eq!(ms_detector.get_name(), "MS2 detector (Advanced)", "NOT same name");                                                             //test it has the correct name
        ms_detector.add_module(MSDetectorAdvancedModules::FactorOfLowestSignal(FactorOfLowestSignal::new(Some(0.0)))); 
        assert_eq!(ms_detector.module_length(), 1, "NOT 1 element pushed");                                                                            //test something has been inserted
        assert_eq!(ms_detector.get_module(0), MSDetectorAdvancedModules::FactorOfLowestSignal(FactorOfLowestSignal::new(Some(0.0))), "NOT good type inserted"); //test that it is in fact this type of object

        ms_detector.set_ms2(Some(1000.0));
        assert_eq!(*ms_detector.get_module(0).get_value().unwrap(), Some(1000.0), "NOT matching value");
    }

    #[test]
    fn auto_initialization(){
        let auto_obj = Auto::new();
        assert_eq!(auto_obj.get_name(), "Auto");
    }

    #[test]
    fn auto_set_get_value(){
        let mut auto_obj = Auto::new();
        auto_obj.set_value(Some(17.0));
        assert_eq!(*auto_obj.get_value(), Some(17.0));
    }

    #[test]
    fn parameter_auto_initialization(){
        let parameter_auto = ParameterAuto::new();
        assert_eq!(parameter_auto.get_name(), "Noise level");
    }

    #[test]
    fn parameter_auto_set_get_value(){
        let mut parameter_auto_obj = ParameterAuto::new();
        assert_eq!(*parameter_auto_obj.get_value(), Some(1000.0));
        parameter_auto_obj.set_value(Some(28.4));
        assert_eq!(*parameter_auto_obj.get_value(), Some(28.4));
    }

    #[test]
    fn centroid_initialization(){
        let centroid_obj = Centroid::new();
        assert_eq!(centroid_obj.get_name(), "Centroid");
        assert_eq!(*centroid_obj.get_value(), None);
    }


    #[test]
    fn centroid_set_get_value(){
        let mut centroid_obj = Centroid::new();
        assert_eq!(*centroid_obj.get_value(), None);
        centroid_obj.set_value(Some(34.8));
        assert_eq!(*centroid_obj.get_value(), Some(34.8));
    }

    #[test]
    fn centroid_parameter_initialization(){
        let par_centroid_obj = ParameterCentroid::new();
        assert_eq!(par_centroid_obj.get_name(), "Noise level");
    }

    #[test]
    fn centroid_parameter_set_get_value(){
        let mut par_centroid_obj = ParameterCentroid::new();
        assert_eq!(*par_centroid_obj.get_value(), None);
        par_centroid_obj.set_value(Some(12.35));
        assert_eq!(*par_centroid_obj.get_value(), Some(12.35));
    }

    #[test]
    fn exact_mass_initialization(){
        let centroid_obj = ExactMass::new();
        assert_eq!(centroid_obj.get_name(), "Exact mass");
        assert_eq!(*centroid_obj.get_value(), None);
    }


    #[test]
    fn exact_mass_set_get_value(){
        let mut exact_mass_obj = ExactMass::new();
        assert_eq!(*exact_mass_obj.get_value(), None);
        exact_mass_obj.set_value(Some(34.8));
        assert_eq!(*exact_mass_obj.get_value(), Some(34.8));
    }

    #[test]
    fn exact_mass_parameter_initialization(){
        let par_exact_mass_obj = ParameterExactMass::new();
        assert_eq!(par_exact_mass_obj.get_name(), "Noise level");
    }

    #[test]
    fn exact_mass_parameter_set_get_value(){
        let mut par_centroid_obj = ParameterExactMass::new();
        assert_eq!(*par_centroid_obj.get_value(), None);
        par_centroid_obj.set_value(Some(12.35));
        assert_eq!(*par_centroid_obj.get_value(), Some(12.35));
    }

    #[test]
    fn local_maxima_initialization(){
        let centroid_obj = LocalMaxima::new();
        assert_eq!(centroid_obj.get_name(), "Local maxima");
        assert_eq!(*centroid_obj.get_value(), None);
    }


    #[test]
    fn local_maxima_set_get_value(){
        let mut local_maxima_obj = LocalMaxima::new();
        assert_eq!(*local_maxima_obj.get_value(), None);
        local_maxima_obj.set_value(Some(34.8));
        assert_eq!(*local_maxima_obj.get_value(), Some(34.8));
    }

    #[test]
    fn local_maxima_parameter_initialization(){
        let par_local_maxima_obj = ParameterLocalMaxima::new();
        assert_eq!(par_local_maxima_obj.get_name(), "Noise level");
    }

    #[test]
    fn local_maxima_parameter_set_get_value(){
        let mut par_local_maxima_obj = ParameterLocalMaxima::new();
        assert_eq!(*par_local_maxima_obj.get_value(), None);
        par_local_maxima_obj.set_value(Some(12.35));
        assert_eq!(*par_local_maxima_obj.get_value(), Some(12.35));
    }

    #[test]
    fn recursive_threshold_initialization(){
        let recursive_thr_obj = RecursiveThreshold::new();
        assert_eq!(recursive_thr_obj.get_name(), "Recursive threshold");
        assert_eq!(recursive_thr_obj.paramater_length(), 0);
    }

    #[test]
    fn recursive_threshold_get_set_value(){
        let mut recursive_thr_obj = RecursiveThreshold::new();
        assert_eq!(recursive_thr_obj.paramater_length(), 0);

        recursive_thr_obj.add_parameter(RecursiveThresholdParameters::RTNoiseLevel(RTNoiseLevel::new()));
        recursive_thr_obj.set_parameter_value("RTNoiseLevel", Some(2.2));
        assert_eq!(recursive_thr_obj.get_parameter_value("RTNoiseLevel"), Some(2.2), "At RTNoiseLevel");

        recursive_thr_obj.add_parameter(RecursiveThresholdParameters::MinMZPeakWidth(MinMZPeakWidth::new()));
        recursive_thr_obj.set_parameter_value("MinMZPeakWidth", Some(4.4));
        assert_eq!(recursive_thr_obj.get_parameter_value("MinMZPeakWidth"), Some(4.4), "At MinMZPeakWidth");

        recursive_thr_obj.add_parameter(RecursiveThresholdParameters::MaxMZPeakWidth(MaxMZPeakWidth::new()));
        recursive_thr_obj.set_parameter_value("MaxMZPeakWidth", Some(6.6));
        assert_eq!(recursive_thr_obj.get_parameter_value("MaxMZPeakWidth"), Some(6.6), "At MinMZPeakWidth");
    } 

    #[test]
    fn rtnoise_level_initialization(){
        let rt_noise_level_obj = RTNoiseLevel::new();
        assert_eq!(rt_noise_level_obj.get_name(), "Noise level");
        assert_eq!(rt_noise_level_obj.get_value(), None);
    }

    #[test]
    fn rtnoise_level_get_set(){
        let mut rtnoise_obj = RTNoiseLevel::new();
        assert_eq!(rtnoise_obj.get_value(), None);
        rtnoise_obj.set_value(Some(3.3));
        assert_eq!(rtnoise_obj.get_value(), Some(3.3));
    }

    #[test]
    fn min_mz_peak_width_initialization(){
        let min_mz_peak_width_obj = MinMZPeakWidth::new();
        assert_eq!(min_mz_peak_width_obj.get_name(), "Min m/z peak width");
        assert_eq!(min_mz_peak_width_obj.get_value(), None);
    }

    #[test]
    fn min_mz_peak_width_get_set(){
        let mut min_mz_peak_width_obj = MinMZPeakWidth::new();
        assert_eq!(min_mz_peak_width_obj.get_value(), None);
        min_mz_peak_width_obj.set_value(Some(3.3));
        assert_eq!(min_mz_peak_width_obj.get_value(), Some(3.3));
    }

    #[test]
    fn max_mz_peak_width_initialization(){
        let max_mz_peak_width_obj = MaxMZPeakWidth::new();
        assert_eq!(max_mz_peak_width_obj.get_name(), "Max m/z peak width");
        assert_eq!(max_mz_peak_width_obj.get_value(), None);
    }

    #[test]
    fn max_mz_peak_width_get_set(){
        let mut max_mz_peak_width_obj = MaxMZPeakWidth::new();
        assert_eq!(max_mz_peak_width_obj.get_value(), None);
        max_mz_peak_width_obj.set_value(Some(1.1));
        assert_eq!(max_mz_peak_width_obj.get_value(), Some(1.1));
    }

    #[test]
    fn wavelet_transform_initialization(){
        let wavelet_obj = WaveletTransform::new();
        assert_eq!(wavelet_obj.get_name(), "Wavelet transform");
        assert_eq!(wavelet_obj.parameters_length(), 0);
    }

    #[test]
    fn wavelet_transform_get_set(){
        let mut wavelet_obj = WaveletTransform::new();
        assert_eq!(wavelet_obj.parameters_length(), 0);
        wavelet_obj.add_parameter(WaveletTransformParameters::WTNoiseLevel(WTNoiseLevel::new()));
        wavelet_obj.add_parameter(WaveletTransformParameters::ScaleLevel(ScaleLevel::new()));
        wavelet_obj.add_parameter(WaveletTransformParameters::WaveletWindowSize(WaveletWindowSize::new()));

        assert_eq!(wavelet_obj.parameters_length(), 3);

        wavelet_obj.set_parameter_value("WTNoiseLevel", Some(1.2));
        wavelet_obj.set_parameter_value("ScaleLevel", Some(2.3));
        wavelet_obj.set_parameter_value("WaveletWindowSize", Some(3.4));

        assert_eq!(wavelet_obj.get_parameter_value("WTNoiseLevel"), Some(1.2));
        assert_eq!(wavelet_obj.get_parameter_value("ScaleLevel"), Some(2.3));
        assert_eq!(wavelet_obj.get_parameter_value("WaveletWindowSize"), Some(3.4));
    }

    #[test]
    fn wt_noise_level_initialization(){
        let wt_obj = WTNoiseLevel::new();
        assert_eq!(wt_obj.get_name(), "Noise level");
        assert_eq!(wt_obj.get_value(), None);
    }

    #[test]
    fn wt_noise_level_get_set(){
        let mut wt_obj = WTNoiseLevel::new();
        assert_eq!(wt_obj.get_value(), None);
        wt_obj.set_value(Some(2.2));
        assert_eq!(wt_obj.get_value(), Some(2.2));
    }

    #[test]
    fn scale_level_initialization(){
        let scale_obj = ScaleLevel::new();
        assert_eq!(scale_obj.get_name(), "Scale level");
        assert_eq!(scale_obj.get_value(), None);
    }

    #[test]
    fn scale_level_get_set(){
        let mut scale_obj = ScaleLevel::new();
        assert_eq!(scale_obj.get_value(), None);
        scale_obj.set_value(Some(1.4));
        assert_eq!(scale_obj.get_value(), Some(1.4));
    }

    #[test]
    fn wavelet_window_size_initialization(){
        let window_obj = WaveletWindowSize::new();
        assert_eq!(window_obj.get_name(), "Wavelet window size (%)");
        assert_eq!(window_obj.get_value(), None);
    }

    #[test]
    fn wavelet_window_size_get_set(){
        let mut window_obj = WaveletWindowSize::new();
        assert_eq!(window_obj.get_value(), None);
        window_obj.set_value(Some(5.4));
        assert_eq!(window_obj.get_value(), Some(5.4));
    }

    #[test]
    fn denormalize_fragment_scans_traps_initialization(){
        let dfst_obj: DenormalizeFragmentScansTraps = DenormalizeFragmentScansTraps::new();
        assert_eq!(dfst_obj.get_name(), "Denormalize fragment scans (traps)");
        assert_eq!(dfst_obj.get_value(), true);
    }

    #[test]
    fn denormalize_fragment_scans_traps_set_value(){
        let mut dfst_obj = DenormalizeFragmentScansTraps::new();
        assert_eq!(dfst_obj.get_value(), true);
        dfst_obj.set_false();
        assert_eq!(dfst_obj.get_value(), false);
        dfst_obj.set_true();
        assert_eq!(dfst_obj.get_value(), true);
    }
}
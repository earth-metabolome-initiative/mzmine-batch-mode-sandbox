use mzbatch_generator::all_spectral_data_import_module_parameters::*;

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn ms_detector_advanced_initialization(){
        let ms_detector_obj = MSDetectorAdvanced::new();
        assert_eq!(ms_detector_obj.get_name(), "");
        assert_eq!(ms_detector_obj.is_selected(), true);
        assert_eq!(ms_detector_obj.get_module_length(), 0)
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
            MSDetectorAdvancedModules::FactorOfLowestSignal(FactorOfLowestSignal::new()),
            MSDetectorAdvancedModules::LocalMaxima(LocalMaxima::new()),
            MSDetectorAdvancedModules::RecursiveThreshold(RecursiveThreshold::new()),
            MSDetectorAdvancedModules::WaveletTransform(WaveletTransform::new())
        ];

        let mut ms_detector_obj = MSDetectorAdvanced::new();
        let mut length = 0;

        for module in modules{
            ms_detector_obj.add_module(module);
            length += 1;
            assert_eq!(ms_detector_obj.get_module_length(), length);
        }
    }

    #[test]
    fn ms_detector_serialization() -> Result<(), Box<dyn std::error::Error>>{
        let mut ms_detector_obj = MSDetectorAdvanced::new();
        
        let factor = FactorOfLowestSignal::new();

        // add here otherwise .set_ms1() does not add the parameter value
        ms_detector_obj.add_module(MSDetectorAdvancedModules::FactorOfLowestSignal(factor));
        ms_detector_obj.set_ms1(Some(5.0));


        let mut auto = Auto::new();
        auto.set_value(Some(1000.0));

        let centroid = Centroid::new();

        let mass = ExactMass::new();

        let maxima = LocalMaxima::new();

        let mut recursive = RecursiveThreshold::new();
        let rec_par_1 = RTNoiseLevel::new();
        let rec_par_2 = MinMZPeakWidth::new();
        let rec_par_3 = MaxMZPeakWidth::new();
        recursive.add_parameter(RecursiveThresholdParameters::RTNoiseLevel(rec_par_1));
        recursive.add_parameter(RecursiveThresholdParameters::MinMZPeakWidth(rec_par_2));
        recursive.add_parameter(RecursiveThresholdParameters::MaxMZPeakWidth(rec_par_3));

        let mut wavelet = WaveletTransform::new();
        let wav_par_1 = WTNoiseLevel::new();
        let wav_par_2: ScaleLevel = ScaleLevel::new();
        let wav_par_3: WaveletWindowSize = WaveletWindowSize::new();
        wavelet.add_parameter(WaveletTransformParameters::WTNoiseLevel(wav_par_1));
        wavelet.add_parameter(WaveletTransformParameters::ScaleLevel(wav_par_2));
        wavelet.add_parameter(WaveletTransformParameters::WaveletWindowSize(wav_par_3));

        ms_detector_obj.add_module(MSDetectorAdvancedModules::Auto(auto));
        ms_detector_obj.add_module(MSDetectorAdvancedModules::Centroid(centroid));
        ms_detector_obj.add_module(MSDetectorAdvancedModules::ExactMass(mass));
        ms_detector_obj.add_module(MSDetectorAdvancedModules::LocalMaxima(maxima));
        ms_detector_obj.add_module(MSDetectorAdvancedModules::RecursiveThreshold(recursive));
        ms_detector_obj.add_module(MSDetectorAdvancedModules::WaveletTransform(wavelet));

        let mut buffer = "".to_owned(); // Create the string buffer for the XML content
        quick_xml::se::to_writer(&mut buffer, &ms_detector_obj)?;
    
        // IMPORTANT
        // serializer print int if float is .0
        let expected = r#"<parameter name="MS1 detector (Advanced)" selected="true" selected_item="Factor of lowest signal"><module name="Factor of lowest signal"><parameter name="Noise factor">5</parameter></module><module name="Auto"><parameter name="Noise level">1000</parameter></module><module name="Centroid"><parameter name="Noise level"/></module><module name="Exact mass"><parameter name="Noise level"/></module><module name="Local maxima"><parameter name="Noise level"/></module><module name="Recursive threshold"><parameter name="Noise level"/><parameter name="Min m/z peak width"/><parameter name="Max m/z peak width"/></module><module name="Wavelet transform"><parameter name="Noise level"/><parameter name="Scale level"/><parameter name="Wavelet window size (%)"/></module></parameter>"#;
       
        assert_eq!(buffer, expected, ".set_ms1() failed to serialize correctly");

//
//                                       ########################################
//                                       ###      Test MS2 serialization      ###
//                                       ########################################
//

        let mut buffer = "".to_owned(); // Create the string buffer for the XML content

        ms_detector_obj.set_ms2(Some(0.0));

        quick_xml::se::to_writer(&mut buffer, &ms_detector_obj)?;
    
        // IMPORTANT
        // serializer print int if float is .0
        let expected = r#"<parameter name="MS2 detector (Advanced)" selected="true" selected_item="Factor of lowest signal"><module name="Factor of lowest signal"><parameter name="Noise factor">0</parameter></module><module name="Auto"><parameter name="Noise level">1000</parameter></module><module name="Centroid"><parameter name="Noise level"/></module><module name="Exact mass"><parameter name="Noise level"/></module><module name="Local maxima"><parameter name="Noise level"/></module><module name="Recursive threshold"><parameter name="Noise level"/><parameter name="Min m/z peak width"/><parameter name="Max m/z peak width"/></module><module name="Wavelet transform"><parameter name="Noise level"/><parameter name="Scale level"/><parameter name="Wavelet window size (%)"/></module></parameter>"#;
        
        assert_eq!(buffer, expected, ".set_ms2() failed to serialize correctly");

        Ok(())
    }
 
    #[test]
    fn ms1_content(){
        let mut ms_detector = MSDetectorAdvanced::new();
        assert_eq!(ms_detector.get_name(), "");                                                                                       //test it has been initialized correctly
        ms_detector.set_ms1(Some(0.0));
        assert_eq!(ms_detector.get_name(), "MS1 detector (Advanced)");                                                                //test it has the correct name
        ms_detector.add_module(MSDetectorAdvancedModules::FactorOfLowestSignal(FactorOfLowestSignal::new())); 
        assert_eq!(ms_detector.get_module_length(), 1);                                                                                   //test something has been inserted
        assert_eq!(ms_detector.get_module("FactorOfLowestSignal").get_value().unwrap(), &None); //test that it is in fact this type of object
        ms_detector.set_ms1(Some(7.0));
        assert_eq!(*ms_detector.get_module("FactorOfLowestSignal").get_value().unwrap(), Some(7.0));
    }

    #[test]
    fn ms2_content(){
        let mut ms_detector = MSDetectorAdvanced::new();
        assert_eq!(ms_detector.get_name(), "", "NOT empty");                                                                                    //test it has been initialized correctly
        ms_detector.set_ms2(Some(0.0));
        assert_eq!(ms_detector.get_name(), "MS2 detector (Advanced)", "NOT same name");                                                             //test it has the correct name
        ms_detector.add_module(MSDetectorAdvancedModules::FactorOfLowestSignal(FactorOfLowestSignal::new())); 
        assert_eq!(ms_detector.get_module_length(), 1, "NOT 1 element pushed");                                                                            //test something has been inserted
        assert_eq!(ms_detector.get_module("FactorOfLowestSignal").get_value().unwrap(), &None, "NOT good type inserted"); //test that it is in fact this type of object

        ms_detector.set_ms2(Some(1000.0));
        assert_eq!(*ms_detector.get_module("FactorOfLowestSignal").get_value().unwrap(), Some(1000.0), "NOT matching value");
    }

    #[test]
    fn factor_of_lowest_signal_parameter_serialization()-> Result<(), Box<dyn std::error::Error>>{
        let mut par_factor_obj = ParameterFactorOfLowestSignal::new();
        par_factor_obj.set_value(Some(5.0));
    
        let mut buffer = "".to_owned(); // Create the string buffer for the XML content
        quick_xml::se::to_writer(&mut buffer, &par_factor_obj)?;
        
        // IMPORTANT
        // serializer print int if float is .0
        let expected = r#"<parameter name="Noise factor">5</parameter>"#;
        
        assert_eq!(buffer, expected);
        
        Ok(())
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
    fn auto_serialization() -> Result<(), Box<dyn std::error::Error>>{
        let auto_obj = Auto::new();
    
        let mut buffer = "".to_owned(); // Create the string buffer for the XML content
        quick_xml::se::to_writer(&mut buffer, &auto_obj)?;
    
        let expected = r#"<module name="Auto"><parameter name="Noise level"/></module>"#;
        
        assert_eq!(buffer, expected);
    
        Ok(())
    }  
   

    #[test]
    fn parameter_auto_initialization(){
        let parameter_auto = ParameterAuto::new();
        assert_eq!(parameter_auto.get_name(), "Noise level");
    }

    #[test]
    fn parameter_auto_set_get_value(){
        let mut parameter_auto_obj = ParameterAuto::new();
        assert_eq!(*parameter_auto_obj.get_value(), None);
        parameter_auto_obj.set_value(Some(28.4));
        assert_eq!(*parameter_auto_obj.get_value(), Some(28.4));
    }

    #[test]
    fn parameter_auto_serialization() -> Result<(), Box<dyn std::error::Error>>{
        let parameter_auto = ParameterAuto::new();
    
        let mut buffer = "".to_owned(); // Create the string buffer for the XML content
        quick_xml::se::to_writer(&mut buffer, &parameter_auto)?;
    
        let expected = r#"<parameter name="Noise level"/>"#;
        
        assert_eq!(buffer, expected);
    
        Ok(())
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
    fn centroid_serialization() -> Result<(), Box<dyn std::error::Error>>{
        let centroid_obj = Centroid::new();
    
        let mut buffer = "".to_owned(); // Create the string buffer for the XML content
        quick_xml::se::to_writer(&mut buffer, &centroid_obj)?;
    
        let expected = r#"<module name="Centroid"><parameter name="Noise level"/></module>"#;
        
        assert_eq!(buffer, expected);
    
        Ok(())
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
    fn centroid_parameter_serialization() -> Result<(), Box<dyn std::error::Error>>{
        let par_centroid_obj = ParameterCentroid::new();
    
        let mut buffer = "".to_owned(); // Create the string buffer for the XML content
        quick_xml::se::to_writer(&mut buffer, &par_centroid_obj)?;
    
        let expected = r#"<parameter name="Noise level"/>"#;
        
        assert_eq!(buffer, expected);
    
        Ok(())
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
    fn exact_mass_serialization() -> Result<(), Box<dyn std::error::Error>>{
        let exact_mass_obj = ExactMass::new();
    
        let mut buffer = "".to_owned(); // Create the string buffer for the XML content
        quick_xml::se::to_writer(&mut buffer, &exact_mass_obj)?;
    
        let expected = r#"<module name="Exact mass"><parameter name="Noise level"/></module>"#;
        
        assert_eq!(buffer, expected);
    
        Ok(())
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
    fn exact_mass_parameter_serialization() -> Result<(), Box<dyn std::error::Error>>{
        let par_centroid_obj = ParameterExactMass::new();
    
        let mut buffer = "".to_owned(); // Create the string buffer for the XML content
        quick_xml::se::to_writer(&mut buffer, &par_centroid_obj)?;
    
        let expected = r#"<parameter name="Noise level"/>"#;
        
        assert_eq!(buffer, expected);
    
        Ok(())
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
    fn local_maxima_serialization() -> Result<(), Box<dyn std::error::Error>>{
        let local_maxima_obj = LocalMaxima::new();
    
        let mut buffer = "".to_owned(); // Create the string buffer for the XML content
        quick_xml::se::to_writer(&mut buffer, &local_maxima_obj)?;
    
        let expected = r#"<module name="Local maxima"><parameter name="Noise level"/></module>"#;
        
        assert_eq!(buffer, expected);
    
        Ok(())
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
    fn local_maxima_parameter_serialization() -> Result<(), Box<dyn std::error::Error>>{
        let par_local_maxima_obj = ParameterLocalMaxima::new();
    
        let mut buffer = "".to_owned(); // Create the string buffer for the XML content
        quick_xml::se::to_writer(&mut buffer, &par_local_maxima_obj)?;
    
        let expected = r#"<parameter name="Noise level"/>"#;
        
        assert_eq!(buffer, expected);
    
        Ok(())
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
        assert_eq!(*recursive_thr_obj.get_parameter_value("RTNoiseLevel"), Some(2.2), "At RTNoiseLevel");

        recursive_thr_obj.add_parameter(RecursiveThresholdParameters::MinMZPeakWidth(MinMZPeakWidth::new()));
        recursive_thr_obj.set_parameter_value("MinMZPeakWidth", Some(4.4));
        assert_eq!(*recursive_thr_obj.get_parameter_value("MinMZPeakWidth"), Some(4.4), "At MinMZPeakWidth");

        recursive_thr_obj.add_parameter(RecursiveThresholdParameters::MaxMZPeakWidth(MaxMZPeakWidth::new()));
        recursive_thr_obj.set_parameter_value("MaxMZPeakWidth", Some(6.6));
        assert_eq!(*recursive_thr_obj.get_parameter_value("MaxMZPeakWidth"), Some(6.6), "At MinMZPeakWidth");
    } 

    #[test]
    fn recursive_threshold_serialization() -> Result<(), Box<dyn std::error::Error>>{
        let mut recursive_thr_obj = RecursiveThreshold::new();

        recursive_thr_obj.add_parameter(RecursiveThresholdParameters::RTNoiseLevel(RTNoiseLevel::new()));
        recursive_thr_obj.add_parameter(RecursiveThresholdParameters::MinMZPeakWidth(MinMZPeakWidth::new()));
        recursive_thr_obj.add_parameter(RecursiveThresholdParameters::MaxMZPeakWidth(MaxMZPeakWidth::new()));
    
        let mut buffer = "".to_owned(); // Create the string buffer for the XML content
        quick_xml::se::to_writer(&mut buffer, &recursive_thr_obj)?;
    
        let expected = r#"<module name="Recursive threshold"><parameter name="Noise level"/><parameter name="Min m/z peak width"/><parameter name="Max m/z peak width"/></module>"#;
        
        assert_eq!(buffer, expected);
    
        Ok(())
    }
  

    #[test]
    fn rtnoise_level_initialization(){
        let rt_noise_level_obj = RTNoiseLevel::new();
        assert_eq!(rt_noise_level_obj.get_name(), "Noise level");
        assert_eq!(*rt_noise_level_obj.get_value(), None);
    }

    #[test]
    fn rtnoise_level_get_set(){
        let mut rtnoise_obj = RTNoiseLevel::new();
        assert_eq!(*rtnoise_obj.get_value(), None);
        rtnoise_obj.set_value(Some(3.3));
        assert_eq!(*rtnoise_obj.get_value(), Some(3.3));
    }

    #[test]
    fn rtnoise_level_serialization() -> Result<(), Box<dyn std::error::Error>>{
        let rtnoise_obj = RTNoiseLevel::new();
    
        let mut buffer = "".to_owned(); // Create the string buffer for the XML content
        quick_xml::se::to_writer(&mut buffer, &rtnoise_obj)?;
    
        let expected = r#"<parameter name="Noise level"/>"#;
        
        assert_eq!(buffer, expected);
    
        Ok(())
    }
    
    #[test]
    fn min_mz_peak_width_initialization(){
        let min_mz_peak_width_obj = MinMZPeakWidth::new();
        assert_eq!(min_mz_peak_width_obj.get_name(), "Min m/z peak width");
        assert_eq!(*min_mz_peak_width_obj.get_value(), None);
    }

    #[test]
    fn min_mz_peak_width_get_set(){
        let mut min_mz_peak_width_obj = MinMZPeakWidth::new();
        assert_eq!(*min_mz_peak_width_obj.get_value(), None);
        min_mz_peak_width_obj.set_value(Some(3.3));
        assert_eq!(*min_mz_peak_width_obj.get_value(), Some(3.3));
    }

    #[test]
    fn min_mz_peak_width_serialization() -> Result<(), Box<dyn std::error::Error>>{
        let min_mz_peak_width_obj = MinMZPeakWidth::new();
    
        let mut buffer = "".to_owned(); // Create the string buffer for the XML content
        quick_xml::se::to_writer(&mut buffer, &min_mz_peak_width_obj)?;
    
        let expected = r#"<parameter name="Min m/z peak width"/>"#;
        
        assert_eq!(buffer, expected);
    
        Ok(())
    }

    #[test]
    fn max_mz_peak_width_initialization(){
        let max_mz_peak_width_obj = MaxMZPeakWidth::new();
        assert_eq!(max_mz_peak_width_obj.get_name(), "Max m/z peak width");
        assert_eq!(*max_mz_peak_width_obj.get_value(), None);
    }

    #[test]
    fn max_mz_peak_width_get_set(){
        let mut max_mz_peak_width_obj = MaxMZPeakWidth::new();
        assert_eq!(*max_mz_peak_width_obj.get_value(), None);
        max_mz_peak_width_obj.set_value(Some(1.1));
        assert_eq!(*max_mz_peak_width_obj.get_value(), Some(1.1));
    }

    #[test]
    fn max_mz_peak_width_serialization() -> Result<(), Box<dyn std::error::Error>>{
        let max_mz_peak_width_obj = MaxMZPeakWidth::new();
    
        let mut buffer = "".to_owned(); // Create the string buffer for the XML content
        quick_xml::se::to_writer(&mut buffer, &max_mz_peak_width_obj)?;
    
        let expected = r#"<parameter name="Max m/z peak width"/>"#;
        
        assert_eq!(buffer, expected);
    
        Ok(())
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

        assert_eq!(*wavelet_obj.get_parameter_value("WTNoiseLevel"), Some(1.2));
        assert_eq!(*wavelet_obj.get_parameter_value("ScaleLevel"), Some(2.3));
        assert_eq!(*wavelet_obj.get_parameter_value("WaveletWindowSize"), Some(3.4));
    }

    #[test]
    fn wavelet_transform_serialization() -> Result<(), Box<dyn std::error::Error>>{
        let mut wavelet_obj = WaveletTransform::new();
        wavelet_obj.add_parameter(WaveletTransformParameters::WTNoiseLevel(WTNoiseLevel::new()));
        wavelet_obj.add_parameter(WaveletTransformParameters::ScaleLevel(ScaleLevel::new()));
        wavelet_obj.add_parameter(WaveletTransformParameters::WaveletWindowSize(WaveletWindowSize::new()));
    
        let mut buffer = "".to_owned(); // Create the string buffer for the XML content
        quick_xml::se::to_writer(&mut buffer, &wavelet_obj)?;
    
        let expected = r#"<module name="Wavelet transform"><parameter name="Noise level"/><parameter name="Scale level"/><parameter name="Wavelet window size (%)"/></module>"#;
        
        assert_eq!(buffer, expected);
    
        Ok(())
    }

    #[test]
    fn wt_noise_level_initialization(){
        let wt_obj = WTNoiseLevel::new();
        assert_eq!(wt_obj.get_name(), "Noise level");
        assert_eq!(*wt_obj.get_value(), None);
    }

    #[test]
    fn wt_noise_level_get_set(){
        let mut wt_obj = WTNoiseLevel::new();
        assert_eq!(*wt_obj.get_value(), None);
        wt_obj.set_value(Some(2.2));
        assert_eq!(*wt_obj.get_value(), Some(2.2));
    }

    #[test]
    fn wt_noise_level_serialization() -> Result<(), Box<dyn std::error::Error>> {
        let wt_obj = WTNoiseLevel::new();
    
        let mut buffer = "".to_owned(); // Create the string buffer for the XML content
        quick_xml::se::to_writer(&mut buffer, &wt_obj)?;
    
        let expected = r#"<parameter name="Noise level"/>"#;
        
        assert_eq!(buffer, expected);
    
        Ok(())
    }

    #[test]
    fn scale_level_initialization(){
        let scale_obj = ScaleLevel::new();
        assert_eq!(scale_obj.get_name(), "Scale level");
        assert_eq!(*scale_obj.get_value(), None);
    }

    #[test]
    fn scale_level_get_set(){
        let mut scale_obj = ScaleLevel::new();
        assert_eq!(*scale_obj.get_value(), None);
        scale_obj.set_value(Some(1.4));
        assert_eq!(*scale_obj.get_value(), Some(1.4));
    }

    #[test]
    fn scale_level_serialization() -> Result<(), Box<dyn std::error::Error>> {
        let scale_obj = ScaleLevel::new();
    
        let mut buffer = "".to_owned(); // Create the string buffer for the XML content
        quick_xml::se::to_writer(&mut buffer, &scale_obj)?;
    
        let expected = r#"<parameter name="Scale level"/>"#;
        
        assert_eq!(buffer, expected);
    
        Ok(())
    }

    #[test]
    fn wavelet_window_size_initialization(){
        let window_obj = WaveletWindowSize::new();
        assert_eq!(window_obj.get_name(), "Wavelet window size (%)");
        assert_eq!(*window_obj.get_value(), None);
    }

    #[test]
    fn wavelet_window_size_get_set(){
        let mut window_obj = WaveletWindowSize::new();
        assert_eq!(*window_obj.get_value(), None);
        window_obj.set_value(Some(5.4));
        assert_eq!(*window_obj.get_value(), Some(5.4));
    }

    #[test]
    fn wavelet_window_size_serialization() -> Result<(), Box<dyn std::error::Error>> {
        let window_obj = WaveletWindowSize::new();
    
        let mut buffer = "".to_owned(); // Create the string buffer for the XML content
        quick_xml::se::to_writer(&mut buffer, &window_obj)?;
    
        let expected = r#"<parameter name="Wavelet window size (%)"/>"#;
        
        assert_eq!(buffer, expected);
    
        Ok(())
    }

    #[test]
    fn denormalize_fragment_scans_traps_initialization(){
        let dfst_obj: DenormalizeFragmentScansTraps = DenormalizeFragmentScansTraps::new();
        assert_eq!(dfst_obj.get_name(), "Denormalize fragment scans (traps)");
        assert_eq!(*dfst_obj.get_value(), true);
    }

    #[test]
    fn denormalize_fragment_scans_traps_set_value(){
        let mut dfst_obj = DenormalizeFragmentScansTraps::new();
        assert_eq!(*dfst_obj.get_value(), true);
        dfst_obj.set_false();
        assert_eq!(*dfst_obj.get_value(), false);
        dfst_obj.set_true();
        assert_eq!(*dfst_obj.get_value(), true);
    }

    #[test]
    fn denormalize_fragment_scan_traps_serialization() -> Result<(), Box<dyn std::error::Error>> {
        let mut denormalize_obj = DenormalizeFragmentScansTraps::new();
        denormalize_obj.set_true();
    
        let mut buffer = "".to_owned(); // Create the string buffer for the XML content
        quick_xml::se::to_writer(&mut buffer, &denormalize_obj)?;
    
        let expected = r#"<parameter name="Denormalize fragment scans (traps)">true</parameter>"#;
        
        assert_eq!(buffer, expected);
    
        Ok(())
    }
}
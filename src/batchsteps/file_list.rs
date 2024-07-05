use simple_xml_builder::XMLElement;

struct Scan_filter {
    selected: bool,
    scan_number: bool,
    base_iltering_integer: bool,
    retention_time: bool,
    mobility: bool,
    ms_level_filter: bool,
    scan_definition: bool,
    polarity: String,
    spectrum_type: String,
}

impl Default for Scan_filter {
    fn default() -> Scan_filter {
        Scan_filter {
            selected: false,
            scan_number: false,
            base_iltering_integer: false,
            retention_time: false,
            mobility: false,
            ms_level_filter: false,
            scan_definition: false,
            polarity: String::from("ANY"),
            spectrum_type: String::from("ANY"),
        }
    }
}

struct Crop_MS1_mz {
    selected: bool,
}

impl Default for Crop_MS1_mz {
    fn default() -> Crop_MS1_mz {
        Crop_MS1_mz {
            selected: false,
        }
    }
}

struct MS1_detector_advanced {
    selected: bool,
    selected_item: String,

    modules: Vec<Vec<String>>,
}

impl Default for MS1_detector_advanced {
    fn default() -> MS1_detector_advanced {
        MS1_detector_advanced {
            selected: false,
            selected_item: String::from("Factor of lowest signal"),
            modules: vec![
                vec![
                    "Factor of lowest signal".to_string(),
                    "Noise factor".to_string(),
                    "5.0".to_string(),
                ],
                vec![
                    "Auto".to_string(),
                    "Noise level".to_string(),
                    "1000.0".to_string(),
                ],
                vec![
                    "Centroid".to_string(),
                    "Noise level".to_string(),
                ],
                vec![
                    "Exact mass".to_string(),
                    "Noise level".to_string(),
                ],
                vec![
                    "Local maxima".to_string(),
                    "Noise level".to_string(),
                ],
                vec![
                    "Recursive threshold".to_string(),
                    "Noise level".to_string(),
                    "Min m/z peak width".to_string(),
                    "Max m/z peak width".to_string(),
                ],
                vec![
                    "Wavelet transform".to_string(),
                    "Noise level".to_string(),
                    "Scale level".to_string(),
                    "Wavelet window size (%)".to_string(),
                ],
            ],
            
        }
    }
}


// Pretty much the same as MS1, redundancy or needed?
struct MS2_detector_advanced {
    selected: bool,
    selected_item: String,

    modules: Vec<Vec<String>>,
}

impl Default for MS2_detector_advanced {
    fn default() -> MS2_detector_advanced {
        MS2_detector_advanced {
            selected: false,
            selected_item: String::from("Factor of lowest signal"),

            modules: vec![
                vec![
                    "Factor of lowest signal".to_string(),
                    "Noise factor".to_string(),
                    "0.0".to_string(),
                ],
                vec![
                    "Auto".to_string(),
                    "Noise level".to_string(),
                    "1000.0".to_string(),
                ],
                vec![
                    "Centroid".to_string(),
                    "Noise level".to_string(),
                ],
                vec![
                    "Exact mass".to_string(),
                    "Noise level".to_string(),
                ],
                vec![
                    "Local maxima".to_string(),
                    "Noise level".to_string(),
                ],
                vec![
                    "Recursive threshold".to_string(),
                    "Noise level".to_string(),
                    "Min m/z peak width".to_string(),
                    "Max m/z peak width".to_string(),
                ],
                vec![
                    "Wavelet transform".to_string(),
                    "Noise level".to_string(),
                    "Scale level".to_string(),
                    "Wavelet window size (%)".to_string(),
                ],
            ],
            
        }
    }
}

struct Denormalize_fragment_scans_traps {
    selected: bool,
    value: String,
}

impl Default for Denormalize_fragment_scans_traps {
    fn default() -> Denormalize_fragment_scans_traps {
        Denormalize_fragment_scans_traps {
            selected: false,
            value: String::from("true")
        }
    }
}

struct Metadata_files {
//<parameter name="Metadata file" selected="true">
//    <current_file>/home/allardpm/git_repos/EMI/mzmine-batch-mode-sandbox/docs/dbgi_project_00002/cuso_batch_00001/metadata/treated/cuso_batch_00001_metadata.tsv</current_file>
//    <last_file>/home/allardpm/git_repos/DBGI/dbgi-metabolomics/docs/dbgi_project_00001/dbgi_batch_00001/metadata/treated/dbgi_batch_00001_metadata.tsv</last_file>
//    <last_file>/home/allardpm/git_repos/DBGI/dbgi-metabolomics/docs/dbgi_project_00001/dbgi_batch_00001/metadata/treated/dbgi_batch_00001_metadata_fully_resolved.csv</last_file>
//    <last_file>/home/allardpm/git_repos/mapp-metabolomics-unit/adria-leboeuf-group/docs/mapp_project_00014/mapp_batch_00074/metadata/treated/mapp_batch_00074_metadata.tsv</last_file>
//</parameter>
    selected: bool,
    files: Vec<String>,
}

impl Default for Metadata_files {
    fn default() -> Metadata_files {
        Metadata_files {
            selected: false,
            files: Vec::new(),
        }
    }
}

struct Spectral_library_files {
    selected: bool,
}

impl Default for Spectral_library_files {
    fn default() -> Spectral_library_files {
        Spectral_library_files {
            selected: false,
        }
    }
}


pub struct FileList{
    pub files: Vec<String>,
    pub number_of_files: usize,

    //parent parameter
    pub advanced_import: bool,

    //sub parameters
    pub scan_filter: Scan_filter,
    pub crop_MS1_mz: Crop_MS1_mz,
    pub msm1_detector_advanced: MS1_detector_advanced,
    pub msm2_detector_advanced: MS2_detector_advanced,
    pub denormalize_fragment_scans_traps: Denormalize_fragment_scans_traps,
    pub metadata_files: Metadata_files,
    pub spectral_library_files: Spectral_library_files,

}

impl Default for FileList {
    fn default() -> FileList {
        FileList {
            files: Vec::new(),
            number_of_files: 0,

            advanced_import: false, //parent paramenter of everything!

            scan_filter: Scan_filter::default(),
            crop_MS1_mz: Crop_MS1_mz::default(),
            msm1_detector_advanced: MS1_detector_advanced::default(),
            msm2_detector_advanced: MS2_detector_advanced::default(),
            denormalize_fragment_scans_traps: Denormalize_fragment_scans_traps::default(),

            metadata_files: Metadata_files::default(),

            spectral_library_files: Spectral_library_files::default(),
        }
    }
}

impl FileList{

    pub fn gen_XML_element(&self) -> XMLElement{
        let mut parent = XMLElement::new("batchstep");
        parent.add_attribute("method", "io.github.mzmine.modules.io.import_rawdata_all.AllSpectralDataImportModule");
        parent.add_attribute("parameter_version=", "1");        

        for file in self.files.iter(){
            let mut parameter = XMLElement::new("file");
            parameter.add_text(file);
            parent.add_child(parameter);
        }

        parent
    }
}




//<parameter name="Advanced import" selected="false">
//            <parameter name="Scan filters" selected="true">
//                <parameter name="Scan number"/>
//                <parameter name="Base Filtering Integer"/>
//                <parameter name="Retention time"/>
//                <parameter name="Mobility"/>
//                <parameter name="MS level filter" selected="All MS levels">1</parameter>
//                <parameter name="Scan definition"/>
//                <parameter name="Polarity">Any</parameter>
//                <parameter name="Spectrum type">ANY</parameter>
//            </parameter>
//            <parameter name="Crop MS1 m/z" selected="false"/>
//            <parameter name="MS1 detector (Advanced)" selected="true" selected_item="Factor of lowest signal">
//                <module name="Factor of lowest signal">
//                    <parameter name="Noise factor">5.0</parameter>
//                </module>
//                <module name="Auto">
//                    <parameter name="Noise level">1000.0</parameter>
//                </module>
//                <module name="Centroid">
//                    <parameter name="Noise level"/>
//                </module>
//                <module name="Exact mass">
//                    <parameter name="Noise level"/>
//                </module>
//                <module name="Local maxima">
//                    <parameter name="Noise level"/>
//                </module>
//                <module name="Recursive threshold">
//                    <parameter name="Noise level"/>
//                    <parameter name="Min m/z peak width"/>
//                    <parameter name="Max m/z peak width"/>
//                </module>
//                <module name="Wavelet transform">
//                    <parameter name="Noise level"/>
//                    <parameter name="Scale level"/>
//                    <parameter name="Wavelet window size (%)"/>
//                </module>
//            </parameter>
//            <parameter name="MS2 detector (Advanced)" selected="true" selected_item="Factor of lowest signal">
//                <module name="Factor of lowest signal">
//                    <parameter name="Noise factor">0.0</parameter>
//                </module>
//                <module name="Auto">
//                    <parameter name="Noise level">1000.0</parameter>
//                </module>
//                <module name="Centroid">
//                    <parameter name="Noise level"/>
//                </module>
//                <module name="Exact mass">
//                    <parameter name="Noise level"/>
//                </module>
//                <module name="Local maxima">
//                    <parameter name="Noise level"/>
//                </module>
//                <module name="Recursive threshold">
//                    <parameter name="Noise level"/>
//                    <parameter name="Min m/z peak width"/>
//                    <parameter name="Max m/z peak width"/>
//                </module>
//                <module name="Wavelet transform">
//                    <parameter name="Noise level"/>
//                    <parameter name="Scale level"/>
//                    <parameter name="Wavelet window size (%)"/>
//                </module>
//            </parameter>
//            <parameter name="Denormalize fragment scans (traps)">true</parameter>
//        </parameter>
//        <parameter name="Metadata file" selected="true">
//            <current_file>/home/allardpm/git_repos/EMI/mzmine-batch-mode-sandbox/docs/dbgi_project_00002/cuso_batch_00001/metadata/treated/cuso_batch_00001_metadata.tsv</current_file>
//            <last_file>/home/allardpm/git_repos/DBGI/dbgi-metabolomics/docs/dbgi_project_00001/dbgi_batch_00001/metadata/treated/dbgi_batch_00001_metadata.tsv</last_file>
//            <last_file>/home/allardpm/git_repos/DBGI/dbgi-metabolomics/docs/dbgi_project_00001/dbgi_batch_00001/metadata/treated/dbgi_batch_00001_metadata_fully_resolved.csv</last_file>
//            <last_file>/home/allardpm/git_repos/mapp-metabolomics-unit/adria-leboeuf-group/docs/mapp_project_00014/mapp_batch_00074/metadata/treated/mapp_batch_00074_metadata.tsv</last_file>
//        </parameter>
//        <parameter name="Spectral library files"/>
//</batchstep>
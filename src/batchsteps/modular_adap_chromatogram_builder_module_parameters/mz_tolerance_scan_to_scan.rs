use serde::{Serialize, Deserialize};

// <parameter name="m/z tolerance (scan-to-scan)">
    // <absolutetolerance>0.002</absolutetolerance>
    // <ppmtolerance>10.0</ppmtolerance>
// </parameter>

#[derive(Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
pub struct MzToleranceScanToScan{
    #[serde(rename="@name")]
    name: String,

    absolute_tolerance_value: AbsoluteTolerance,
    
    ppm_tolerance_value: PpmTolerance,
}

impl MzToleranceScanToScan{
    pub fn new() -> Self{
        MzToleranceScanToScan{
            name: "m/z tolerance (scan-to-scan)".to_owned(),
            absolute_tolerance_value: AbsoluteTolerance::new(),
            ppm_tolerance_value: PpmTolerance::new()
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn set_absolute_tolerance(&mut self, value: Option<f32>){
        self.absolute_tolerance_value.set_value(value);
    }

    pub fn get_absolute_tolerance(&self) -> &Option<f32>{
        self.absolute_tolerance_value.get_value()
    }

    pub fn set_ppm_tolerance(&mut self, value:Option<f32>){
        self.ppm_tolerance_value.set_value(value);
    }

    pub fn get_ppm_tolerance(&self) -> &Option<f32>{
        self.ppm_tolerance_value.get_value()
    }
}

#[derive(Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
pub struct AbsoluteTolerance{
    #[serde(rename = "$text")]
    value: Option<f32>,
}

impl AbsoluteTolerance {
    pub fn new() -> Self{
        AbsoluteTolerance{
            value: None,
        }
    }

    pub fn set_value(&mut self, value:Option<f32>){
        self.value = value;
    }

    pub fn get_value(&self) -> &Option<f32>{
        &self.value
    }
}

#[derive(Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
pub struct PpmTolerance{
    #[serde(rename = "$text")]
    value: Option<f32>,
}

impl PpmTolerance {
    pub fn new() -> Self{
        PpmTolerance{
            value: None,
        }
    }

    pub fn set_value(&mut self, value:Option<f32>){
        self.value = value;
    }

    pub fn get_value(&self) -> &Option<f32>{
        &self.value
    }
}
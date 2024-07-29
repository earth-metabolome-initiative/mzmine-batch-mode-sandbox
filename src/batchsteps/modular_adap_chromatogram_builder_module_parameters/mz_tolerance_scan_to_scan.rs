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

    pub fn set_absolute_tolerance(&mut self, value: Option<f32>){
        self.absolute_tolerance_value.set_value(value);
    }

    pub fn get_absolute_tolerance(&self) -> Option<f32>{
        self.absolute_tolerance_value.get_value()
    }

    pub fn set_ppm_tolerance(&mut self, value:Option<f32>){
        self.ppm_tolerance_value.set_value(value);
    }

    pub fn get_ppm_tolerance(&self) -> Option<f32>{
        self.ppm_tolerance_value.get_value()
    }
}

#[derive(Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
struct AbsoluteTolerance{
    #[serde(rename = "$text")]
    value: Option<f32>,
}

impl AbsoluteTolerance {
    fn new() -> Self{
        AbsoluteTolerance{
            value: None,
        }
    }
    fn set_value(&mut self, value:Option<f32>){
        self.value = value;
    }

    fn get_value(&self) -> Option<f32>{
        self.value
    }
}

#[derive(Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
struct PpmTolerance{
    #[serde(rename = "$text")]
    value: Option<f32>,
}

impl PpmTolerance {
    fn new() -> Self{
        PpmTolerance{
            value: None,
        }
    }
    fn set_value(&mut self, value:Option<f32>){
        self.value = value;
    }

    fn get_value(&self) -> Option<f32>{
        self.value
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_mz_initialization(){
        let mz_obj = MzToleranceScanToScan::new();
        assert_eq!(mz_obj.name, "m/z tolerance (scan-to-scan)");
        assert_eq!(mz_obj.absolute_tolerance_value.get_value(), None);
        assert_eq!(mz_obj.ppm_tolerance_value.get_value(), None)
    }

    #[test]
    fn test_mz_abs_tol_get_value(){
        let mut mz_obj = MzToleranceScanToScan::new();
        mz_obj.absolute_tolerance_value.value = Some(34.5);
        assert_eq!(mz_obj.absolute_tolerance_value.get_value(), Some(34.5));
    }

    #[test]
    fn test_mz_ppm_tol_get_value(){
        let mut mz_obj = MzToleranceScanToScan::new();
        mz_obj.ppm_tolerance_value.value = Some(25.5);
        assert_eq!(mz_obj.ppm_tolerance_value.get_value(), Some(25.5));
    }

    #[test]
    fn test_mz_abs_tol_set_value(){
        let mut mz_obj = MzToleranceScanToScan::new();
        mz_obj.absolute_tolerance_value.set_value(Some(12.6));
        assert_eq!(mz_obj.absolute_tolerance_value.value, Some(12.6));
    }

    #[test]
    fn test_mz_ppm_tol_set_value(){
        let mut mz_obj = MzToleranceScanToScan::new();
        mz_obj.ppm_tolerance_value.set_value(Some(11.6));
        assert_eq!(mz_obj.ppm_tolerance_value.value, Some(11.6));
    }
}
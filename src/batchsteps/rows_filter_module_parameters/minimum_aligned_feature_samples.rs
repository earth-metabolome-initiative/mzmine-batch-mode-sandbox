use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename = "parameter")]
pub struct MinimumAlignedFeaturesSamples{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "@selected")]
    selected: bool,

    #[serde(rename = "abs")]
    abs: AbsRel,

    #[serde(rename = "rel")]
    rel: AbsRel
}

impl MinimumAlignedFeaturesSamples{
    pub fn new() -> Self{
        MinimumAlignedFeaturesSamples{
            name: "Minimum aligned features (samples)".to_owned(),
            selected: false,
            abs: AbsRel::new(),
            rel: AbsRel::new(),
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn is_selected(&self) -> &bool{
        &self.selected
    }

    pub fn select(&mut self){
        self.selected = true;
    }
    
    pub fn deselect(&mut self){
        self.selected = false;
    }

    pub fn get_abs_value(&self) -> &Option<f32>{
        self.abs.get_value()
    }

    pub fn get_rel_value(&self) -> &Option<f32>{
        self.rel.get_value()
    }

    pub fn set_abs_value(&mut self, value:Option<f32>){
        self.abs.set_value(value);
    }

    pub fn set_rel_value(&mut self, value:Option<f32>){
        self.rel.set_value(value);
    }
}


#[derive(Default, Serialize, Deserialize, Clone, Debug, PartialEq)]
struct AbsRel{
    #[serde(rename = "$text")]
    value: Option<f32>,
}

impl AbsRel{
    pub fn new() -> Self{
        AbsRel{
            value: None
        }
    }

    pub fn get_value(&self) -> &Option<f32>{
        &self.value
    }

    pub fn set_value(&mut self, value: Option<f32>){
        self.value = value;
    }
}

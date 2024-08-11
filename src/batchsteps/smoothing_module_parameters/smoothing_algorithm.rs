use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
pub struct SmoothingAlgorithm{
    #[serde(rename="@name")]
    name: String,

    #[serde(rename="@type")]
    selected_item: String,

    modules: Vec<SmoothingAlgorithmModule>
}

impl SmoothingAlgorithm{
    pub fn new() -> Self{
        SmoothingAlgorithm{
            name: "Smoothing algorithm".to_owned(),
            selected_item: "".to_owned(),
            modules:Vec::new(),
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn set_selected_item(&mut self, item: String){
        self.selected_item = item;
    }

    pub fn get_selected_item(&self) -> &str{
        &self.selected_item
    }

    pub fn add_module(&mut self, module: SmoothingAlgorithmModule){
        self.modules.push(module);
    }

    pub fn get_modules_length(&self) -> usize{
        self.modules.len()
    }

    pub fn get_module(&self, target:&str) -> Option<&SmoothingAlgorithmModule>{
        for module in &self.modules{
            match module { 
                SmoothingAlgorithmModule::SavitzkyGolay(_f) if target == "Savitzky golay" => return Some(&module), 
                SmoothingAlgorithmModule::LoessSmoothing(_f) if target == "Loess smoothing" => return Some(&module),
                _ => continue
             }
        }
        None
    }
}

#[derive(Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum SmoothingAlgorithmModule{
    SavitzkyGolay(SavitzkyGolay),
    LoessSmoothing(LoessSmoothing),
}

impl SmoothingAlgorithmModule{
    pub fn get_name(&self) -> &str{
        match self{
            SmoothingAlgorithmModule::SavitzkyGolay(_f) => return _f.get_name(),
            SmoothingAlgorithmModule::LoessSmoothing(_f) => return _f.get_name(),
        }
    }
}

#[derive(Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
pub struct SavitzkyGolay{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "@selected")]
    selected: bool,

    parameters: Vec<SavitzkyGolayParameter>,
}

impl SavitzkyGolay{
    pub fn new() -> Self{
        SavitzkyGolay{
            name: "Savitzky Golay".to_owned(),
            selected: false,
            parameters: Vec::new(),
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn is_selected(&self) -> bool{
        self.selected
    }

    pub fn select(&mut self) {
        self.selected = true;
    }

    pub fn deselect(&mut self){
        self.selected=false;
    }    
    
    pub fn invert_selected(&mut self){
        self.selected = !self.selected;
    }

    pub fn get_parameters_length(&self) -> usize{
        self.parameters.len()
    }

    pub fn add_parameter(&mut self, parameter:SavitzkyGolayParameter){
        self.parameters.push(parameter);
    }

    pub fn get_parameter(&mut self, target:&str) -> Option<&SavitzkyGolayParameter>{
        for parameter in &mut self.parameters{
            match parameter{
                SavitzkyGolayParameter::RetentionTimeSmoothing(_) if target == "Retention time smoothing" => return Some(parameter),
                SavitzkyGolayParameter::MobilitySmoothing(_) if target == "Mobility smoothing" => return Some(parameter),
                _ => continue
            }
        }
        None
    }
}

#[derive(Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum SavitzkyGolayParameter{
    RetentionTimeSmoothing(RetentionTimeSmoothing),
    MobilitySmoothing(MobilitySmoothing)
}

impl SavitzkyGolayParameter{
    pub fn get_name(&self) -> &str{
        match self {
            SavitzkyGolayParameter::RetentionTimeSmoothing(_f) => return _f.get_name(),
            SavitzkyGolayParameter::MobilitySmoothing(_f) => return _f.get_name(),
        }
    }

    pub fn get_value(&self) -> &Option<f32>{
        match self {
            SavitzkyGolayParameter::RetentionTimeSmoothing(_f) => return _f.get_value(),
            SavitzkyGolayParameter::MobilitySmoothing(_f) => return _f.get_value(),
        }
    }
}

#[derive(Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
pub struct RetentionTimeSmoothing{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "@selected")]
    selected: bool,

    #[serde(rename = "$text")]
    value: Option<f32>,
}

impl RetentionTimeSmoothing{
    pub fn new() -> Self{
        RetentionTimeSmoothing{
            name: "Retention time smoothing".to_owned(),
            selected: false,
            value: None,
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn is_selected(&self) -> bool{
        self.selected
    }

    pub fn select(&mut self) {
        self.selected = true;
    }

    pub fn deselect(&mut self){
        self.selected=false;
    }    
    
    pub fn invert_selected(&mut self){
        self.selected = !self.selected;
    }

    pub fn get_value(&self) -> &Option<f32>{
        &self.value
    }

    pub fn set_value(&mut self, value:Option<f32>){
        self.value = value;
    }
}

#[derive(Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
pub struct MobilitySmoothing{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "@selected")]
    selected: bool,
    
    #[serde(rename = "$text")]
    value: Option<f32>
}

impl MobilitySmoothing{
    pub fn new() -> Self{
        MobilitySmoothing{
            name: "Mobility smoothing".to_owned(),
            selected: false,
            value: None,
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn is_selected(&self) -> bool{
        self.selected
    }

    pub fn select(&mut self) {
        self.selected = true;
    }

    pub fn deselect(&mut self){
        self.selected=false;
    }    
    
    pub fn invert_selected(&mut self){
        self.selected = !self.selected;
    }

    pub fn get_value(&self) -> &Option<f32>{
        &self.value
    }

    pub fn set_value(&mut self, value:Option<f32>){
        self.value = value;
    }
}

#[derive(Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
pub struct LoessSmoothing{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "@selected")]
    selected: bool,

    parameters: Vec<LoessSmoothingParameter>,
}

impl LoessSmoothing{
    pub fn new() -> Self{
        LoessSmoothing{
                name: "Loess smoothing".to_owned(),
                selected: false,
                parameters: Vec::new(),
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn is_selected(&self) -> bool{
        self.selected
    }

    pub fn select(&mut self) {
        self.selected = true;
    }

    pub fn deselect(&mut self){
        self.selected=false;
    }    
    
    pub fn invert_selected(&mut self){
        self.selected = !self.selected;
    }

    pub fn add_parameter(&mut self, parameter:LoessSmoothingParameter){
        self.parameters.push(parameter);
    }

    pub fn get_parameter(&mut self, target:&str) -> Option<&mut LoessSmoothingParameter>{
        for parameter in &mut self.parameters{
            match parameter{
                LoessSmoothingParameter::MobilityWidth(_f) if target == "Mobility width" => return Some(parameter),
                LoessSmoothingParameter::RetentionTimeWidth(_f) if target == "Retention time width" => return Some(parameter),
                _ => continue
            }
        }
        None
    }

    pub fn get_parameters_length(&self) -> usize{
        self.parameters.len()
    }

}

#[derive(Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum LoessSmoothingParameter{
    RetentionTimeWidth(RetentionTimeWidth),
    MobilityWidth(MobilityWidth),
}

impl LoessSmoothingParameter{
    pub fn get_name(&self) -> &str{
        match self{
            LoessSmoothingParameter::MobilityWidth(_f) => _f.get_name(),
            LoessSmoothingParameter::RetentionTimeWidth(_f) => _f.get_name(),
        }
    }

    pub fn get_value(&self) -> &Option<f32>{
        match self{
            LoessSmoothingParameter::MobilityWidth(_f) => _f.get_value(),
            LoessSmoothingParameter::RetentionTimeWidth(_f) => _f.get_value(),
        }
    }

    pub fn set_value(&mut self, value:Option<f32>) {
        match self{
            LoessSmoothingParameter::MobilityWidth(_f) => _f.set_value(value),
            LoessSmoothingParameter::RetentionTimeWidth(_f) => _f.set_value(value),
        }
    }
}

#[derive(Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
pub struct RetentionTimeWidth{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename="@selected")]
    selected: bool,

    #[serde(rename = "$text")]
    value: Option<f32>,
}

impl RetentionTimeWidth{
    pub fn new() -> Self {
        RetentionTimeWidth{
            name: "Retention time width (scans)".to_owned(),
            selected: true,
            value: None,
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn is_selected(&self) -> bool{
        self.selected
    }

    pub fn select(&mut self) {
        self.selected = true;
    }

    pub fn deselect(&mut self){
        self.selected=false;
    }    
    
    pub fn invert_selected(&mut self){
        self.selected = !self.selected;
    }

    pub fn get_value(&self) -> &Option<f32>{
        &self.value
    }

    pub fn set_value(&mut self, value:Option<f32>){
        self.value = value;
    }
}

#[derive(Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
pub struct MobilityWidth{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "@selected")]
    selected: bool,

    #[serde(rename = "$text")]
    value: Option<f32>,
}

impl MobilityWidth{
    pub fn new() -> Self{
        MobilityWidth{
            name: "Mobility width (scans)".to_owned(),
            selected: true,
            value: None,
        }
    }

    pub fn get_name(&self) -> &str{
        &self.name
    }

    pub fn is_selected(&self) -> bool{
        self.selected
    }

    pub fn select(&mut self) {
        self.selected = true;
    }

    pub fn deselect(&mut self){
        self.selected=false;
    }    
    
    pub fn invert_selected(&mut self){
        self.selected = !self.selected;
    }

    pub fn get_value(&self) -> &Option<f32>{
        &self.value
    }

    pub fn set_value(&mut self, value:Option<f32>){
        self.value = value;
    }
}
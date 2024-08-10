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

    pub fn set_selected_item(&mut self, item: String){
        self.selected_item = item;
    }

    pub fn get_selected_item(&self) -> String{
        self.selected_item.clone()
    }

    pub fn add_module(&mut self, module: SmoothingAlgorithmModule){
        self.modules.push(module);
    }

    // add setter and getters value for each module::parameter
}

#[derive(Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
enum SmoothingAlgorithmModule{
    SavitzkyGolay(SavitzkyGolay),
    LoessSmoothing(LoessSmoothing),
}

#[derive(Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
struct SavitzkyGolay{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "@selected")]
    selected: bool,

    parameters: Vec<SavitzkyGolayParameter>,
}

impl SavitzkyGolay{
    fn new() -> Self{
        SavitzkyGolay{
            name: "Savitzky Golay".to_owned(),
            selected: false,
            parameters: Vec::new(),
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
enum SavitzkyGolayParameter{
    RetentionTimeSmoothing(RetentionTimeSmoothing),
    MobilitySmoothing(MobilitySmoothing)
}

#[derive(Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
struct RetentionTimeSmoothing{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "@selected")]
    selected: bool,

    #[serde(rename = "$text")]
    value: Option<f32>,
}

impl RetentionTimeSmoothing{
    fn new() -> Self{
        RetentionTimeSmoothing{
            name: "Retention time smoothing".to_owned(),
            selected: false,
            value: None,
        }
    }
}

#[derive(Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
struct MobilitySmoothing{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "@selected")]
    selected: bool,
    
    #[serde(rename = "$text")]
    value: Option<f32>
}

impl MobilitySmoothing{
    fn new() -> Self{
        MobilitySmoothing{
            name: "Mobility smoothing".to_owned(),
            selected: false,
            value: None,
        }
    }
}

#[derive(Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
struct LoessSmoothing{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "@selected")]
    selected: bool,

    #[serde(rename = "$text")]
    value: Option<f32>
}

impl LoessSmoothing{
    fn new() -> Self{
        LoessSmoothing{
                name: "Loess smoothing".to_owned(),
                selected: true,
                value: None,
        }
    }
}

enum LoessSmoothingParameter{
    RetentionTimeWidth(RetentionTimeWidth),
    MobilityWidth(MobilityWidth),
}

#[derive(Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
struct RetentionTimeWidth{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename="@selected")]
    selected: bool,

    #[serde(rename = "$text")]
    value: Option<f32>,
}

impl RetentionTimeWidth{
    fn new() -> Self {
        RetentionTimeWidth{
            name: "Retention time width (scans)".to_owned(),
            selected: true,
            value: None,
        }
    }
}

#[derive(Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
struct MobilityWidth{
    #[serde(rename = "@name")]
    name: String,

    #[serde(rename = "@selected")]
    selected: bool,

    #[serde(rename = "$text")]
    value: Option<f32>,
}

impl MobilityWidth{
    fn new() -> Self{
        MobilityWidth{
            name: "".to_owned(),
            selected: true,
            value: None,
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_smoothing_algorithm_initialization(){
        let feature_lists_obj = SmoothingAlgorithm::new();
        assert_eq!(feature_lists_obj.name, "Smoothing algorithm");
        assert_eq!(feature_lists_obj.selected_item, "");
        assert_eq!(feature_lists_obj.modules.len(), 0);
    }
}
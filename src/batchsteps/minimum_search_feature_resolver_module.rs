use serde::{Serialize, Deserialize};

#[derive(Default, Serialize, Deserialize)]
pub struct MinimumSearchFeatureResolverModule{
    selected: bool,
}
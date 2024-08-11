use serde::{Serialize, Deserialize};
use crate::prelude::*;

#[derive(Default, Serialize, Deserialize, PartialEq)]
#[serde(default, rename_all = "lowercase")]
pub struct MinimumSearchFeatureResolverModule{
    #[serde(rename="@name")]
    method: String,
    
    #[serde(rename="@name")]
    parameter_version: u8,

    parameters: Vec<MinimumSearchFeatureResolverModuleParameters>
}


#[derive(Serialize, Deserialize, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum MinimumSearchFeatureResolverModuleParameters{
    ChromaticThreshold(ChromatographicThreshold),
    Dimension(Dimension),
    //FeatureList(FeatureList),
    //LimitByIonMobilityEdges(LimitByIonMobilityEdges),
    //MinRatioAbsolutePeakMinMobility(MinRatioAbsolutePeakMinMobility),
    //MinRatioOfPeakTopEdge(MinRatioOfPeakTopEdge),
    //MinimumRelativeHeight(MinimumRelativeHeight),
    //MinimumScansDataPoints(MinimumScansDataPoints),
    //MinimumSearchRangeRTMobilityAbsolute(MinimumSearchRangeRTMobilityAbsolute),
    //MsMsScanPairing(MsMsScanPairing),
    //OriginalFeatureList(OriginalFeatureList)
    //RetentionTimeFilter(RetentionTimeFilter)
    //Suffix(Suffix)

}
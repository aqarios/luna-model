use crate::core::{Model, Timing};

use super::{analysis_cache::AnalysisCache, base_passes::TransformationType};

pub struct IntermediateRepresentation {
    pub model: Model,
    pub cache: AnalysisCache,
    pub execution_log: Vec<(String, Timing, Option<TransformationType>)>
}



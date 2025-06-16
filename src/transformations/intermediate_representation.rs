use crate::core::{expression::{BiasConstraints, IndexConstraints}, Model, Timing};

use super::{analysis_cache::AnalysisCache, base_passes::TransformationType};

pub struct IntermediateRepresentation<Index: IndexConstraints, Bias: BiasConstraints> {
    pub model: Model<Index, Bias>,
    pub cache: AnalysisCache,
    pub execution_log: Vec<(String, Timing, Option<TransformationType>)>
}



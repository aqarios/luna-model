use lunamodel_core::prelude::Model;

use crate::{cache::AnalysisCache, log::ExecutionLog};

#[derive(Debug)]
pub struct IR {
    pub model: Model,
    pub cache: AnalysisCache,
    pub execution_log: ExecutionLog,
    pub input_model: Option<Model>,
}

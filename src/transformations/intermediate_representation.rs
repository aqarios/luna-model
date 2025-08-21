use crate::core::{Model, Timing};
use std::slice::Iter;

use super::{analysis_cache::AnalysisCache, base_passes::ActionType};
#[derive(Debug, Clone)]
pub struct LogElement {
    pub pass: String,
    pub timing: Timing,
    pub kind: ActionType,
    pub components: Option<ExecutionLog>,
}

impl LogElement {
    pub fn new(
        pass: String,
        timing: Timing,
        kind: ActionType,
        components: Option<ExecutionLog>,
    ) -> Self {
        Self {
            pass,
            timing,
            kind,
            components,
        }
    }
}

#[derive(Debug, Clone)]
pub struct ExecutionLog {
    log: Vec<LogElement>,
}

impl ExecutionLog {
    pub fn new() -> Self {
        Self { log: Vec::new() }
    }
    pub fn iter(&self) -> Iter<'_, LogElement> {
        self.log.iter()
    }

    pub fn push(
        &mut self,
        pass_name: String,
        timing: Timing,
        kind: ActionType,
        components: Option<ExecutionLog>,
    ) {
        self.log
            .push(LogElement::new(pass_name, timing, kind, components));
    }
}

#[derive(Debug)]
pub struct IntermediateRepresentation {
    pub model: Model,
    pub cache: AnalysisCache,
    pub execution_log: ExecutionLog,
    pub input_model: Option<Model>,
}

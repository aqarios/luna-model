use crate::core::{Model, Timing};
use std::slice::Iter;

use super::{analysis_cache::AnalysisCache, base_passes::ActionType};

pub struct LogElement {
    pub pass: String,
    pub timing: Timing,
    pub kind: ActionType,
}

impl LogElement {
    fn new(pass: String, timing: Timing, kind: ActionType) -> Self {
        Self { pass, timing, kind }
    }
}

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

    pub fn push(&mut self, pass_name: String, timing: Timing, kind: ActionType) {
        self.log.push(LogElement::new(pass_name, timing, kind));
    }
}

pub struct IntermediateRepresentation {
    pub model: Model,
    pub cache: AnalysisCache,
    pub execution_log: ExecutionLog,
}

use std::slice::Iter;

use lunamodel_core::prelude::Timing;

use super::base::ActionType;

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

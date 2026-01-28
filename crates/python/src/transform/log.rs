use lunamodel_transform::{ActionType, LogElement};
use pyo3::pyclass;

use crate::timer::PyTiming;

#[pyclass(get_all)]
#[derive(Clone)]
pub struct PyLogElement {
    pass_name: String,
    timing: PyTiming,
    kind: ActionType,
    components: Option<Vec<PyLogElement>>,
}

impl From<LogElement> for PyLogElement {
    fn from(e: LogElement) -> Self {
        Self {
            pass_name: e.pass,
            timing: e.timing.into(),
            kind: e.kind,
            components: e.components.map(|l| {
                let components: Vec<PyLogElement> = l.iter().map(|l| l.clone().into()).collect();
                components
            }),
        }
    }
}

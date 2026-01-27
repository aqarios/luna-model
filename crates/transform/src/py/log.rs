use lunamodel_python::PyTiming;
use pyo3::prelude::{pyclass, pymethods};

use crate::{base::ActionType, log::LogElement};

#[pyclass(get_all)]
#[derive(Clone)]
pub struct PyLogElement {
    pass_name: String,
    timing: PyTiming,
    kind: ActionType,
    components: Option<Vec<PyLogElement>>,
}
impl PyLogElement {
    pub fn new(elem: &LogElement) -> Self {
        Self {
            pass_name: elem.pass.clone(),
            timing: PyTiming(elem.timing),
            kind: elem.kind.clone(),
            components: elem
                .components
                .as_ref()
                .map(|x| x.iter().map(|e| PyLogElement::new(e)).collect()),
        }
    }
}

#[pymethods]
impl PyLogElement {
    fn __repr__(&self) -> String {
        format!(
            "Log(\"{}\", action={:?}, components={:?})",
            self.pass_name,
            self.kind,
            self.components.as_ref().map(|x| x.len())
        )
    }
}

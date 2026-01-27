use lunamodel_transform::ActionType;
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

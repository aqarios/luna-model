use strum_macros::Display;

#[cfg(feature = "py")]
use pyo3::prelude::pyclass;

#[derive(Display, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "py", pyclass(eq, eq_int, name = "PySolutionSource"))]
pub enum SolutionSource {
    Aws,
    Dimod,
    Qiskit,
    Numpy,
    Qctrl,
    Zib,
    Dict,
    DictList,
}

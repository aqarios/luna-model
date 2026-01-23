use crate::base::Pass;
use pyo3::PyResult;

pub trait PyPass {
    fn as_pass(self) -> PyResult<Pass>;
}

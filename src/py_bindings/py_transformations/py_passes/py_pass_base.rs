use crate::transformations::base_passes::Pass;
use pyo3::PyResult;

pub trait PyPass {
    fn as_pass(self) -> PyResult<Pass>;
}

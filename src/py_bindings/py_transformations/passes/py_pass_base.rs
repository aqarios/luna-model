
use pyo3::Python;

use crate::transformations::base_passes::Pass;

pub trait PyPass {
    fn as_pass(self) -> Pass;
}

pub trait PyPassPy {
    fn as_pass_py(self, py: Python) -> Pass;
}

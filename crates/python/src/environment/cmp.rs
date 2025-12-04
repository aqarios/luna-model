use lunamodel_core::prelude::ContentEquality;
use pyo3::pymethods;

use super::PyEnvironment;

#[pymethods]
impl PyEnvironment {
    fn equal_contents(&self, other: &Self) -> bool {
        self.env.is_equal_contents(&other.env)
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.env.id() == other.env.id() && self.equal_contents(other)
    }
}

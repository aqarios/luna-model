use pyo3::{PyResult, pymethods};

use super::PyConstraint;

#[pymethods]
impl PyConstraint {
    #[setter]
    fn set_name(&self, name: String) -> PyResult<()> {
        Ok(self.c.write_arc().set_name(name)?)
    }
}

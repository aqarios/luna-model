use pyo3::{Bound, FromPyObject, IntoPyObject, PyAny, PyErr};

use crate::py::{luna_model, utils::TypeCheck};

/// Marker corresponding to the public Python `Unbounded` sentinel type.
pub struct PyUnbounded;

impl<'a, 'py> FromPyObject<'a, 'py> for PyUnbounded {
    type Error = PyErr;

    fn extract(obj: pyo3::Borrowed<'a, 'py, PyAny>) -> Result<Self, Self::Error> {
        obj.check_type_literal("Unbounded")?;
        Ok(Self)
    }
}

impl<'py> IntoPyObject<'py> for PyUnbounded {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: pyo3::Python<'py>) -> Result<Self::Output, Self::Error> {
        use pyo3::types::PyAnyMethods;
        luna_model(py)?.getattr("Unbounded")
    }
}

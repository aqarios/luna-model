use lunamodel_core::Solution;
use lunamodel_serializer::prelude::*;
use lunamodel_unwind::*;
use pyo3::{
    Bound, Py, PyAny, PyResult, Python, pymethods,
    types::{PyBytes, PyType},
};

use super::PySolution;

#[unwindable]
#[pymethods]
impl PySolution {
    #[pyo3(signature=(compress=true, level=3))]
    fn encode(
        &self,
        py: Python,
        compress: Option<bool>,
        level: Option<i32>,
    ) -> PyResult<Py<PyAny>> {
        let bytes = self.s.read_arc().encode(compress, level)?;
        Ok(PyBytes::new(py, bytes.as_slice()).into())
    }

    /// Alias for `encode()`.
    ///
    /// See `encode()` for full documentation.
    #[pyo3(signature=(compress=true, level=3))]
    fn serialize(
        &self,
        py: Python,
        compress: Option<bool>,
        level: Option<i32>,
    ) -> PyResult<Py<PyAny>> {
        self.encode(py, compress, level)
    }

    #[classmethod]
    fn decode(_cls: &Bound<'_, PyType>, py: Python, data: Py<PyBytes>) -> PyResult<Self> {
        let sol: Solution = data.as_bytes(py).unversionize().decompress()?.decode(())?;
        Ok(sol.into())
    }

    /// Alias for `decode()`.
    ///
    /// See `decode()` for full documentation.
    #[classmethod]
    fn deserialize(cls: &Bound<'_, PyType>, py: Python, data: Py<PyBytes>) -> PyResult<Self> {
        Self::decode(cls, py, data)
    }
}

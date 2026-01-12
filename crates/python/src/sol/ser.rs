use lunamodel_core::Solution;
use pyo3::{
    Bound, Py, PyAny, PyResult, Python, pymethods,
    types::{PyBytes, PyType},
};

use lunamodel_serializer::prelude::*;

use super::PySolution;

#[pymethods]
impl PySolution {
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

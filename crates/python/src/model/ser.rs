use lunamodel_core::Model;
use lunamodel_serializer::prelude::{Decodable, Decompressable, Encodable, Unversionizable};
use lunamodel_unwind::*;
use pyo3::{
    Bound, Py, PyAny, PyResult, Python, pymethods,
    types::{PyBytes, PyType},
};

use super::PyModel;

#[unwindable]
#[pymethods]
impl PyModel {
    /// Serialize the model into a compact binary format.
    ///
    /// Parameters
    /// ----------
    /// compress : bool, optional
    ///     Whether to compress the binary output. Default is True.
    /// level : int, optional
    ///     Compression level (0–9). Default is 3.
    ///
    /// Returns
    /// -------
    /// bytes
    ///     Encoded model representation.
    ///
    /// Raises
    /// ------
    /// IOError
    ///     If serialization fails.
    #[pyo3(signature=(compress=true, level=3))]
    fn encode(
        &self,
        py: Python,
        compress: Option<bool>,
        level: Option<i32>,
    ) -> PyResult<Py<PyAny>> {
        Ok(PyBytes::new(py, &self.m.read_arc().encode(compress, level)?).into())
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

    /// Reconstruct a model from encoded bytes.
    ///
    /// Parameters
    /// ----------
    /// data : bytes
    ///     Binary blob returned by `encode()`.
    ///
    /// Returns
    /// -------
    /// Model
    ///     Deserialized model object.
    ///
    /// Raises
    /// ------
    /// DecodeError
    ///     If decoding fails due to corruption or incompatibility.
    #[classmethod]
    fn decode(_cls: &Bound<'_, PyType>, py: Python, data: Py<PyBytes>) -> PyResult<Self> {
        let model: Model = data.as_bytes(py).unversionize().decompress()?.decode(())?;
        Ok(model.into())
    }

    /// Alias for `decode()`.
    ///
    /// See `decode()` for full documentation.
    #[classmethod]
    fn deserialize(cls: &Bound<'_, PyType>, py: Python, data: Py<PyBytes>) -> PyResult<Self> {
        Self::decode(cls, py, data)
    }
}

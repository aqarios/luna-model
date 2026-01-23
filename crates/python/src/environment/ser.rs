use lunamodel_core::Environment;
use lunamodel_serializer::prelude::{Decodable, Decompressable, Encodable, Unversionizable};
use lunamodel_unwind::*;
use pyo3::{
    prelude::*,
    types::{PyBytes, PyType},
};

use crate::PyEnvironment;

#[unwindable]
#[pymethods]
impl PyEnvironment {
    /// Serialize the environment into a compact binary format.
    ///
    /// This is the preferred method for persisting an environment's state.
    ///
    /// Parameters
    /// ----------
    /// compress : bool, optional
    ///     Whether to compress the binary output. Default is `True`.
    /// level : int, optional
    ///     Compression level (e.g., from 0 to 9). Default is `3`.
    ///
    /// Returns
    /// -------
    /// bytes
    ///     Encoded binary representation of the environment.
    ///
    /// Raises
    /// ------
    /// IOError
    ///     If serialization fails.
    #[pyo3(signature=(compress=true, level=3))]
    pub fn encode(
        &self,
        py: Python,
        compress: Option<bool>,
        level: Option<i32>,
    ) -> PyResult<Py<PyAny>> {
        Ok(PyBytes::new(py, &self.env.read_arc().encode(compress, level)?).into())
    }

    /// Alias for `encode()`.
    ///
    /// See `encode()` for full usage details.
    #[pyo3(signature=(compress=true, level=3))]
    fn serialize(
        &self,
        py: Python,
        compress: Option<bool>,
        level: Option<i32>,
    ) -> PyResult<Py<PyAny>> {
        self.encode(py, compress, level)
    }

    /// Reconstruct an expression from a previously encoded binary blob.
    ///
    /// Parameters
    /// ----------
    /// data : bytes
    ///     The binary data returned from `Environment.encode()`.
    ///
    /// Returns
    /// -------
    /// Expression
    ///     The reconstructed symbolic expression.
    ///
    /// Raises
    /// ------
    /// DecodeError
    ///     If decoding fails due to corruption or incompatibility.
    #[classmethod]
    pub fn decode(_cls: &Bound<'_, PyType>, py: Python, data: Py<PyBytes>) -> PyResult<Self> {
        let env: Environment = data.as_bytes(py).unversionize().decompress()?.decode(())?;
        Ok(env.into())
    }

    /// Alias for `decode()`.
    ///
    /// See `decode()` for full usage details.
    #[classmethod]
    fn deserialize(cls: &Bound<'_, PyType>, py: Python, data: Py<PyBytes>) -> PyResult<Self> {
        Self::decode(cls, py, data)
    }
}

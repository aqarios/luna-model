use lunamodel_core::{ArcEnv, Environment};
use lunamodel_serializer::prelude::{Decodable, Decompressable, Unversionizable};
use lunamodel_unwind::unwindable;
use pyo3::{
    Bound, FromPyObject, Py, PyAny, PyResult, Python, pymethods,
    types::{PyBytes, PyType},
};

use super::PyConstraintCollection;
use crate::PyEnvironment;
use crate::unwind::unwind;

#[derive(FromPyObject)]
enum DecodeEnvData {
    Env(PyEnvironment),
    Bytes(Py<PyBytes>),
}

#[unwindable]
#[pymethods]
impl PyConstraintCollection {
    /// Serialize the constraint collection to a binary blob.
    ///
    /// Parameters
    /// ----------
    /// compress : bool, optional
    ///     Whether to compress the result. Default is True.
    /// level : int, optional
    ///     Compression level (0–9). Default is 3.
    ///
    /// Returns
    /// -------
    /// bytes
    ///     Encoded representation of the constraints.
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
        let bytes = self.c.encode(compress, level)?;
        Ok(PyBytes::new(py, bytes.as_slice()).into())
    }

    /// Alias for `encode()`.
    ///
    /// See `encode()` for details.
    #[pyo3(signature=(compress=true, level=3))]
    fn serialize(
        &self,
        py: Python,
        compress: Option<bool>,
        level: Option<i32>,
    ) -> PyResult<Py<PyAny>> {
        self.encode(py, compress, level)
    }

    /// Deserialize an expression from binary constraint data.
    ///
    /// Parameters
    /// ----------
    /// data : bytes
    ///     Encoded blob from `encode()`.
    ///
    /// Returns
    /// -------
    /// Expression
    ///     Expression reconstructed from the constraint context.
    ///
    /// Raises
    /// ------
    /// DecodeError
    ///     If decoding fails due to corruption or incompatibility.
    #[classmethod]
    fn decode(
        _cls: &Bound<'_, PyType>,
        py: Python,
        data: Py<PyBytes>,
        env: DecodeEnvData,
    ) -> PyResult<Self> {
        let env: ArcEnv = match env {
            DecodeEnvData::Env(pyenv) => pyenv.env,
            DecodeEnvData::Bytes(pybytes) => {
                let env: Environment = pybytes
                    .as_bytes(py)
                    .unversionize()
                    .decompress()?
                    .decode(())?;
                ArcEnv::from(env)
            }
        };
        Ok(PyConstraintCollection::new(
            data.as_bytes(py).unversionize().decompress()?.decode(env)?,
        ))
    }

    /// Alias for `decode()`.
    ///
    /// See `decode()` for usage.
    #[classmethod]
    fn deserialize(
        cls: &Bound<'_, PyType>,
        py: Python,
        data: Py<PyBytes>,
        env: DecodeEnvData,
    ) -> PyResult<Self> {
        Self::decode(cls, py, data, env)
    }
}

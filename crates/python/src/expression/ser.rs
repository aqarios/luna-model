use lunamodel_serializer::prelude::*;
use pyo3::{
    prelude::*,
    types::{PyBytes, PyType},
};

use super::content::PyExprContent as PyEC;
use crate::{PyEnvironment, PyExpression};

#[pymethods]
impl PyExpression {
    /// Serialize the expression into a compact binary format.
    ///
    /// Parameters
    /// ----------
    /// compress : bool, optional
    ///     Whether to compress the data. Default is True.
    /// level : int, optional
    ///     Compression level (0–9). Default is 3.
    ///
    /// Returns
    /// -------
    /// bytes
    ///     Encoded representation of the expression.
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
        let bytes = match &self.expr {
            PyEC::Expr(expr) => expr.read_arc().encode(compress, level),
            PyEC::Model(parent) => &parent.read_arc().objective.encode(compress, level),
        }?;
        Ok(PyBytes::new(py, bytes).into())
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

    /// Reconstruct an expression from encoded bytes.
    ///
    /// Parameters
    /// ----------
    /// data : bytes
    ///     Binary blob returned by `encode()`.
    ///
    /// Returns
    /// -------
    /// Expression
    ///     Deserialized expression object.
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
        env: PyEnvironment,
    ) -> PyResult<Self> {
        Ok(PyExpression::new(
            data.as_bytes(py)
                .unversionize()
                .decompress()?
                .decode(env.env)?,
        ))
    }

    /// Alias for `decode()`.
    ///
    /// See `decode()` for full documentation.
    #[classmethod]
    fn deserialize(
        cls: &Bound<'_, PyType>,
        py: Python,
        data: Py<PyBytes>,
        env: PyEnvironment,
    ) -> PyResult<Self> {
        Self::decode(cls, py, data, env)
    }
}

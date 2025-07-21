use super::py_utilities::unwind;
use unwind_macros::unwindable;
use crate::{
    core::{environment::SharedEnvironment, ContentEquality},
    serialization::{
        Compressable, Decodable, Decompressable, Encodable, Unversionizable, Versionizable,
    },
};
use derive_more::{Deref, DerefMut};
use pyo3::{
    prelude::*,
    types::{PyBytes, PyType},
};
use std::{cell::RefCell, ops::Deref, rc::Rc};

use super::{py_exceptions::MultipleActiveEnvironmentsError, py_var::PyVariable};

/// Execution context for variable creation and expression scoping.
///
/// An `Environment` provides the symbolic scope in which `Variable` objects are defined.
/// It is required for variable construction, and ensures consistency across expressions.
/// The environment does **not** store constraints or expressions — it only facilitates
/// their creation by acting as a context manager and anchor for `Variable` instances.
///
/// Environments are best used with `with` blocks, but can also be passed manually
/// to models or variables.
///
/// Examples
/// --------
/// Create variables inside an environment:
///
/// >>> from luna_quantum import Environment, Variable
/// >>> with Environment() as env:
/// ...     x = Variable("x")
/// ...     y = Variable("y")
///
/// Serialize the environment state:
///
/// >>> data = env.encode()
/// >>> expr = Environment.decode(data)
///
/// Notes
/// -----
/// - The environment is required to create `Variable` instances.
/// - It does **not** own constraints or expressions — they merely reference variables tied to an environment.
/// - Environments **cannot be nested**. Only one can be active at a time.
/// - Use `encode()` / `decode()` to persist and recover expression trees.
#[cfg_attr(
    not(feature = "lq"),
    pyclass(unsendable, name = "Environment", module = "aqmodels._core")
)]
#[cfg_attr(
    feature = "lq",
    pyclass(unsendable, name = "Environment", module = "luna_quantum._core")
)]
#[derive(Deref, DerefMut, Clone)]
pub struct PyEnvironment(pub SharedEnvironment);

impl PyEnvironment {
    pub fn new(env: SharedEnvironment) -> Self {
        Self(env)
    }
}

thread_local! {
    pub static CURRENT_ENV: RefCell<Option<PyEnvironment>> = RefCell::new(None);
}

#[unwindable]
#[pymethods]
impl PyEnvironment {
    /// Initialize a new environment for variable construction.
    ///
    /// It is recommended to use this in a `with` statement to ensure proper scoping.
    #[new]
    fn py_new() -> PyResult<Self> {
        Ok(PyEnvironment::new(SharedEnvironment::default()))
    }

    /// Activate this environment for variable creation.
    ///
    /// Returns
    /// -------
    /// Environment
    ///     The current environment (self).
    ///
    /// Raises
    /// ------
    /// MultipleActiveEnvironmentsError
    ///     If another environment is already active.
    fn __enter__(&self) -> PyResult<Self> {
        CURRENT_ENV.with(|current| {
            let mut mut_curr = current.borrow_mut();
            if mut_curr.is_some() {
                return Err(MultipleActiveEnvironmentsError::new_err(
                    "multiple active environments are not allowed",
                ));
            }
            *mut_curr = Some(self.clone());
            Ok(())
        })?;
        Ok(self.clone())
    }

    /// Deactivate this environment.
    ///
    /// Called automatically at the end of a `with` block.
    fn __exit__(
        &self,
        _exc_type: &Bound<'_, PyAny>,
        _exc_value: &Bound<'_, PyAny>,
        _traceback: &Bound<'_, PyAny>,
    ) -> PyResult<()> {
        CURRENT_ENV.with(|current| {
            *current.borrow_mut() = None;
        });
        Ok(())
    }

    /// Get a variable by its label (name).
    ///
    /// Parameters
    /// ----------
    /// label : str
    ///     The name/label of the variable
    ///
    /// Returns
    /// -------
    /// Variable
    ///     The variable with the specified label/name.
    ///
    /// Raises
    /// ------
    /// VariableNotExistingError
    ///     If no variable with the specified name is registered.
    fn get_variable(&self, name: String) -> PyResult<PyVariable> {
        Ok(PyVariable(Rc::new(self.0.get_vref_by_name(&name)?)))
    }

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
    fn encode(&self, py: Python, compress: Option<bool>, level: Option<i32>) -> PyResult<PyObject> {
        let compress = compress.unwrap_or(level.is_some());
        Ok(PyBytes::new(
            py,
            &self
                .borrow()
                .deref()
                .encode()
                .maybe_compress(compress, level)?
                .versionize(),
        )
        .into())
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
    ) -> PyResult<PyObject> {
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
    fn decode(_cls: &Bound<'_, PyType>, py: Python, data: Py<PyBytes>) -> PyResult<Self> {
        Ok(PyEnvironment::new(SharedEnvironment::from(
            data.as_bytes(py).unversionize().decompress()?.decode(())?,
        )))
    }

    /// Alias for `decode()`.
    ///
    /// See `decode()` for full usage details.
    #[classmethod]
    fn deserialize(cls: &Bound<'_, PyType>, py: Python, data: Py<PyBytes>) -> PyResult<Self> {
        Self::decode(cls, py, data)
    }

    fn __eq__(&self, other: &PyEnvironment) -> bool {
        *self.borrow() == *other.borrow()
    }

    fn __str__(&self) -> String {
        self.borrow().to_string()
    }

    fn __repr__(&self) -> String {
        format!("{:#?}", self.borrow())
    }

    fn equal_contents(&self, other: &Self) -> bool {
        self.0.is_equal_contents(&other.0)
    }

    fn __contains__(&self, varname: String) -> bool {
        self.0.contains(varname)
    }

}

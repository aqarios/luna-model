//! Python wrapper for solution value-source enums.

// pyo3's `from_py_object` derive emits `.clone()` on these `Copy` types.
#![allow(clippy::clone_on_copy)]

use lunamodel_types::ValueSource;
use pyo3::pyclass;
use strum_macros::Display;

/// Python-facing wrapper for [`ValueSource`].
#[pyclass(from_py_object, eq, eq_int)]
#[derive(Debug, Clone, Copy, Display, Hash, PartialEq)]
pub enum PyValueSource {
    Raw,
    Obj,
}

impl From<ValueSource> for PyValueSource {
    /// Converts the Rust value-source enum into its Python wrapper.
    fn from(value: ValueSource) -> Self {
        match value {
            ValueSource::Raw => PyValueSource::Raw,
            ValueSource::Obj => PyValueSource::Obj,
        }
    }
}

impl From<PyValueSource> for ValueSource {
    /// Converts the Python value-source wrapper back into the core enum.
    fn from(val: PyValueSource) -> Self {
        match val {
            PyValueSource::Raw => ValueSource::Raw,
            PyValueSource::Obj => ValueSource::Obj,
        }
    }
}

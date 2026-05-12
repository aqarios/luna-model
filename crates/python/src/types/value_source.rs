//! Python wrapper for solution value-source enums.

use lunamodel_core::ValueSource;
use pyo3::pyclass;
use strum_macros::Display;

/// Python-facing wrapper for [`ValueSource`].
#[pyclass(eq, eq_int)]
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

use lunamodel_core::ValueSource;
use pyo3::pyclass;

/// Python-facing wrapper for [`ValueSource`].
#[derive(Debug, Clone)]
#[pyclass]
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

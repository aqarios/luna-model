//! Python wrapper for model translation targets.

use lunamodel_translate::TranslationTarget;
use pyo3::pyclass;
use strum_macros::Display;

/// Python-facing wrapper for [`TranslationTarget`].
#[pyclass(from_py_object, eq, eq_int)]
#[derive(Debug, Clone, Copy, Display, Hash, PartialEq)]
pub enum PyTranslationTarget {
    Qubo,
    Lp,
    Mps,
    Bqm,
    Cqm,
    OptMapper,
}

impl From<TranslationTarget> for PyTranslationTarget {
    /// Converts the Rust translation-target enum into its Python wrapper.
    fn from(value: TranslationTarget) -> Self {
        match value {
            TranslationTarget::Qubo => Self::Qubo,
            TranslationTarget::Lp => Self::Lp,
            TranslationTarget::Mps => Self::Mps,
            TranslationTarget::Bqm => Self::Bqm,
            TranslationTarget::Cqm => Self::Cqm,
            TranslationTarget::OptMapper => Self::OptMapper,
        }
    }
}

impl From<PyTranslationTarget> for TranslationTarget {
    /// Converts the Python translation-target wrapper back into the core enum.
    fn from(val: PyTranslationTarget) -> Self {
        match val {
            PyTranslationTarget::Qubo => TranslationTarget::Qubo,
            PyTranslationTarget::Lp => TranslationTarget::Lp,
            PyTranslationTarget::Mps => TranslationTarget::Mps,
            PyTranslationTarget::Bqm => TranslationTarget::Bqm,
            PyTranslationTarget::Cqm => TranslationTarget::Cqm,
            PyTranslationTarget::OptMapper => TranslationTarget::OptMapper,
        }
    }
}

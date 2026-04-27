use lunamodel_translate::TranslationTarget;
use pyo3::pyclass;
use strum_macros::Display;

#[pyclass(eq, eq_int)]
#[derive(Debug, Display, Hash, PartialEq)]
pub enum PyTranslationTarget {
    Qubo,
    Lp,
    Mps,
    Bqm,
    Cqm,
}

impl From<TranslationTarget> for PyTranslationTarget {
    fn from(value: TranslationTarget) -> Self {
        match value {
            TranslationTarget::Qubo => Self::Qubo,
            TranslationTarget::Lp => Self::Lp,
            TranslationTarget::Mps => Self::Mps,
            TranslationTarget::Bqm => Self::Bqm,
            TranslationTarget::Cqm => Self::Cqm,
        }
    }
}

impl From<PyTranslationTarget> for TranslationTarget {
    fn from(val: PyTranslationTarget) -> Self {
        match val {
            PyTranslationTarget::Qubo => TranslationTarget::Qubo,
            PyTranslationTarget::Lp => TranslationTarget::Lp,
            PyTranslationTarget::Mps => TranslationTarget::Mps,
            PyTranslationTarget::Bqm => TranslationTarget::Bqm,
            PyTranslationTarget::Cqm => TranslationTarget::Cqm,
        }
    }
}

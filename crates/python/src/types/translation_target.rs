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

impl Into<TranslationTarget> for PyTranslationTarget {
    fn into(self) -> TranslationTarget {
        match self {
            Self::Qubo => TranslationTarget::Qubo,
            Self::Lp => TranslationTarget::Lp,
            Self::Mps => TranslationTarget::Mps,
            Self::Bqm => TranslationTarget::Bqm,
            Self::Cqm => TranslationTarget::Cqm,
        }
    }
}

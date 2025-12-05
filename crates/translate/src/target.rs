use strum_macros::Display;

#[cfg(feature = "py")]
use pyo3::prelude::pyclass;

#[derive(Debug, Display, Hash, PartialEq)]
#[cfg_attr(feature = "py", pyclass(eq, eq_int, name = "PyTranslationTarget"))]
pub enum TranslationTarget {
    Qubo,
    Lp,
    Bqm,
    Cqm,
}

//! Python wrapper for constraint comparator/type enums.

use lunamodel_types::Ctype;
use pyo3::pyclass;

/// Python-facing wrapper for [`Ctype`].
#[derive(Copy, PartialEq, Hash, Clone, Debug, Eq)]
#[pyclass(eq, eq_int, name = "PyCtype")]
pub enum PyCtype {
    Unconstrained,
    Equality,
    Inequality,
    LessEqual,
    GreaterEqual,
}

impl From<Ctype> for PyCtype {
    /// Converts the Rust constraint-type enum into its Python wrapper.
    fn from(value: Ctype) -> Self {
        match value {
            Ctype::Unconstrained => PyCtype::Unconstrained,
            Ctype::Equality => PyCtype::Equality,
            Ctype::Inequality => PyCtype::Inequality,
            Ctype::LessEqual => PyCtype::LessEqual,
            Ctype::GreaterEqual => PyCtype::GreaterEqual,
        }
    }
}

impl From<PyCtype> for Ctype {
    /// Converts the Python constraint-type wrapper back into the core enum.
    fn from(val: PyCtype) -> Self {
        match val {
            PyCtype::Unconstrained => Ctype::Unconstrained,
            PyCtype::Equality => Ctype::Equality,
            PyCtype::Inequality => Ctype::Inequality,
            PyCtype::LessEqual => Ctype::LessEqual,
            PyCtype::GreaterEqual => Ctype::GreaterEqual,
        }
    }
}

//! Python wrapper for variable-type enums.

use lunamodel_types::Vtype;
use pyo3::pyclass;

/// Python-facing wrapper for [`Vtype`].
#[pyclass(from_py_object, eq, eq_int, name = "PyVtype")]
#[derive(Eq, PartialEq, Clone, Copy)]
pub enum PyVtype {
    Binary,
    InvertedBinary,
    Spin,
    Integer,
    Real,
}

impl From<Vtype> for PyVtype {
    /// Converts the Rust variable-type enum into its Python wrapper.
    fn from(value: Vtype) -> Self {
        match value {
            Vtype::Binary => Self::Binary,
            Vtype::InvertedBinary => Self::InvertedBinary,
            Vtype::Spin => Self::Spin,
            Vtype::Integer => Self::Integer,
            Vtype::Real => Self::Real,
        }
    }
}

impl From<PyVtype> for Vtype {
    /// Converts the Python variable-type wrapper back into the core enum.
    fn from(val: PyVtype) -> Self {
        match val {
            PyVtype::Binary => Vtype::Binary,
            PyVtype::InvertedBinary => Vtype::InvertedBinary,
            PyVtype::Spin => Vtype::Spin,
            PyVtype::Integer => Vtype::Integer,
            PyVtype::Real => Vtype::Real,
        }
    }
}

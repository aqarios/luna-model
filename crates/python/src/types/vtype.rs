use lunamodel_types::Vtype;
use pyo3::pyclass;

#[pyclass(eq, eq_int, name = "PyVtype")]
#[derive(Eq, PartialEq, Clone, Copy)]
pub enum PyVtype {
    Binary,
    InvertedBinary,
    Spin,
    Integer,
    Real,
}

impl From<Vtype> for PyVtype {
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

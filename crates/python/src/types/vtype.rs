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

impl Into<Vtype> for PyVtype {
    fn into(self) -> Vtype {
        match self {
            Self::Binary => Vtype::Binary,
            Self::InvertedBinary => Vtype::InvertedBinary,
            Self::Spin => Vtype::Spin,
            Self::Integer => Vtype::Integer,
            Self::Real => Vtype::Real,
        }
    }
}

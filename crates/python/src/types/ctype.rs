use lunamodel_types::Ctype;
use pyo3::pyclass;

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

impl Into<Ctype> for PyCtype {
    fn into(self) -> Ctype {
        match self {
            PyCtype::Unconstrained => Ctype::Unconstrained,
            PyCtype::Equality => Ctype::Equality,
            PyCtype::Inequality => Ctype::Inequality,
            PyCtype::LessEqual => Ctype::LessEqual,
            PyCtype::GreaterEqual => Ctype::GreaterEqual,
        }
    }
}

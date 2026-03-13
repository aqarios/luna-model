use lunamodel_types::Comparator;
use pyo3::pyclass;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[pyclass(eq, eq_int, name = "PyComparator")]
pub enum PyComparator {
    Eq,
    Le,
    Ge,
}

impl From<Comparator> for PyComparator {
    fn from(value: Comparator) -> Self {
        match value {
            Comparator::Eq => PyComparator::Eq,
            Comparator::Le => PyComparator::Le,
            Comparator::Ge => PyComparator::Ge,
        }
    }
}

impl Into<Comparator> for PyComparator {
    fn into(self) -> Comparator {
        match self {
            PyComparator::Eq => Comparator::Eq,
            PyComparator::Le => Comparator::Le,
            PyComparator::Ge => Comparator::Ge,
        }
    }
}


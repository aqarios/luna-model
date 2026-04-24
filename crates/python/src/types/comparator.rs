use lunamodel_types::Comparator;
use pyo3::pyclass;

/// Python-facing wrapper for [`Comparator`].
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[pyclass(eq, eq_int, name = "PyComparator")]
pub enum PyComparator {
    Eq,
    Le,
    Ge,
}

impl From<Comparator> for PyComparator {
    /// Converts the Rust comparator enum into its Python wrapper.
    fn from(value: Comparator) -> Self {
        match value {
            Comparator::Eq => PyComparator::Eq,
            Comparator::Le => PyComparator::Le,
            Comparator::Ge => PyComparator::Ge,
        }
    }
}

impl From<PyComparator> for Comparator {
    /// Converts the Python comparator wrapper back into the core enum.
    fn from(val: PyComparator) -> Self {
        match val {
            PyComparator::Eq => Comparator::Eq,
            PyComparator::Le => Comparator::Le,
            PyComparator::Ge => Comparator::Ge,
        }
    }
}

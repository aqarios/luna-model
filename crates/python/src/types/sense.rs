//! Python wrapper for objective-sense enums.

use lunamodel_types::Sense;
use pyo3::pyclass;

/// Python-facing wrapper for [`Sense`].
#[derive(Copy, PartialEq, Hash, Clone, Debug, Eq)]
#[pyclass(from_py_object, eq, eq_int, name = "PySense")]
pub enum PySense {
    Min,
    Max,
}

impl From<Sense> for PySense {
    /// Converts the Rust sense enum into its Python wrapper.
    fn from(value: Sense) -> Self {
        match value {
            Sense::Min => PySense::Min,
            Sense::Max => PySense::Max,
        }
    }
}

impl From<PySense> for Sense {
    /// Converts the Python sense wrapper back into the core enum.
    fn from(val: PySense) -> Self {
        match val {
            PySense::Min => Sense::Min,
            PySense::Max => Sense::Max,
        }
    }
}

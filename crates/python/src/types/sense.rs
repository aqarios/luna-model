use lunamodel_types::Sense;
use pyo3::pyclass;

#[derive(Copy, PartialEq, Hash, Clone, Debug, Eq)]
#[pyclass(eq, eq_int, name = "PySense")]
pub enum PySense {
    Min,
    Max,
}

impl From<Sense> for PySense {
    fn from(value: Sense) -> Self {
        match value {
            Sense::Min => PySense::Min,
            Sense::Max => PySense::Max,
        }
    }
}

impl From<PySense> for Sense {
    fn from(val: PySense) -> Self {
        match val {
            PySense::Min => Sense::Min,
            PySense::Max => Sense::Max,
        }
    }
}

use lunamodel_core::ValueSource;
use pyo3::pyclass;

#[derive(Debug, Clone)]
#[pyclass]
pub enum PyValueSource {
    Raw,
    Obj,
}

impl From<ValueSource> for PyValueSource {
    fn from(value: ValueSource) -> Self {
        match value {
            ValueSource::Raw => PyValueSource::Raw,
            ValueSource::Obj => PyValueSource::Obj,
        }
    }
}

impl Into<ValueSource> for PyValueSource {
    fn into(self) -> ValueSource {
        match self {
            PyValueSource::Raw => ValueSource::Raw,
            PyValueSource::Obj => ValueSource::Obj,
        }
    }
}

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

impl From<PyValueSource> for ValueSource {
    fn from(val: PyValueSource) -> Self {
        match val {
            PyValueSource::Raw => ValueSource::Raw,
            PyValueSource::Obj => ValueSource::Obj,
        }
    }
}

use lunamodel_types::Vtype;
use pyo3::prelude::*;

use crate::{
    bounds::PyBounds
};
use super::PyVariable;

#[pymethods]
impl PyVariable {
    #[getter]
    fn get_id(&self) -> usize {
        self.v.id() as usize
    }

    #[getter]
    fn get_name(&self) -> &str {
        self.v.name()
    }
    #[getter]
    fn get_bounds(&self) -> PyBounds {
        // todo: special bounds right away or rely on outside protocol?
        self.v.bounds()
    }

    #[getter]
    fn get_vtype(&self) -> Vtype {
        self.vtype()
    }

    fn __hash__(&self) -> PyResult<u64> {
        self.v.hash()?
    }
}

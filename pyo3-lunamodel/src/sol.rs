use std::sync::Arc;

use lunamodel_core::Solution;
use parking_lot::RwLock;
use lunamodel_python::PySolution as PyS;
use pyo3::{
    Bound, FromPyObject, IntoPyObject, PyAny, PyErr,
    types::{PyAnyMethods, PyCapsule},
};

use crate::LUNA_MODEL;

#[repr(transparent)]
/// A wrapper around a [`Arc<RwLock<Solution>>`] that can be converted to and from python with `pyo3`.
pub struct PySolution(pub Arc<RwLock<Solution>>);

impl<'a, 'py> FromPyObject<'a, 'py> for PySolution {
    type Error = PyErr;

    fn extract(obj: pyo3::Borrowed<'a, 'py, pyo3::PyAny>) -> Result<Self, Self::Error> {
        // check if it is the wrapper type or the PySolution type from the crate.
        let capsule: Bound<'py, PyCapsule> = if let Some(pys) = obj.getattr("_s").ok() {
            pys.call_method0("_to_capsule")
        } else {
            obj.call_method0("_to_capsule")
        }?
        .extract()?;
        let pys = PyS::_from_capsule(&capsule)?;
        Ok(PySolution(pys.s))
    }
}

impl<'py> IntoPyObject<'py> for PySolution {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: pyo3::Python<'py>) -> Result<Self::Output, Self::Error> {
        let pys_capsule = PyS { s: self.0 }._to_capsule(py)?;
        // We **must** call into the other library to ensure the exact same types are used.
        let lm = LUNA_MODEL.bind(py);
        let pys = lm
            .getattr("_lm")?
            .getattr("PySolution")?
            .call_method1("_from_capsule", (pys_capsule,))?;
        lm.getattr("Solution")?
            .call_method1("_from_pys", (pys,))
    }
}

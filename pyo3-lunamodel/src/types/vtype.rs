use derive_more::Deref;
use lunamodel_python::{PyVtype as PyV, ffi::CapsuleFFI};
use lunamodel_types::Vtype;
use pyo3::{Bound, FromPyObject, IntoPyObject, PyAny, PyErr, types::PyAnyMethods};

use crate::LUNA_MODEL;

#[repr(transparent)]
/// A wrapper around a [`Vtype`] that can be converted to and from python with `pyo3`.
#[derive(Deref)]
pub struct PyVtype(pub Vtype);

impl<'a, 'py> FromPyObject<'a, 'py> for PyVtype {
    type Error = PyErr;

    fn extract(obj: pyo3::Borrowed<'a, 'py, pyo3::PyAny>) -> Result<Self, Self::Error> {
        // check if it is the wrapper type or the PyEnvironment type from the crate.
        let capsule: String = if let Some(pye) = obj.getattr("_val").ok() {
            pye.call_method0("_to_capsule")
        } else {
            obj.call_method0("_to_capsule")
        }?
        .extract()?;
        Ok(PyVtype(PyV::from_capsule(capsule)?.into()))
    }
}

impl<'py> IntoPyObject<'py> for PyVtype {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: pyo3::Python<'py>) -> Result<Self::Output, Self::Error> {
        let pyvtype: PyV = self.0.into();
        // We **must** call into the other library to ensure the exact same types are used.
        let lm = LUNA_MODEL.bind(py);
        let pyv = lm
            .getattr("_lm")?
            .getattr("PyVtype")?
            .call_method1("_from_capsule", (pyvtype.to_capsule(py)?,))?;
        lm.getattr("Vtype")?.call_method1("_from_pyvtype", (pyv,))
    }
}

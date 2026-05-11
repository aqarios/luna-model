use derive_more::Deref;
use lunamodel_python::{PyCtype as PyC, ffi::CapsuleFFI};
use lunamodel_types::Ctype;
use pyo3::{Bound, FromPyObject, IntoPyObject, PyAny, PyErr, types::PyAnyMethods};

use crate::{luna_model, utils::TypeCheck};

#[repr(transparent)]
/// A wrapper around a [`Ctype`] that can be converted to and from python with `pyo3`.
#[derive(Deref)]
pub struct PyCtype(pub Ctype);

impl<'a, 'py> FromPyObject<'a, 'py> for PyCtype {
    type Error = PyErr;

    fn extract(obj: pyo3::Borrowed<'a, 'py, pyo3::PyAny>) -> Result<Self, Self::Error> {
        obj.check_type("Ctype")?;
        // check if it is the wrapper type or the PyEnvironment type from the crate.
        let capsule: String = if let Ok(pye) = obj.getattr("_val") {
            pye.call_method0("_to_capsule")
        } else {
            obj.call_method0("_to_capsule")
        }?
        .extract()?;
        Ok(PyCtype(PyC::from_capsule(capsule)?.into()))
    }
}

impl<'py> IntoPyObject<'py> for PyCtype {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: pyo3::Python<'py>) -> Result<Self::Output, Self::Error> {
        let pyctype: PyC = self.0.into();
        // We **must** call into the other library to ensure the exact same types are used.
        let lm = luna_model(py)?;
        let pyv = lm
            .getattr("_lm")?
            .getattr("PyCtype")?
            .call_method1("_from_capsule", (pyctype.to_capsule(py)?,))?;
        lm.getattr("Ctype")?.call_method1("_from_pyctype", (pyv,))
    }
}

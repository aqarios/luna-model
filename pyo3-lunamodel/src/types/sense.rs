use derive_more::Deref;
use lunamodel_python::{PySense as PyS, ffi::CapsuleFFI};
use lunamodel_types::Sense;
use pyo3::{Bound, FromPyObject, IntoPyObject, PyAny, PyErr, types::PyAnyMethods};

use crate::{luna_model, utils::TypeCheck};

#[repr(transparent)]
/// A wrapper around a [`Sense`] that can be converted to and from python with `pyo3`.
#[derive(Deref)]
pub struct PySense(pub Sense);

impl<'a, 'py> FromPyObject<'a, 'py> for PySense {
    type Error = PyErr;

    fn extract(obj: pyo3::Borrowed<'a, 'py, pyo3::PyAny>) -> Result<Self, Self::Error> {
        obj.check_type("Sense")?;
        // check if it is the wrapper type or the PyEnvironment type from the crate.
        let capsule: String = if let Ok(pye) = obj.getattr("_val") {
            pye.call_method0("_to_capsule")
        } else {
            obj.call_method0("_to_capsule")
        }?
        .extract()?;
        Ok(PySense(PyS::from_capsule(capsule)?.into()))
    }
}

impl<'py> IntoPyObject<'py> for PySense {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: pyo3::Python<'py>) -> Result<Self::Output, Self::Error> {
        let pysense: PyS = self.0.into();
        // We **must** call into the other library to ensure the exact same types are used.
        let lm = luna_model(py)?;
        let pyv = lm
            .getattr("_lm")?
            .getattr("PySense")?
            .call_method1("_from_capsule", (pysense.to_capsule(py)?,))?;
        lm.getattr("Sense")?.call_method1("_from_pysense", (pyv,))
    }
}

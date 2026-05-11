use lunamodel_python::{ffi::CapsuleFFI, prelude::PyModelContent};
use pyo3::{
    Bound, FromPyObject, IntoPyObject, PyAny, PyErr,
    types::{PyAnyMethods, PyCapsule},
};

use crate::{luna_model, utils::TypeCheck};

#[repr(transparent)]
pub struct PyModel(pub PyModelContent);

impl<'a, 'py> FromPyObject<'a, 'py> for PyModel {
    type Error = PyErr;

    fn extract(obj: pyo3::Borrowed<'a, 'py, pyo3::PyAny>) -> Result<Self, Self::Error> {
        obj.check_type("Model")?;
        let capsule: (Bound<'py, PyCapsule>, Bound<'py, PyCapsule>) =
            if let Ok(pye) = obj.getattr("_m") {
                pye.call_method0("_to_capsule")
            } else {
                obj.call_method0("_to_capsule")
            }?
            .extract()?;
        let pymodelcontent = PyModelContent::from_capsule(capsule)?;
        Ok(Self(pymodelcontent))
    }
}

impl<'py> IntoPyObject<'py> for PyModel {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: pyo3::Python<'py>) -> Result<Self::Output, Self::Error> {
        let capsule = self.0.to_capsule(py)?;
        // We **must** call into the other library to ensure the exact same types are used.
        let lm = luna_model(py)?;
        let pye = lm
            .getattr("_lm")?
            .getattr("PyModel")?
            .call_method1("_from_capsule", (capsule,))?;
        lm.getattr("Model")?.call_method1("_from_pym", (pye,))
    }
}

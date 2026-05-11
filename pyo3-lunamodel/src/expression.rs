use lunamodel_python::{ffi::CapsuleFFI, prelude::PyExprContent};
use pyo3::{
    Bound, FromPyObject, IntoPyObject, PyAny, PyErr,
    types::{PyAnyMethods, PyCapsule},
};

use crate::{luna_model, utils::TypeCheck};

#[repr(transparent)]
pub struct PyExpression(pub PyExprContent);

impl<'a, 'py> FromPyObject<'a, 'py> for PyExpression {
    type Error = PyErr;

    fn extract(obj: pyo3::Borrowed<'a, 'py, pyo3::PyAny>) -> Result<Self, Self::Error> {
        obj.check_type("Expression")?;
        let capsule: Bound<'py, PyCapsule> = if let Ok(pye) = obj.getattr("_expr") {
            pye.call_method0("_to_capsule")
        } else {
            obj.call_method0("_to_capsule")
        }?
        .extract()?;
        let pyexprcont = PyExprContent::from_capsule(capsule)?;
        Ok(Self(pyexprcont))
    }
}

impl<'py> IntoPyObject<'py> for PyExpression {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: pyo3::Python<'py>) -> Result<Self::Output, Self::Error> {
        let capsule = self.0.to_capsule(py)?;
        // We **must** call into the other library to ensure the exact same types are used.
        let lm = luna_model(py)?;
        let pye = lm
            .getattr("_lm")?
            .getattr("PyExpression")?
            .call_method1("_from_capsule", (capsule,))?;
        lm.getattr("Expression")?
            .call_method1("_from_pyexpr", (pye,))
    }
}

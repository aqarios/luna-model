use lunamodel_python::ffi::CapsuleFFI;
use lunamodel_types::Specs;
use pyo3::{
    Bound, FromPyObject, IntoPyObject, PyAny, PyErr,
    types::{PyAnyMethods, PyCapsule},
};

use crate::{luna_model, utils::TypeCheck};

#[repr(transparent)]
pub struct PyModelSpecs(pub Specs);

impl<'a, 'py> FromPyObject<'a, 'py> for PyModelSpecs {
    type Error = PyErr;

    fn extract(obj: pyo3::Borrowed<'a, 'py, pyo3::PyAny>) -> Result<Self, Self::Error> {
        obj.check_type("ModelSpecs")?;
        let capsule: Bound<'py, PyCapsule> = if let Ok(pyms) = obj.getattr("_sp") {
            pyms.call_method0("_to_capsule")
        } else {
            obj.call_method0("_to_capsule")
        }?
        .extract()?;
        Ok(Self(Specs::from_capsule(capsule)?))
    }
}

impl<'py> IntoPyObject<'py> for PyModelSpecs {
    type Target = PyAny;
    type Output = Bound<'py, Self::Target>;
    type Error = PyErr;

    fn into_pyobject(self, py: pyo3::Python<'py>) -> Result<Self::Output, Self::Error> {
        let capsule = self.0.to_capsule(py)?;
        let lm = luna_model(py)?;
        let pyms = lm
            .getattr("_lm")?
            .getattr("PyModelSpecs")?
            .call_method1("_from_capsule", (capsule,))?;
        lm.getattr("ModelSpecs")?
            .call_method1("_from_pysp", (pyms,))
    }
}

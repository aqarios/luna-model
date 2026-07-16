#[macro_use]
mod macros;

mod bounds;
mod constraint;
mod constraint_collection;
mod environment;
mod expression;
mod model;
mod sol;
mod types;
mod utils;
mod variable;

pub mod prelude;

use lunamodel_cext_pyffi::C_API_CAPSULE_NAME;
use pyo3::exceptions::PyImportError;
use pyo3::prelude::{Bound, Py, PyAnyMethods, PyModule, PyResult, Python};
use pyo3::types::{PyCapsule, PyCapsuleMethods};
use rs_lunamodel::LunaModelCApi;
use std::sync::LazyLock;

static LUNA_MODEL: LazyLock<PyResult<Py<PyModule>>> =
    LazyLock::new(|| Python::attach(|py| PyModule::import(py, "luna_model").map(Bound::unbind)));

pub(crate) fn luna_model(py: Python<'_>) -> PyResult<Bound<'_, PyModule>> {
    LUNA_MODEL
        .as_ref()
        .map(|m| m.bind(py).clone())
        .map_err(|e| e.clone_ref(py))
}

pub(crate) fn ensure_c_api(py: Python<'_>) -> PyResult<()> {
    if rs_lunamodel::is_initialized() {
        return Ok(());
    }

    let module = PyModule::import(py, "luna_model._lm")?;
    let capsule = module.getattr("_C_API")?.cast_into::<PyCapsule>()?;

    let pointer = capsule.pointer_checked(Some(C_API_CAPSULE_NAME))?;
    let table = pointer.cast::<LunaModelCApi>().as_ptr();

    match unsafe { rs_lunamodel::init(table) } {
        Ok(()) | Err(rs_lunamodel::InitError::AlreadyInitialized) => Ok(()),
        Err(error) => Err(PyImportError::new_err(error.to_string())),
    }
}

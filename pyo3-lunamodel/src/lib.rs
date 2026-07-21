#[macro_use]
mod macros;

mod bounds;
mod constraint;
mod constraint_collection;
mod environment;
pub mod error;
mod expression;
mod model;
mod sol;
mod types;
mod utils;
mod variable;

pub mod prelude;

// Re-exported so the plugin-authoring macros below can reference these crates through `$crate`,
// keeping extension authors' dependency list to just `pyo3` + `pyo3-lunamodel` + `rs-lunamodel`.
pub use lunamodel_cext_pyffi;
pub use rs_lunamodel;

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

/// Ensures the host LunaModel C API table is installed for this extension.
///
/// Reads the `luna_model._lm._C_API` capsule and hands it to [`rs_lunamodel::init`] (idempotent).
/// Extension authors call this before any raw ABI use — e.g. before registering a plugin pass's
/// reversal handler, or before returning a plugin descriptor whose trampolines dispatch through
/// the table. The conversion wrappers call it automatically, so most code never needs to.
pub fn ensure_c_api(py: Python<'_>) -> PyResult<()> {
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

/// Generates a Python-facing pass class wrapping a type that implements
/// [`rs_lunamodel::plugin::TransformationPass`].
///
/// The generated `#[pyclass]` is a newtype `struct $py($logic)` — it *stores* the pass instance
/// and clones it to build the descriptor, so construction lives outside the macro (no `Default`
/// requirement). You supply the constructor(s) in your own `#[pymethods]` block (enable pyo3's
/// `multiple-pymethods` feature), building `Self(<your configured pass>)`. The logic type must be
/// `Clone`; the SDK supplies all the C-boundary glue.
///
/// ```ignore
/// #[derive(Clone)]
/// struct Relabel { prefix: String }
/// impl rs_lunamodel::plugin::TransformationPass for Relabel { /* ... */ }
///
/// pyo3_lunamodel::py_transform_plugin!(RelabelPass => Relabel);
///
/// #[pymethods]
/// impl RelabelPass {
///     #[new]
///     fn new(prefix: String) -> Self { Self(Relabel { prefix }) }
/// }
/// ```
#[macro_export]
macro_rules! py_transform_plugin {
    ($vis:vis $py:ident => $logic:ty) => {
        #[::pyo3::pyclass]
        $vis struct $py($logic);

        #[::pyo3::pymethods]
        impl $py {
            /// Hands the host this pass's C descriptor (read when added to a `PassManager`).
            fn _luna_plugin_transform_capsule<'py>(
                &self,
                py: ::pyo3::Python<'py>,
            ) -> ::pyo3::PyResult<::pyo3::Bound<'py, ::pyo3::types::PyCapsule>> {
                $crate::ensure_c_api(py)?;
                let descriptor = $crate::rs_lunamodel::plugin::descriptor(self.0.clone())
                    .map_err(|error| ::pyo3::exceptions::PyRuntimeError::new_err(error.to_string()))?;
                $crate::lunamodel_cext_pyffi::plugin::new_plugin_transform_capsule(py, descriptor)
            }
        }
    };
}

/// Wires up the `__luna_register__` module hook that `luna_model.register(module)` invokes,
/// registering the reversal handlers for every listed [`rs_lunamodel::plugin::TransformPlugin`]
/// type. Invoke inside your `#[pymodule]` function, passing the module.
///
/// ```ignore
/// #[pymodule]
/// fn my_ext(m: &Bound<'_, PyModule>) -> PyResult<()> {
///     m.add_class::<RelabelPass>()?;
///     pyo3_lunamodel::register_plugins!(m, Relabel)?;
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! register_plugins {
    ($module:expr, $($logic:ty),+ $(,)?) => {{
        #[::pyo3::pyfunction]
        fn __luna_register__(py: ::pyo3::Python<'_>) -> ::pyo3::PyResult<()> {
            $crate::ensure_c_api(py)?;
            $(
                $crate::rs_lunamodel::plugin::register_backward::<$logic>()
                    .map_err(|error| ::pyo3::exceptions::PyRuntimeError::new_err(error.to_string()))?;
            )+
            ::core::result::Result::Ok(())
        }
        $module.add_function(::pyo3::wrap_pyfunction!(__luna_register__, $module)?)
    }};
}

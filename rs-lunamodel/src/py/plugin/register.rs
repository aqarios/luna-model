//! The `register_plugins!` macro.

/// Wires up the `__luna_register__` module hook that `luna_model.register(module)` invokes,
/// registering the reversal handlers for every listed reversible pass type (transforms). Invoke
/// inside your `#[pymodule]` function, passing the module. Analysis passes are not reversible and
/// are not listed here.
///
/// ```ignore
/// #[pymodule]
/// fn my_ext(m: &Bound<'_, PyModule>) -> PyResult<()> {
///     m.add_class::<RelabelPass>()?;
///     rs_lunamodel::py::register_plugins!(m, Relabel)?;
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! register_plugins {
    ($module:expr, $($logic:ty),+ $(,)?) => {{
        #[::pyo3::pyfunction]
        fn __luna_register__(py: ::pyo3::Python<'_>) -> ::pyo3::PyResult<()> {
            $crate::py::ensure_c_api(py)?;
            $(
                $crate::plugin::register_backward::<$logic>()
                    .map_err(|error| ::pyo3::exceptions::PyRuntimeError::new_err(error.to_string()))?;
            )+
            ::core::result::Result::Ok(())
        }
        $module.add_function(::pyo3::wrap_pyfunction!(__luna_register__, $module)?)
    }};
}

//! The `register_plugins!` macro.

/// Wires up the `__luna_register__` module hook that `luna_model.register(module)` invokes,
/// registering the reversal handlers for every listed reversible pass type (transforms), and
/// handing the host each pass's artifact class so `ErasedArtifact.restore()` can rebuild it. Every
/// listed type must be wrapped by `py_transform_plugin!` or `py_composite_plugin!`. Invoke
/// inside your `#[pymodule]` function, passing the module. Analysis passes are not reversible and
/// are not listed here.
///
/// ```ignore
/// #[pymodule]
/// fn my_ext(m: &Bound<'_, PyModule>) -> PyResult<()> {
///     m.add_class::<RelabelPass>()?;
///     luna_model_ffi::py::register_plugins!(m, Relabel)?;
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! register_plugins {
    // Explicit kind groups. Transformation and composite passes are the reversible kinds; analyses
    // and the other kinds are not reversible and are not listed.
    ($module:expr, transform: [$($t:ty),* $(,)?], composite: [$($c:ty),* $(,)?] $(,)?) => {{
        /// Registers the reversal handlers and returns `{pass id: artifact class}`.
        #[::pyo3::pyfunction]
        fn __luna_register__(
            py: ::pyo3::Python<'_>,
        ) -> ::pyo3::PyResult<::pyo3::Bound<'_, ::pyo3::types::PyDict>> {
            use ::pyo3::types::PyDictMethods;
            $crate::py::ensure_c_api(py)?;
            let types = ::pyo3::types::PyDict::new(py);
            $(
                $crate::plugin::register_backward::<$t>()
                    .map_err($crate::py::error::into_pyerr)?;
                let (id, cls) = $crate::py::plugin_artifact_type::<$t, _>(py);
                types.set_item(id, cls)?;
            )*
            $(
                $crate::plugin::register_composite_backward::<$c>()
                    .map_err($crate::py::error::into_pyerr)?;
                let (id, cls) = $crate::py::plugin_artifact_type::<$c, _>(py);
                types.set_item(id, cls)?;
            )*
            ::core::result::Result::Ok(types)
        }
        $module.add_function(::pyo3::wrap_pyfunction!(__luna_register__, $module)?)
    }};
    // Back-compat shorthand: a bare list is all transformation passes.
    ($module:expr, $($logic:ty),+ $(,)?) => {
        $crate::register_plugins!($module, transform: [$($logic),+], composite: [])
    };
}

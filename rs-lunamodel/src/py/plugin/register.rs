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
///     luna_model_ffi::py::register_plugins!(m, Relabel)?;
///     Ok(())
/// }
/// ```
#[macro_export]
macro_rules! register_plugins {
    // Explicit kind groups. Transformation and composite passes are the reversible kinds; analyses
    // and the other kinds are not reversible and are not listed.
    ($module:expr, transform: [$($t:ty),* $(,)?], composite: [$($c:ty),* $(,)?] $(,)?) => {{
        #[::pyo3::pyfunction]
        fn __luna_register__(py: ::pyo3::Python<'_>) -> ::pyo3::PyResult<()> {
            $crate::py::ensure_c_api(py)?;
            $(
                $crate::plugin::register_backward::<$t>()
                    .map_err($crate::py::error::into_pyerr)?;
            )*
            $(
                $crate::plugin::register_composite_backward::<$c>()
                    .map_err($crate::py::error::into_pyerr)?;
            )*
            ::core::result::Result::Ok(())
        }
        $module.add_function(::pyo3::wrap_pyfunction!(__luna_register__, $module)?)
    }};
    // Back-compat shorthand: a bare list is all transformation passes.
    ($module:expr, $($logic:ty),+ $(,)?) => {
        $crate::register_plugins!($module, transform: [$($logic),+], composite: [])
    };
}

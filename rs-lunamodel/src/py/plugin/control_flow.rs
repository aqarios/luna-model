//! The `py_control_flow_plugin!` macro.

/// Generates a Python-facing **control-flow** pass class from a type implementing
/// [`rs_lunamodel::plugin::ControlFlowPass`].
///
/// Unlike the other plugin macros there is no artifact or result class: a control-flow pass produces
/// a `ControlFlowPlan` (a pipeline of steps), not a serialized payload, and it is not reversible.
/// It is also not Python-callable directly — building a plan needs the host-lent model/context — so
/// it is exercised only by adding it to a `PassManager`. `py_control_flow_plugin!(vis Pass(Inner))`
/// emits `struct Pass(Inner)` — a `#[pyclass]` with `name`/`requires`/`provides`/`invalidates` and
/// the internal `_luna_plugin_control_flow_capsule`.
///
/// You supply the pass constructor(s) in your own `#[pymethods]` block. `Inner` must be `Clone`.
#[macro_export]
macro_rules! py_control_flow_plugin {
    ($vis:vis $py:ident ( $logic:ty )) => {
        #[::pyo3::pyclass]
        $vis struct $py($logic);

        #[::pyo3::pymethods]
        impl $py {
            fn name(&self) -> ::std::string::String {
                $crate::plugin::ControlFlowPass::name(&self.0).to_string()
            }

            fn requires(&self) -> ::std::vec::Vec<::std::string::String> {
                $crate::plugin::ControlFlowPass::requires(&self.0).to_vec()
            }

            fn provides(&self) -> ::std::vec::Vec<::std::string::String> {
                $crate::plugin::ControlFlowPass::provides(&self.0).to_vec()
            }

            fn invalidates(&self) -> ::std::vec::Vec<::std::string::String> {
                $crate::plugin::ControlFlowPass::invalidates(&self.0).to_vec()
            }

            /// Hands the host this pass's C descriptor (read when added to a `PassManager`).
            fn _luna_plugin_control_flow_capsule<'py>(
                &self,
                py: ::pyo3::Python<'py>,
            ) -> ::pyo3::PyResult<::pyo3::Bound<'py, ::pyo3::types::PyCapsule>> {
                $crate::py::ensure_c_api(py)?;
                let descriptor = $crate::plugin::descriptor_control_flow(self.0.clone())
                    .map_err(|e| ::pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
                $crate::py::lunamodel_capi_pyffi::plugin::new_plugin_control_flow_capsule(py, descriptor)
            }
        }
    };
}

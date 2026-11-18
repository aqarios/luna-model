//! The `py_meta_analysis_plugin!` macro.

/// Generates a Python-facing **meta-analysis** pass class **and** its result class from a type
/// implementing [`luna_model_ffi::plugin::MetaAnalysisPass`].
///
/// Unlike [`py_analysis_plugin!`], a meta-analysis reads no model — it inspects the pipeline steps
/// that follow it, which the host lends as a `StepList` only inside a `PassManager` run. There is
/// therefore no Python-callable `run` (a `StepList` cannot be constructed from Python); the pass is
/// exercised by adding it to a `PassManager`. It is not reversible, so — like an analysis — it is
/// not passed to [`register_plugins!`]. `py_meta_analysis_plugin!(vis Pass(Inner) => Result)` emits:
/// - `struct Pass(Inner)` — a `#[pyclass]` with `name`/`provides` and the internal
///   `_luna_plugin_meta_analysis_capsule`.
/// - `struct Result(<Inner as MetaAnalysisPass>::Output)` — a `#[pyclass]` with `serialize`/
///   `deserialize`.
///
/// You supply the pass constructor(s) in your own `#[pymethods]` block. `Inner` must be `Clone`.
#[macro_export]
macro_rules! py_meta_analysis_plugin {
    ($vis:vis $py:ident ( $logic:ty ) => $result:ident) => {
        // ---- result class ----
        #[::pyo3::pyclass]
        $vis struct $result(<$logic as $crate::plugin::MetaAnalysisPass>::Output);

        impl $result {
            /// Borrows the wrapped analysis result.
            pub fn inner(&self) -> &<$logic as $crate::plugin::MetaAnalysisPass>::Output {
                &self.0
            }
        }

        #[::pyo3::pymethods]
        impl $result {
            /// Serializes the result to protobuf bytes.
            fn serialize<'py>(
                &self,
                py: ::pyo3::Python<'py>,
            ) -> ::pyo3::PyResult<::pyo3::Bound<'py, ::pyo3::types::PyBytes>> {
                let bytes = $crate::prost::Message::encode_to_vec(&self.0);
                ::core::result::Result::Ok(::pyo3::types::PyBytes::new(py, &bytes))
            }

            /// Reconstructs the result from protobuf bytes.
            #[staticmethod]
            fn deserialize(
                bytes: &::pyo3::Bound<'_, ::pyo3::types::PyBytes>,
            ) -> ::pyo3::PyResult<Self> {
                use ::pyo3::types::PyBytesMethods;
                let result = <
                    <$logic as $crate::plugin::MetaAnalysisPass>::Output
                    as $crate::prost::Message
                >::decode(bytes.as_bytes())
                    .map_err(|e| ::pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
                ::core::result::Result::Ok(Self(result))
            }
        }

        // ---- pass class ----
        #[::pyo3::pyclass]
        $vis struct $py($logic);

        #[::pyo3::pymethods]
        impl $py {
            fn name(&self) -> ::std::string::String {
                $crate::plugin::MetaAnalysisPass::name(&self.0).to_string()
            }

            fn provides(&self) -> ::std::string::String {
                <$logic as $crate::plugin::MetaAnalysisPass>::PROVIDES.to_string()
            }

            /// Hands the host this pass's C descriptor (read when added to a `PassManager`).
            fn _luna_plugin_meta_analysis_capsule<'py>(
                &self,
                py: ::pyo3::Python<'py>,
            ) -> ::pyo3::PyResult<::pyo3::Bound<'py, ::pyo3::types::PyCapsule>> {
                $crate::py::ensure_c_api(py)?;
                let descriptor = $crate::plugin::descriptor_meta_analysis(self.0.clone())
                    .map_err($crate::py::error::into_pyerr)?;
                $crate::py::lunamodel_capi_pyffi::plugin::new_plugin_meta_analysis_capsule(py, descriptor)
            }
        }
    };
}

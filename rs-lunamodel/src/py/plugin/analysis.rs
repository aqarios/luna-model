//! The `py_analysis_plugin!` macro.

/// Generates a Python-facing **analysis** pass class **and** its result class from a type
/// implementing [`luna_model_ffi::plugin::AnalysisPass`], giving the plugin the full native
/// `AnalysisPass` surface from Python.
///
/// The C-boundary analog of a pure-Python analysis pass (subclassing luna-model's `PyAnalysisPass`);
/// both kinds can be mixed in one `PassManager`. `py_analysis_plugin!(vis Pass(Inner) => Result)`
/// emits:
/// - `struct Pass(Inner)` — a `#[pyclass]` with `name`/`provides`/`requires`/`depends_on`,
///   `run(model, ctx=None) -> Result`, and the internal `_luna_plugin_analysis_capsule`. Every
///   method delegates to the Rust facade trait, so the logic stays in Rust.
/// - `struct Result(<Inner as AnalysisPass>::Output)` — a `#[pyclass]` with `serialize`/
///   `deserialize`.
///
/// Analysis passes are not reversible, so — unlike [`py_transform_plugin!`] — they need no backward
/// registration and are not passed to [`register_plugins!`]. You supply the pass constructor(s) in
/// your own `#[pymethods]` block (enable pyo3's `multiple-pymethods` feature). `Inner` must be
/// `Clone`.
///
/// ```ignore
/// luna_model_ffi::py::py_analysis_plugin!(pub CountVariables(CountLogic) => VariableCount);
///
/// #[pymethods]
/// impl CountVariables {
///     #[new]
///     fn new() -> Self { Self(CountLogic) }
/// }
/// ```
#[macro_export]
macro_rules! py_analysis_plugin {
    ($vis:vis $py:ident ( $logic:ty ) => $result:ident) => {
        // ---- result class ----
        #[::pyo3::pyclass]
        $vis struct $result(<$logic as $crate::plugin::AnalysisPass>::Output);

        impl $result {
            /// Borrows the wrapped analysis result.
            pub fn inner(
                &self,
            ) -> &<$logic as $crate::plugin::AnalysisPass>::Output {
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
                    <$logic as $crate::plugin::AnalysisPass>::Output
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
                $crate::plugin::AnalysisPass::name(&self.0).to_string()
            }

            fn provides(&self) -> ::std::string::String {
                <$logic as $crate::plugin::AnalysisPass>::PROVIDES.to_string()
            }

            fn requires(&self) -> ::std::vec::Vec<::std::string::String> {
                $crate::plugin::AnalysisPass::requires(&self.0).to_vec()
            }

            fn depends_on<'py>(
                &self,
                py: ::pyo3::Python<'py>,
            ) -> ::pyo3::PyResult<::pyo3::Bound<'py, ::pyo3::types::PySet>> {
                $crate::py::model_parts_to_py(py, $crate::plugin::AnalysisPass::depends_on(&self.0))
            }

            /// Computes the analysis result from the model, mirroring the native Python API. The
            /// analysis does not mutate the model; `ctx` supplies analyses available so far
            /// (detached if omitted).
            #[pyo3(signature = (model, ctx = None))]
            fn run(
                &self,
                model: $crate::py::prelude::PyModel,
                ctx: ::core::option::Option<::pyo3::PyRef<'_, $crate::py::prelude::PyPassContext>>,
            ) -> ::pyo3::PyResult<$result> {
                let detached;
                let context = match &ctx {
                    ::core::option::Option::Some(ctx) => ctx.inner(),
                    ::core::option::Option::None => {
                        detached = $crate::plugin::PassContext::detached();
                        &detached
                    }
                };
                let output = $crate::plugin::AnalysisPass::run(
                    &self.0,
                    model.inner(),
                    context,
                )
                .map_err($crate::py::error::into_pyerr)?;
                ::core::result::Result::Ok($result(output))
            }

            /// Hands the host this pass's C descriptor (read when added to a `PassManager`).
            fn _luna_plugin_analysis_capsule<'py>(
                &self,
                py: ::pyo3::Python<'py>,
            ) -> ::pyo3::PyResult<::pyo3::Bound<'py, ::pyo3::types::PyCapsule>> {
                $crate::py::ensure_c_api(py)?;
                let descriptor = $crate::plugin::descriptor_analysis(self.0.clone())
                    .map_err($crate::py::error::into_pyerr)?;
                $crate::py::lunamodel_capi_pyffi::plugin::new_plugin_analysis_capsule(py, descriptor)
            }
        }
    };
}

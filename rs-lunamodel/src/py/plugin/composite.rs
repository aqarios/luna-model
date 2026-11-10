//! The `py_composite_plugin!` macro.

/// Generates a Python-facing **composite** pass class, its artifact class, and its result class from
/// a type implementing [`luna_model_ffi::plugin::CompositePass`], giving the plugin the full native
/// `CompositePass` surface from Python.
///
/// A composite fuses a transformation and an analysis, so — unlike [`py_transform_plugin!`] — its
/// `forward` returns `(model, artifact, result)`, and — unlike [`py_analysis_plugin!`] — it is
/// reversible (list it in [`register_plugins!`]'s `composite:` group).
/// `py_composite_plugin!(vis Pass(Inner) => Artifact, Result)` emits:
/// - `struct Pass(Inner)` — a `#[pyclass]` with `name`/`provides`/`requires`/`modifies`/`depends_on`,
///   `forward(model, ctx=None) -> (Model, Artifact, Result)`, classmethod `backward(artifact,
///   solution) -> Solution`, and the internal `_luna_plugin_composite_capsule`.
/// - `struct Artifact(<Inner as Reversible>::Artifact)` — a `#[pyclass]` with `serialize`/
///   `deserialize`.
/// - `struct Result(<Inner as CompositePass>::Output)` — a `#[pyclass]` with `serialize`/
///   `deserialize`.
///
/// You supply the pass constructor(s) in your own `#[pymethods]` block (enable pyo3's
/// `multiple-pymethods` feature). `Inner` must be `Clone`.
///
/// ```ignore
/// luna_model_ffi::py::py_composite_plugin!(pub RenameAndCount(Logic) => RenameArtifact, VariableCount);
///
/// #[pymethods]
/// impl RenameAndCount {
///     #[new]
///     fn new() -> Self { Self(Logic) }
/// }
/// ```
#[macro_export]
macro_rules! py_composite_plugin {
    ($vis:vis $py:ident ( $logic:ty ) => $artifact:ident, $result:ident) => {
        // ---- artifact class ----
        #[::pyo3::pyclass]
        $vis struct $artifact(<$logic as $crate::plugin::Reversible>::Artifact);

        impl $artifact {
            /// Borrows the wrapped artifact.
            pub fn inner(&self) -> &<$logic as $crate::plugin::Reversible>::Artifact {
                &self.0
            }
        }

        #[::pyo3::pymethods]
        impl $artifact {
            /// Serializes the artifact to protobuf bytes.
            fn serialize<'py>(
                &self,
                py: ::pyo3::Python<'py>,
            ) -> ::pyo3::PyResult<::pyo3::Bound<'py, ::pyo3::types::PyBytes>> {
                let bytes = $crate::prost::Message::encode_to_vec(&self.0);
                ::core::result::Result::Ok(::pyo3::types::PyBytes::new(py, &bytes))
            }

            /// Reconstructs the artifact from protobuf bytes.
            #[staticmethod]
            fn deserialize(
                bytes: &::pyo3::Bound<'_, ::pyo3::types::PyBytes>,
            ) -> ::pyo3::PyResult<Self> {
                use ::pyo3::types::PyBytesMethods;
                let artifact = <
                    <$logic as $crate::plugin::Reversible>::Artifact
                    as $crate::prost::Message
                >::decode(bytes.as_bytes())
                    .map_err(|e| ::pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
                ::core::result::Result::Ok(Self(artifact))
            }
        }

        // ---- result class ----
        #[::pyo3::pyclass]
        $vis struct $result(<$logic as $crate::plugin::CompositePass>::Output);

        impl $result {
            /// Borrows the wrapped analysis result.
            pub fn inner(&self) -> &<$logic as $crate::plugin::CompositePass>::Output {
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
                    <$logic as $crate::plugin::CompositePass>::Output
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
                $crate::plugin::CompositePass::name(&self.0).to_string()
            }

            fn provides(&self) -> ::std::string::String {
                <$logic as $crate::plugin::CompositePass>::PROVIDES.to_string()
            }

            fn requires(&self) -> ::std::vec::Vec<::std::string::String> {
                $crate::plugin::CompositePass::requires(&self.0).to_vec()
            }

            fn modifies<'py>(
                &self,
                py: ::pyo3::Python<'py>,
            ) -> ::pyo3::PyResult<::pyo3::Bound<'py, ::pyo3::types::PySet>> {
                $crate::py::model_parts_to_py(py, $crate::plugin::CompositePass::modifies(&self.0))
            }

            fn depends_on<'py>(
                &self,
                py: ::pyo3::Python<'py>,
            ) -> ::pyo3::PyResult<::pyo3::Bound<'py, ::pyo3::types::PySet>> {
                $crate::py::model_parts_to_py(py, $crate::plugin::CompositePass::depends_on(&self.0))
            }

            /// Runs the fused forward and returns `(model, artifact, result)`, mirroring the native
            /// Python API. The Rust `forward` mutates the model in place through its shared handle,
            /// so the returned model is the same underlying object.
            ///
            /// Pass a `PassContext` to make analysis results available to the pass; omit it (or
            /// pass `None`) to run detached. When the pass runs inside a `PassManager`, the host
            /// supplies the context instead.
            #[pyo3(signature = (model, ctx = None))]
            fn forward(
                &self,
                mut model: $crate::py::prelude::PyModel,
                ctx: ::core::option::Option<::pyo3::PyRef<'_, $crate::py::prelude::PyPassContext>>,
            ) -> ::pyo3::PyResult<($crate::py::prelude::PyModel, $artifact, $result)> {
                let detached;
                let context = match &ctx {
                    ::core::option::Option::Some(ctx) => ctx.inner(),
                    ::core::option::Option::None => {
                        detached = $crate::plugin::PassContext::detached();
                        &detached
                    }
                };
                let (artifact, output) = $crate::plugin::CompositePass::forward(
                    &self.0,
                    model.inner_mut(),
                    context,
                )
                .map_err($crate::py::error::into_pyerr)?;
                ::core::result::Result::Ok((model, $artifact(artifact), $result(output)))
            }

            #[classmethod]
            fn backward(
                _cls: &::pyo3::Bound<'_, ::pyo3::types::PyType>,
                artifact: ::pyo3::PyRef<'_, $artifact>,
                solution: $crate::py::prelude::PySolution,
            ) -> ::pyo3::PyResult<$crate::py::prelude::PySolution> {
                let solution = <$logic as $crate::plugin::Reversible>::backward(
                    artifact.inner(),
                    solution.into_inner(),
                )
                .map_err($crate::py::error::into_pyerr)?;
                ::core::result::Result::Ok($crate::py::prelude::PySolution::new(solution))
            }

            /// Hands the host this pass's C descriptor (read when added to a `PassManager`).
            fn _luna_plugin_composite_capsule<'py>(
                &self,
                py: ::pyo3::Python<'py>,
            ) -> ::pyo3::PyResult<::pyo3::Bound<'py, ::pyo3::types::PyCapsule>> {
                $crate::py::ensure_c_api(py)?;
                let descriptor = $crate::plugin::descriptor_composite(self.0.clone())
                    .map_err($crate::py::error::into_pyerr)?;
                $crate::py::lunamodel_capi_pyffi::plugin::new_plugin_composite_capsule(py, descriptor)
            }
        }
    };
}

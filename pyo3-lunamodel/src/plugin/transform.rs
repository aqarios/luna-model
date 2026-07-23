//! The `py_transform_plugin!` macro.

/// Generates a Python-facing pass class **and** its artifact class from a type implementing
/// [`rs_lunamodel::plugin::TransformationPass`], giving the plugin the full native
/// `TransformationPass` surface from Python.
///
/// `py_transform_plugin!(vis Pass(Inner) => Artifact)` emits:
/// - `struct Pass(Inner)` — a `#[pyclass]` with `name`/`requires`/`invalidates`,
///   `forward(model, ctx=None) -> (Model, Artifact)`, classmethod `backward(artifact, solution) ->
///   Solution`, and the internal `_luna_plugin_transform_capsule`. Every method delegates to the
///   Rust facade traits, so the logic stays in Rust; Python just gets the same callable surface.
/// - `struct Artifact(<Inner as Reversible>::Artifact)` — a `#[pyclass]` with `serialize`/
///   `deserialize`.
///
/// You supply the pass constructor(s) and any artifact getters in your own `#[pymethods]` blocks
/// (enable pyo3's `multiple-pymethods` feature). `Inner` must be `Clone`.
///
/// ```ignore
/// pyo3_lunamodel::py_transform_plugin!(pub RelabelPass(Relabel) => RelabelArtifact);
///
/// #[pymethods]
/// impl RelabelPass {
///     #[new]
///     fn new(prefix: String) -> Self { Self(Relabel { prefix }) }
/// }
/// ```
#[macro_export]
macro_rules! py_transform_plugin {
    ($vis:vis $py:ident ( $logic:ty ) => $artifact:ident) => {
        // ---- artifact class ----
        #[::pyo3::pyclass]
        $vis struct $artifact(<$logic as $crate::rs_lunamodel::plugin::Reversible>::Artifact);

        impl $artifact {
            /// Borrows the wrapped artifact.
            pub fn inner(&self) -> &<$logic as $crate::rs_lunamodel::plugin::Reversible>::Artifact {
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
                let bytes = $crate::rs_lunamodel::prost::Message::encode_to_vec(&self.0);
                ::core::result::Result::Ok(::pyo3::types::PyBytes::new(py, &bytes))
            }

            /// Reconstructs the artifact from protobuf bytes.
            #[staticmethod]
            fn deserialize(
                bytes: &::pyo3::Bound<'_, ::pyo3::types::PyBytes>,
            ) -> ::pyo3::PyResult<Self> {
                use ::pyo3::types::PyBytesMethods;
                let artifact = <
                    <$logic as $crate::rs_lunamodel::plugin::Reversible>::Artifact
                    as $crate::rs_lunamodel::prost::Message
                >::decode(bytes.as_bytes())
                    .map_err(|e| ::pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
                ::core::result::Result::Ok(Self(artifact))
            }
        }

        // ---- pass class ----
        #[::pyo3::pyclass]
        $vis struct $py($logic);

        #[::pyo3::pymethods]
        impl $py {
            fn name(&self) -> ::std::string::String {
                $crate::rs_lunamodel::plugin::TransformationPass::name(&self.0).to_string()
            }

            fn requires(&self) -> ::std::vec::Vec<::std::string::String> {
                $crate::rs_lunamodel::plugin::TransformationPass::requires(&self.0).to_vec()
            }

            fn invalidates(&self) -> ::std::vec::Vec<::std::string::String> {
                $crate::rs_lunamodel::plugin::TransformationPass::invalidates(&self.0).to_vec()
            }

            /// Transforms the model and returns `(model, artifact)`, mirroring the native Python
            /// API. The Rust `forward` mutates the model in place through its shared handle, so the
            /// returned model is the same underlying object.
            ///
            /// Pass a `PassContext` to make analysis results available to the pass; omit it (or
            /// pass `None`) to run detached. When the pass runs inside a `PassManager`, the host
            /// supplies the context instead.
            #[pyo3(signature = (model, ctx = None))]
            fn forward(
                &self,
                mut model: $crate::prelude::PyModel,
                ctx: ::core::option::Option<::pyo3::PyRef<'_, $crate::prelude::PyPassContext>>,
            ) -> ::pyo3::PyResult<($crate::prelude::PyModel, $artifact)> {
                let detached;
                let context = match &ctx {
                    ::core::option::Option::Some(ctx) => ctx.inner(),
                    ::core::option::Option::None => {
                        detached = $crate::rs_lunamodel::plugin::PassContext::detached();
                        &detached
                    }
                };
                let artifact = $crate::rs_lunamodel::plugin::TransformationPass::forward(
                    &self.0,
                    model.inner_mut(),
                    context,
                )
                .map_err(|e| ::pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
                ::core::result::Result::Ok((model, $artifact(artifact)))
            }

            #[classmethod]
            fn backward(
                _cls: &::pyo3::Bound<'_, ::pyo3::types::PyType>,
                artifact: ::pyo3::PyRef<'_, $artifact>,
                solution: $crate::prelude::PySolution,
            ) -> ::pyo3::PyResult<$crate::prelude::PySolution> {
                let solution = <$logic as $crate::rs_lunamodel::plugin::Reversible>::backward(
                    artifact.inner(),
                    solution.into_inner(),
                )
                .map_err(|e| ::pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
                ::core::result::Result::Ok($crate::prelude::PySolution::new(solution))
            }

            /// Hands the host this pass's C descriptor (read when added to a `PassManager`).
            fn _luna_plugin_transform_capsule<'py>(
                &self,
                py: ::pyo3::Python<'py>,
            ) -> ::pyo3::PyResult<::pyo3::Bound<'py, ::pyo3::types::PyCapsule>> {
                $crate::ensure_c_api(py)?;
                let descriptor = $crate::rs_lunamodel::plugin::descriptor(self.0.clone())
                    .map_err(|e| ::pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
                $crate::lunamodel_capi_pyffi::plugin::new_plugin_transform_capsule(py, descriptor)
            }
        }
    };
}

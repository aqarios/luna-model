use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Type, parse_macro_input, parse_quote};

fn extract_newtype_inner(input: &DeriveInput, macro_name: &str) -> Type {
    let Data::Struct(data) = &input.data else {
        panic!("{macro_name} can only be applied to structs");
    };
    let Fields::Unnamed(fields) = &data.fields else {
        panic!("{macro_name} requires a tuple struct");
    };
    assert!(
        fields.unnamed.len() == 1,
        "{macro_name} requires exactly one field"
    );
    fields.unnamed[0].ty.clone()
}

/// Attribute macro for analysis pass newtype wrappers.
///
/// Usage:
///   `#[pyanalysis]`                — `run` returns `PyResult<()>` (default)
///   `#[pyanalysis(PyReturnType)]`  — `run` returns `PyResult<PyReturnType>`
///
/// Adds `#[pyclass(subclass)]` to the struct and generates a `#[pymethods]` impl
/// with: `provides` (classmethod), `name`, `requires`, `__str__`, and `run`.
///
/// `PyReturnType` must implement `From<InnerPass::Result>`.
#[proc_macro_attribute]
pub fn pyanalysis(attr: TokenStream, item: TokenStream) -> TokenStream {
    let result_type: Type = if attr.is_empty() {
        parse_quote!(())
    } else {
        parse_macro_input!(attr as Type)
    };

    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;
    let inner = extract_newtype_inner(&input, "pyanalysis");

    quote! {
        #[::pyo3::pyclass(subclass)]
        #input

        #[::pyo3::pymethods]
        impl #name {
            #[classmethod]
            fn provides(_cls: &::pyo3::Bound<'_, ::pyo3::types::PyType>) -> ::std::string::String {
                <#inner as ::lunamodel_transpiler::AnalysisPass>::PROVIDES.to_string()
            }

            fn name(&self) -> ::std::string::String {
                <#inner as ::lunamodel_transpiler::AnalysisPass>::name(&self.0).to_string()
            }

            fn requires(&self) -> ::std::vec::Vec<::std::string::String> {
                <#inner as ::lunamodel_transpiler::AnalysisPass>::requires(&self.0).to_vec()
            }

            fn __str__(&self) -> ::std::string::String {
                <#inner as ::lunamodel_transpiler::AnalysisPass>::display(&self.0)
            }

            fn run(
                &self,
                model: crate::PyModel,
                ctx: &crate::transform::PyPassContext,
            ) -> ::pyo3::PyResult<#result_type> {
                Ok(<#inner as ::lunamodel_transpiler::AnalysisPass>::run(
                    &self.0,
                    &model.m.read_arc(),
                    &ctx.into(),
                )?.into())
            }
        }
    }
    .into()
}

/// Attribute macro for transformation pass newtype wrappers.
///
/// Usage:
///   `#[pytransformation]`                  — only common methods, no forward/backward
///   `#[pytransformation(PyArtifactType)]`  — also generates `forward` and `backward`
///
/// Adds `#[pyclass(subclass)]` to the struct and generates a `#[pymethods]` impl
/// with: `name`, `requires`, `invalidates`, `__str__`, and optionally `forward`/`backward`.
///
/// `PyArtifactType` must be a tuple struct wrapping the inner `InnerPass::Artifact`.
#[proc_macro_attribute]
pub fn pytransformation(attr: TokenStream, item: TokenStream) -> TokenStream {
    let artifact_type: Option<Type> = if attr.is_empty() {
        None
    } else {
        Some(parse_macro_input!(attr as Type))
    };

    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;
    let inner = extract_newtype_inner(&input, "pytransformation");

    let forward_backward = artifact_type.map(|at| {
        quote! {
            fn forward(
                &self,
                model: crate::PyModel,
                ctx: &crate::transform::PyPassContext,
            ) -> ::pyo3::PyResult<(crate::PyModel, #at)> {
                let mut m = model.m.read_arc().clone();
                let artifact = <#inner as ::lunamodel_transpiler::TransformationPass>::forward(
                    &self.0,
                    &mut m,
                    &ctx.into(),
                )?;
                Ok((m.into(), #at(artifact)))
            }

            #[classmethod]
            fn backward(
                _cls: &::pyo3::Bound<'_, ::pyo3::types::PyType>,
                artifact: &#at,
                solution: crate::PySolution,
            ) -> ::pyo3::PyResult<crate::PySolution> {
                Ok(<#inner as ::lunamodel_transpiler::Reversible>::backward(
                    &artifact.0,
                    solution.s.read_arc().clone(),
                )?
                .into())
            }
        }
    });

    quote! {
        #[::pyo3::pyclass(subclass)]
        #input

        #[::pyo3::pymethods]
        impl #name {
            fn name(&self) -> ::std::string::String {
                <#inner as ::lunamodel_transpiler::TransformationPass>::name(&self.0).to_string()
            }

            fn requires(&self) -> ::std::vec::Vec<::std::string::String> {
                <#inner as ::lunamodel_transpiler::TransformationPass>::requires(&self.0).to_vec()
            }

            fn invalidates(&self) -> ::std::vec::Vec<::std::string::String> {
                <#inner as ::lunamodel_transpiler::TransformationPass>::invalidates(&self.0).to_vec()
            }

            fn __str__(&self) -> ::std::string::String {
                <#inner as ::lunamodel_transpiler::TransformationPass>::display(&self.0)
            }

            #forward_backward
        }
    }
    .into()
}

/// Attribute macro for control-flow pass newtype wrappers.
///
/// Adds `#[pyclass(subclass)]` to the struct and generates a `#[pymethods]` impl
/// with: `name`, `requires`, `provides`, `invalidates`, `__str__`.
#[proc_macro_attribute]
pub fn pycontrolflow(attr: TokenStream, item: TokenStream) -> TokenStream {
    let _ = attr;
    let input = parse_macro_input!(item as DeriveInput);
    let name = &input.ident;
    let inner = extract_newtype_inner(&input, "pycontrolflow");

    quote! {
        #[::pyo3::pyclass(subclass)]
        #input

        #[::pyo3::pymethods]
        impl #name {
            fn name(&self) -> ::std::string::String {
                <#inner as ::lunamodel_transpiler::ControlFlowPass>::name(&self.0).to_string()
            }

            fn requires(&self) -> ::std::vec::Vec<::std::string::String> {
                <#inner as ::lunamodel_transpiler::ControlFlowPass>::requires(&self.0).to_vec()
            }

            fn provides(&self) -> ::std::vec::Vec<::std::string::String> {
                <#inner as ::lunamodel_transpiler::ControlFlowPass>::provides(&self.0).to_vec()
            }

            fn invalidates(&self) -> ::std::vec::Vec<::std::string::String> {
                <#inner as ::lunamodel_transpiler::ControlFlowPass>::invalidates(&self.0).to_vec()
            }

            fn run(
                &self,
                model: crate::PyModel,
                ctx: &crate::transform::PyPassContext,
            ) -> ::pyo3::PyResult<(crate::transform::PyControlFlowPlan)> {
                Ok(<#inner as ::lunamodel_transpiler::ControlFlowPass>::run(
                    &self.0,
                    &model.m.read_arc(),
                    &ctx.into(),
                )?.into())
            }

            fn __str__(&self) -> ::std::string::String {
                <#inner as ::lunamodel_transpiler::ControlFlowPass>::display(&self.0)
            }
        }
    }
    .into()
}

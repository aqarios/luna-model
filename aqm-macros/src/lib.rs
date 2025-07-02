use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    braced,
    parse::{Parse, ParseStream},
    parse_macro_input,
    punctuated::Punctuated,
    token::Comma,
    Ident, ItemStruct, Path, Result as SynResult, Token,
};
#[cfg(feature = "py")]
use syn::{parse::Parser, DeriveInput, Expr, Fields, MetaNameValue};

/// Replace the derive with an attribute macro:
#[cfg(feature = "py")]
#[proc_macro_attribute]
pub fn py_pass(attr: TokenStream, item: TokenStream) -> TokenStream {
    // 1) Parse the struct on which the attribute was placed:
    let input = parse_macro_input!(item as DeriveInput);
    let struct_name = &input.ident;

    // 2) Infer the wrapper and Python class names:
    let wrapper_name = format_ident!("Py{}", struct_name);
    let class_name = struct_name.to_string();

    // 3) Parse the attribute arguments, expecting exactly
    //    `pass_variant = "Something"`
    let mut pass_variant: Option<String> = None;
    let args = Punctuated::<MetaNameValue, Comma>::parse_terminated
        .parse(attr)
        .expect("invalid arguments to #[py_pass]: expected `pass_variant = \"...\"`");
    for nv in args {
        if nv.path.is_ident("pass_variant") {
            if let Expr::Lit(expr_lit) = &nv.value {
                if let syn::Lit::Str(litstr) = &expr_lit.lit {
                    pass_variant = Some(litstr.value());
                }
            }
        }
    }
    let pass_variant =
        pass_variant.unwrap_or_else(|| panic!("you must supply `pass_variant = \"...\"`"));

    let get_invalidates = if pass_variant == "Transformation" {
        quote! {
            #[getter]
            pub fn get_invalidates(&self) -> Vec<String> {
                return self.invalidates()
            }
        }
    } else {
        quote! {}
    };
    let invalidates_repr = if pass_variant == "Transformation" {
        quote! {
            let vec = self.invalidates();
            if !vec.is_empty() {
                parts.push(format!("invalidates={:?}", vec));
            }
        }
    } else {
        quote! {}
    };

    // Turn the string into an Ident so we splice it cleanly:
    let pass_variant_ident = format_ident!("{}", pass_variant);

    // 4) Pull out the struct’s named fields for constructor + getters
    let fields = match &input.data {
        syn::Data::Struct(ds) => {
            if let Fields::Named(named) = &ds.fields {
                named.named.iter().collect::<Vec<_>>()
            } else {
                panic!("py_pass only supports structs with named fields");
            }
        }
        _ => panic!("py_pass can only be used on structs"),
    };
    let init_args = fields.iter().map(|f| {
        let name = &f.ident;
        let ty = &f.ty;
        quote! { #name: #ty }
    });
    let forward = fields.iter().map(|f| {
        let name = &f.ident;
        quote! { #name }
    });
    let field_getters = fields.iter().map(|f| {
        let fname = f.ident.as_ref().unwrap();
        let getter = format_ident!("get_{}", fname);
        let ty = &f.ty;
        quote! {
            #[getter]
            pub fn #getter(&self) -> #ty {
                self.#fname.clone()
            }
        }
    });
    let field_names = fields
        .iter()
        .map(|f| f.ident.as_ref().unwrap())
        .collect::<Vec<_>>();

    // 5) Build your expanded code:
    let expanded = quote! {
        use pyo3::prelude::*;
        // Re-emit the original struct (unchanged)
        #input

        // The Python wrapper type
        #[pyclass(name = #class_name)]
        #[derive(::derive_more::Deref, ::derive_more::DerefMut, Clone)]
        pub struct #wrapper_name(pub #struct_name);

        #[pymethods]
        impl #wrapper_name {
            #[new]
            pub fn py_init(#(#init_args),*) -> Self {
                #wrapper_name(#struct_name::new(#(#forward),*))
            }

            #(#field_getters)*

            #[getter]
            pub fn get_name(&self) -> String {
                self.name()
            }

            #[getter]
            pub fn get_requires(&self) -> Vec<String> {
                self.requires()
            }

            #get_invalidates

            pub fn __repr__(&self) -> PyResult<String> {
                let mut parts = Vec::new();

                #(
                    {
                        let field_name = stringify!(#field_names);
                        let value = format!("{:?}", self.#field_names);
                        parts.push(format!("{}={}", field_name, value));
                    }
                )*

                parts.push(format!("name=\"{}\"", self.name()));
                let vec = self.requires();
                if !vec.is_empty() {
                    parts.push(format!("requires={:?}", vec));
                }
                #invalidates_repr

                Ok(format!("{}({})", #class_name, parts.join(", ")))
            }
        }

        impl #wrapper_name {
            pub fn as_pass(self) -> PyResult<Pass> {
                Ok(Pass::#pass_variant_ident(Box::new(self.0)))
            }
        }
    };

    TokenStream::from(expanded)
}

struct RegisterArgs {
    passes: Vec<Path>,
    specials: Vec<Path>,
    extras: Vec<Path>,
}

impl Parse for RegisterArgs {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let mut passes: Option<Vec<Path>> = None;
        let mut specials: Option<Vec<Path>> = None;
        let mut extras: Option<Vec<Path>> = None;

        // Keep reading `key = { … }` blocks until EOF
        while !input.is_empty() {
            let key: Ident = input.parse()?;
            input.parse::<Token![=]>()?;

            // parse the `{ … }` block
            let content;
            braced!(content in input);
            let list: Vec<Path> = Punctuated::<Path, Comma>::parse_terminated(&content)?
                .into_iter()
                .collect();

            match &*key.to_string() {
                "passes" => {
                    if passes.is_some() {
                        return Err(input.error("`passes` specified more than once"));
                    }
                    passes = Some(list);
                }
                "specials" => {
                    if specials.is_some() {
                        return Err(input.error("`specials` specified more than once"));
                    }
                    specials = Some(list);
                }
                "extras" => {
                    if extras.is_some() {
                        return Err(input.error("`extras` specified more than once"));
                    }
                    extras = Some(list);
                }
                other => {
                    return Err(input.error(format!(
                        "unexpected key `{}`, expected `passes`, `specials`, or `extras`",
                        other
                    )));
                }
            }

            // consume optional trailing comma
            let _ = input.parse::<Comma>();
        }

        let passes = passes.ok_or_else(|| input.error("you must provide `passes = { … }`"))?;
        let specials = specials.unwrap_or_default();
        let extras = extras.unwrap_or_default();

        Ok(RegisterArgs {
            passes,
            specials,
            extras,
        })
    }
}

#[proc_macro]
pub fn register_pytransformations(input: TokenStream) -> TokenStream {
    let RegisterArgs {
        passes,
        specials,
        extras,
    } = parse_macro_input!(input as RegisterArgs);

    // --- Build specials: Variant = TypeIdent, payload = Py<TypeIdent>
    let enum_specials = specials.iter().map(|path| {
        let ty_ident = &path.segments.last().unwrap().ident;
        quote! { #ty_ident(Py<#ty_ident>), }
    });
    let arm_specials = specials.iter().map(|path| {
        let ty_ident = &path.segments.last().unwrap().ident;
        quote! { AnyPass::#ty_ident(x) => x.as_pass()?, }
    });
    let reg_specials = specials.iter().map(|path| {
        quote! { m.add_class::<#path>()?; }
    });

    // --- Build normal passes: strip "Py" / "Pass" for variant names
    let mut enum_passes = Vec::new();
    let mut arm_passes = Vec::new();
    for path in &passes {
        let ty_ident = &path.segments.last().unwrap().ident;
        let s = ty_ident.to_string();
        let stripped = s
            .strip_prefix("Py")
            .unwrap_or(&s)
            .strip_suffix("Pass")
            .unwrap_or(&s);
        let var_ident = format_ident!("{}", stripped);

        enum_passes.push(quote! { #var_ident(#ty_ident), });
        arm_passes.push(quote! { AnyPass::#var_ident(x) => x.as_pass()?, });
    }
    let reg_passes = passes.iter().map(|path| {
        quote! { m.add_class::<#path>()?; }
    });

    // --- Extras
    let reg_extras = extras.iter().map(|path| {
        quote! { m.add_class::<#path>()?; }
    });

    // --- Emit everything
    let expanded = quote! {
        use pyo3::prelude::*;


        #[allow(dead_code)]
        #[derive(FromPyObject)]
        pub enum AnyPass {
            // specials first
            #(#enum_specials)*
            // then normal passes
            #(#enum_passes)*
        }

        impl AnyPass {
            pub fn as_pass(self) -> PyResult<Pass> {
                Ok(match self {
                    #(#arm_specials)*
                    #(#arm_passes)*
                })
            }
        }

        #[cfg(feature = "transformations")]
        pub fn register_transformations(pm: &Bound<'_, PyModule>) -> PyResult<()> {
            let m = PyModule::new(pm.py(), "transformations")?;

            // user-specified specials
            #(#reg_specials)*

            // extras, if any
            #(#reg_extras)*

            // normal passes
            #(#reg_passes)*

            pm.add_submodule(&m)?;
            #[cfg(not(feature = "lq"))]
            pm.py()
              .import("sys")?
              .getattr("modules")?
              .set_item("aqmodels.transformations", m)?;
            #[cfg(feature = "lq")]
            pm.py()
              .import("sys")?
              .getattr("modules")?
              .set_item("luna_quantum.transformations", m)?;
            Ok(())
        }
    };

    expanded.into()
}

// CACHES
/// ## 1) `#[analysis_cache]` attribute
///
/// Wraps your struct in a `cfg_attr(feature="py", pyclass(...))`.
#[proc_macro_attribute]
pub fn analysis_cache(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // Parse the annotated item as a struct (so we preserve its derives & fields)
    let input = parse_macro_input!(item as ItemStruct);
    let name = input.ident.to_string();
    let struct_name = &input.ident;

    let class_name = struct_name.to_string();

    let field_names = input.fields
        .iter()
        .map(|f| f.ident.as_ref().unwrap())
        .collect::<Vec<_>>();

    // Emit a cfg_attr only when `--features py` is on.
    let expanded = quote! {
        #[cfg_attr(
            all(feature = "py", not(feature = "lq")),
            pyo3::pyclass(get_all, name = #name, module = "aqmodels.transformations")
        )]
        #[cfg_attr(
            all(feature = "py", feature = "lq"),
            pyo3::pyclass(get_all, name = #name, module = "luna_quantum.transformations")
        )]
        #input

        #[pymethods]
        impl #struct_name {
            pub fn __repr__(&self) -> PyResult<String> {
                let mut parts = Vec::new();

                #(
                    {
                        let field_name = stringify!(#field_names);
                        let value = format!("{:?}", self.#field_names);
                        parts.push(format!("{}={}", field_name, value));
                    }
                )*

                Ok(format!("{}({})", #class_name, parts.join(", ")))
            }
        }
    };
    expanded.into()
}

/// ------------------------------------------------------------------------
/// 2) The `register_caches!` function‐like macro
/// ------------------------------------------------------------------------
struct CacheRegisterArgs {
    types: Vec<Path>,
}

impl Parse for CacheRegisterArgs {
    fn parse(input: ParseStream) -> SynResult<Self> {
        // Parse a comma‐separated list of type names
        let types = Punctuated::<Path, Comma>::parse_terminated(input)?
            .into_iter()
            .collect();
        Ok(CacheRegisterArgs { types })
    }
}
/// ## 2) `register_caches!` function‐like macro
///
/// Usage:
/// ```ignore
/// register_caches!(
///     SomeName,
///     OtherName,
/// );
/// ```
///
/// Expands to:
/// 1. `pub enum AnalysisCacheElement { … }` with `SomeNameAnalysis(SomeName)`, …, plus
///    a `#[cfg(feature="py")] PyAnalysis(Py<PyAny>)` variant
/// 2. `impl AnalysisCacheElement { fn clone_py(...) { … } }`
/// 3. (behind `#[cfg(feature="py")]`) the `PyAnalysisCache` newtype + `#[pymethods]` block

#[proc_macro]
pub fn register_caches(input: TokenStream) -> TokenStream {
    let args = parse_macro_input!(input as CacheRegisterArgs);
    let types = args.types; // Vec<Path>

    // 1) Build the "AnalysisCacheElement" enum variants
    //    and collect them into Vecs so we can reuse:
    let mut enum_variants = Vec::new();
    let mut clone_arms = Vec::new();
    let mut accessors = Vec::new();
    let mut element_arms = Vec::new();

    for path in &types {
        // e.g. `SomeName`
        let ident = &path.segments.last().unwrap().ident;
        // variant: `SomeNameAnalysis`
        let var_ident = format_ident!("{}Analysis", ident);
        // kebab‐case key: "some-name"
        let key = {
            let s = ident.to_string();
            let mut out = String::new();
            for (i, ch) in s.chars().enumerate() {
                if ch.is_uppercase() {
                    if i != 0 {
                        out.push('-');
                    }
                    for lo in ch.to_lowercase() {
                        out.push(lo);
                    }
                } else {
                    out.push(ch);
                }
            }
            out
        };
        // snake_case method: `some_name`
        let snake = {
            let s = ident.to_string();
            let mut out = String::new();
            for (i, ch) in s.chars().enumerate() {
                if ch.is_uppercase() {
                    if i != 0 {
                        out.push('_');
                    }
                    for lo in ch.to_lowercase() {
                        out.push(lo);
                    }
                } else {
                    out.push(ch);
                }
            }
            format_ident!("{}", out)
        };

        // enum variant
        enum_variants.push(quote! {
            #var_ident(#ident),
        });
        // clone_py arm
        clone_arms.push(quote! {
            AnalysisCacheElement::#var_ident(v) => AnalysisCacheElement::#var_ident(v.clone()),
        });
        // element->PyObject arm in get_element
        element_arms.push(quote! {
            AnalysisCacheElement::#var_ident(v) => v.clone().into_py_any(py)?,
        });
        // accessor method
        accessors.push(quote! {
            pub fn #snake(&self) -> Option<#ident> {
                if let Some(AnalysisCacheElement::#var_ident(v)) = self.get(#key) {
                    Some(v.clone())
                } else {
                    None
                }
            }
        });
    }

    // plus the PyAnalysis variant & arms
    enum_variants.push(quote! {
        #[cfg(feature = "py")]
        PyAnalysis(pyo3::Py<pyo3::PyAny>),
    });
    clone_arms.push(quote! {
        #[cfg(feature = "py")]
        AnalysisCacheElement::PyAnalysis(v) => AnalysisCacheElement::PyAnalysis(v.clone_ref(py)),
    });
    element_arms.push(quote! {
        #[cfg(feature = "py")]
        AnalysisCacheElement::PyAnalysis(v) => v.clone().into_py_any(py)?,
    });

    // 2) Emit the combined code
    let expanded = quote! {
        #[cfg(feature = "py")]
        use {
        //     derive_more::{Deref, DerefMut},
        //     pyo3::{pyclass, pymethods, IntoPyObjectExt, Py, PyAny, PyObject, PyResult, Python},
            // pyo3::{Py, PyAny},
        };

        /// All possible elements in the cache
        pub enum AnalysisCacheElement {
            // Float(f64),
            // Int(i32),
            // String(String),
            #(#enum_variants)*
        }

        impl AnalysisCacheElement {
            #[cfg(feature = "py")]
            pub fn clone_py(&self, py: pyo3::Python) -> Self {
                match self {
                    #(#clone_arms)*
                }
            }
        }

        impl AnalysisCache {
            #[cfg(feature = "py")]
            pub fn clone_py(&self, py: pyo3::Python) -> Self {
                Self {
                    store: self
                        .store
                        .iter()
                        .map(|(k, v)| (k.clone(), v.clone_py(py)))
                        .collect(),
                    history: self
                        .history
                        .iter()
                        .map(|(k, r, e)| (k.clone(), *r, e.clone_py(py)))
                        .collect(),
                }
            }

            #(#accessors)*
        }


        // // Py‐bindings for the entire cache
        #[cfg(feature = "py")]
        mod py_analysis_cache {
            use pyo3::IntoPyObjectExt;
            use super::*;

            #[pyo3::pyclass(unsendable, name = "AnalysisCache")]
            #[derive(derive_more::Deref, derive_more::DerefMut)]
            pub struct PyAnalysisCache(pub AnalysisCache);

            #[cfg(feature = "py")]
            impl PyAnalysisCache {
                pub fn new(cache: AnalysisCache) -> Self {
                    PyAnalysisCache(cache)
                }
            }

            #[cfg(feature = "py")]
            #[pyo3::pymethods]
            impl PyAnalysisCache {
                fn __getitem__(&self, py: pyo3::Python, key: String) -> pyo3::PyResult<Option<pyo3::PyObject>> {
                    self.get_element(py, key)
                }

                #[pyo3(name = "get")]
                pub fn get_element(&self, py: pyo3::Python, key: String) -> pyo3::PyResult<Option<pyo3::PyObject>> {
                    if let Some(val) = self.get(&key) {
                        Ok(Some(match val {
                            #(#element_arms)*
                        }))
                    } else {
                        Ok(None)
                    }
                }

                #(#accessors)*
            }
        }

        #[cfg(feature = "py")]
        pub use py_analysis_cache::PyAnalysisCache;
    };

    expanded.into()
}

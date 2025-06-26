use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    braced, parse::Parser, parse_macro_input, punctuated::Punctuated, token::Comma, DeriveInput,
    Expr, Fields, Ident, MetaNameValue,
};
use syn::{
    parse::{Parse, ParseStream},
    Path, Result as SynResult, Token,
};

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
    passes:   Vec<Path>,
    specials: Vec<Path>,
    extras:   Vec<Path>,
}

impl Parse for RegisterArgs {
    fn parse(input: ParseStream) -> SynResult<Self> {
        let mut passes:   Option<Vec<Path>> = None;
        let mut specials: Option<Vec<Path>> = None;
        let mut extras:   Option<Vec<Path>> = None;

        // Keep reading `key = { … }` blocks until EOF
        while !input.is_empty() {
            let key: Ident = input.parse()?;
            input.parse::<Token![=]>()?;

            // parse the `{ … }` block
            let content;
            braced!(content in input);
            let list: Vec<Path> = Punctuated::<Path, Comma>::parse_terminated(&content)?
                .into_iter().collect();

            match &*key.to_string() {
                "passes"  => {
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
                "extras"  => {
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

        let passes   = passes.ok_or_else(|| input.error("you must provide `passes = { … }`"))?;
        let specials = specials.unwrap_or_default();
        let extras   = extras.unwrap_or_default();

        Ok(RegisterArgs { passes, specials, extras })
    }
}

#[proc_macro]
pub fn register_pytransformations(input: TokenStream) -> TokenStream {
    let RegisterArgs { passes, specials, extras } =
        parse_macro_input!(input as RegisterArgs);

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
    let mut arm_passes  = Vec::new();
    for path in &passes {
        let ty_ident = &path.segments.last().unwrap().ident;
        let s = ty_ident.to_string();
        let stripped = s
            .strip_prefix("Py").unwrap_or(&s)
            .strip_suffix("Pass").unwrap_or(&s);
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
            pm.py()
              .import("sys")?
              .getattr("modules")?
              .set_item("aqmodels.transformations", m)?;
            Ok(())
        }
    };

    expanded.into()
}

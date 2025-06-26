use proc_macro::TokenStream;
use quote::{format_ident, quote};
use syn::{
    parse_macro_input,
    parse::Parser,
    punctuated::Punctuated,
    token::Comma,
    DeriveInput,
    Fields,
    MetaNameValue,
    Expr,
};

/// Replace the derive with an attribute macro:
#[proc_macro_attribute]
pub fn py_pass(attr: TokenStream, item: TokenStream) -> TokenStream {
    // 1) Parse the struct on which the attribute was placed:
    let input = parse_macro_input!(item as DeriveInput);
    let struct_name = &input.ident;

    // 2) Infer the wrapper and Python class names:
    let wrapper_name = format_ident!("Py{}", struct_name);
    let class_name   = struct_name.to_string();

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
    let pass_variant = pass_variant
        .unwrap_or_else(|| panic!("you must supply `pass_variant = \"...\"`"));

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
        let ty   = &f.ty;
        quote! { #name: #ty }
    });
    let forward = fields.iter().map(|f| {
        let name = &f.ident;
        quote! { #name }
    });
    let field_getters = fields.iter().map(|f| {
        let fname  = f.ident.as_ref().unwrap();
        let getter = format_ident!("get_{}", fname);
        let ty     = &f.ty;
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


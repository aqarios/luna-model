extern crate proc_macro;
use std::cell::RefCell;
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::quote;
use syn::{parse_macro_input, ItemStruct, Ident, Type};

// Metadata: struct name and parsed field types
#[derive(Clone)]
struct StructData {
    name: String,
    fields: Vec<(String, Type)>, // (field name, field type)
}

thread_local! {
    static REGISTERED: RefCell<Vec<StructData>> = RefCell::new(Vec::new());
}

/// Register a struct: record its name and its field types
#[proc_macro_attribute]
pub fn transformation(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let struct_name = input.ident.clone();

    // Collect field data: (ident string, Type)
    let fields = input.fields.iter().filter_map(|f| {
        f.ident.as_ref().map(|ident| (ident.to_string(), f.ty.clone()))
    }).collect::<Vec<_>>();

    // Store in thread-local registry (RefCell avoids Sync requirement)
    REGISTERED.with(|reg| reg.borrow_mut().push(StructData {
        name: struct_name.to_string(),
        fields,
    }));

    // Re-emit the struct unchanged
    TokenStream::from(quote! { #input })
}

/// Generate Python wrappers + AnyPass enum
#[proc_macro]
pub fn generate_anypass(_input: TokenStream) -> TokenStream {
    // Snapshot and clear registry
    let structs = REGISTERED.with(|reg| reg.borrow().clone());
    REGISTERED.with(|reg| reg.borrow_mut().clear());

    if structs.is_empty() {
        return syn::Error::new(
            Span::call_site(),
            "generate_anypass!(): no structs registered via #[register_struct]"
        )
        .to_compile_error()
        .into();
    }

    let mut wrappers = Vec::new();
    let mut variants = Vec::new();
    let mut arms = Vec::new();

    for data in structs {
        let struct_ident = Ident::new(&data.name, Span::call_site());
        let py_ident = Ident::new(&format!("Py{}", data.name), Span::call_site());
        let class_name = &data.name;

        // Constructor args and init from stored Types
        let ctor_args = data.fields.iter().map(|(fname, fty)| {
            let id = Ident::new(fname, Span::call_site());
            quote! { #id: #fty }
        }).collect::<Vec<_>>();
        let ctor_init = data.fields.iter().map(|(fname, _)| {
            let id = Ident::new(fname, Span::call_site());
            quote! { #id }
        }).collect::<Vec<_>>();

        // Getter methods
        let getters = data.fields.iter().map(|(fname, fty)| {
            let method = Ident::new(&format!("get_{}", fname), Span::call_site());
            let id = Ident::new(fname, Span::call_site());
            quote! {
                #[getter]
                pub fn #method(&self) -> &#fty { &self.0.#id }
            }
        }).collect::<Vec<_>>();

        // Wrapper struct + impl
        wrappers.push(quote! {
            #[cfg_attr(feature = "py", pyclass(name = #class_name))]
            #[derive(Deref, DerefMut, Clone)]
            pub struct #py_ident(pub #struct_ident);

            #[pymethods]
            impl #py_ident {
                #[new]
                #[pyo3(signature(#(#ctor_args),*))]
                pub fn py_init(#(#ctor_args),*) -> Self {
                    #py_ident(#struct_ident { #(#ctor_init),* })
                }

                #(#getters)*

                #[getter]
                pub fn get_name(&self) -> String { self.0.name().to_owned() }

                #[getter]
                pub fn get_requires(&self) -> Vec<String> { self.0.requires() }

                fn as_pass(self) -> pyo3::PyResult<crate::Pass> {
                    Ok(crate::Pass::Transformation(Box::new(self.0)))
                }
            }
        });

        // Enum variants and match arms
        variants.push(quote! { #struct_ident(#py_ident) });
        arms.push(quote! { Self::#struct_ident(x) => x.as_pass()? });
    }

    // Build the AnyPass enum + impl
    let enum_def = quote! {
        #[cfg_attr(feature = "py", derive(FromPyObject))]
        #[derive(Clone)]
        pub enum AnyPass { #(#variants),* }

        #[cfg(feature = "py")]
        impl AnyPass {
            pub fn as_pass(self) -> pyo3::PyResult<crate::Pass> {
                Ok(match self { #(#arms),* })
            }
        }
    };

    let expanded = quote! { #(#wrappers)* #enum_def };
    TokenStream::from(expanded)
}

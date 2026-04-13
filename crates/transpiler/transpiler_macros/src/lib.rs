use proc_macro::TokenStream;
use quote::quote;
use syn::{ItemStruct, parse_macro_input};

fn expand(item: TokenStream, ctor: proc_macro2::TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemStruct);
    let ident = &input.ident;
    let (impl_generics, ty_generics, where_clause) = input.generics.split_for_impl();

    quote! {
        #input

        impl #impl_generics From<#ident #ty_generics> for ::lunamodel_transpiler::PipelineStep
        #where_clause
        {
            fn from(value: #ident #ty_generics) -> Self {
                #ctor(::std::sync::Arc::new(value))
            }
        }
    }
    .into()
}

#[proc_macro_attribute]
pub fn transformation(_attr: TokenStream, item: TokenStream) -> TokenStream {
    expand(
        item,
        quote!(::lunamodel_transpiler::PipelineStep::Transform),
    )
}

#[proc_macro_attribute]
pub fn analysis(_attr: TokenStream, item: TokenStream) -> TokenStream {
    expand(item, quote!(::lunamodel_transpiler::PipelineStep::Analysis))
}

#[proc_macro_attribute]
pub fn control_flow(_attr: TokenStream, item: TokenStream) -> TokenStream {
    expand(
        item,
        quote!(::lunamodel_transpiler::PipelineStep::ControlFlow),
    )
}

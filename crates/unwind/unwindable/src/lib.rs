//! Proc-macro support for panic-safe Python-exposed impl blocks.
use proc_macro::TokenStream;
use quote::quote;
use syn::{ImplItem, ItemImpl, ReturnType, Type, TypePath, parse_macro_input, parse_quote};

/// Rewrites every method in an `impl` block so it executes through
/// `lunamodel_unwind::unwind`.
///
/// Methods that already return `PyResult<T>` keep that return type. All other
/// methods are rewritten to return `PyResult<OriginalReturnType>`.
#[proc_macro_attribute]
pub fn unwindable(_attr: TokenStream, item: TokenStream) -> TokenStream {
    let mut imp = parse_macro_input!(item as ItemImpl);

    for item in &mut imp.items {
        if let ImplItem::Fn(method) = item {
            // 1) detect if it's already PyResult<...>
            let already_py = matches!(
                &method.sig.output,
                ReturnType::Type(_, ty)
                    if matches!(&**ty, Type::Path(TypePath { path, .. })
                        if path.segments.last().unwrap().ident == "PyResult"
                    )
            );

            // 2) if not, rewrite -> PyResult<Orig>
            if !already_py {
                let orig_ty = match &method.sig.output {
                    ReturnType::Type(_, ty) => (*ty.clone()).clone(),
                    ReturnType::Default => parse_quote! { () },
                };
                method.sig.output = ReturnType::Type(
                    parse_quote! { -> },
                    Box::new(parse_quote! { pyo3::PyResult<#orig_ty> }),
                );
            }

            // 3) wrap the old body
            let old = &method.block;
            let new_block = if already_py {
                // preserves a method already returning PyResult<...>
                quote!({
                    unwind(|| #old )
                })
            } else {
                // capture the original return value in a local and then Ok(...) it
                quote!({
                    unwind(|| {
                        let __unwind_ret = (|| #old )();
                        Ok(__unwind_ret)
                    })
                })
            };
            method.block = syn::parse2(new_block).unwrap();
        }
    }

    TokenStream::from(quote!(#imp))
}

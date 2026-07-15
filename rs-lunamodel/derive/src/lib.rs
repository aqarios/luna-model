use proc_macro::TokenStream;
use quote::quote;
use syn::{Data, DeriveInput, Fields, Type, parse_macro_input};

/// Derives `from_raw`, `as_ptr` and `as_mut_ptr` for a wrapper sturct that owns a single
/// `handle: *mut T` field.
#[proc_macro_derive(RawHandle)]
pub fn derive_raw_handle(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;

    // Pull `T` out of the `handle: *mut T` field.
    let pointee = match handle_pointee(&input) {
        Some(ty) => ty,
        None => {
            return syn::Error::new_spanned(
                &input,
                "RawHandle requires a struct with a `handle: *mut T` field",
            )
            .to_compile_error()
            .into();
        }
    };

    quote! {
        impl #name {
            /// Wraps a raw handle owned by the caller.
            pub(crate) fn from_raw(handle: *mut #pointee) -> Self {
                Self { handle }
            }

            /// Borrows the raw handle for C interop.
            pub(crate) fn as_ptr(&self) -> *const #pointee {
                self.handle
            }

            /// Borrows the raw handle as mutable for C interop.
            pub(crate) fn as_mut_ptr(&mut self) -> *mut #pointee {
                self.handle
            }
        }

        // Lets `out!` convert a raw `*mut #pointee` out value straight into the
        // wrapper via `lm_try_into()`, so call sites need no mapper closure.
        impl crate::traits::LmTryFrom<*mut #pointee> for #name {
            fn lm_try_from(handle: *mut #pointee) -> crate::Result<Self> {
                if handle.is_null() {
                    Err(crate::Error {
                        status: crate::Status::NullPointer,
                        message: concat!(
                            "the LunaModel host returned a null ",
                            stringify!(#pointee),
                            " handle with a successful status",
                        )
                        .to_owned(),
                    })
                } else {
                    Ok(Self::from_raw(handle))
                }
            }
        }

        unsafe impl crate::ffi::OwnedHandle for #name {
            type Raw = #pointee;

            unsafe fn from_raw_owned_unchecked(handle: *mut Self::Raw) -> Self {
                Self { handle }
            }

            fn into_raw_owned(self) -> *mut Self::Raw {
                let this = ::std::mem::ManuallyDrop::new(self);
                this.handle
            }
        }
    }
    .into()
}

/// Returns `T` from `handle: *mut T` field, if the struct has one.
fn handle_pointee(input: &DeriveInput) -> Option<Type> {
    let Data::Struct(data) = &input.data else {
        return None;
    };
    let Fields::Named(fields) = &data.fields else {
        return None;
    };
    fields.named.iter().find_map(|field| {
        if field.ident.as_ref()? != "handle" {
            return None;
        };
        match &field.ty {
            Type::Ptr(ptr) => Some((*ptr.elem).clone()),
            _ => None,
        }
    })
}

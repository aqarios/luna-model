//! Macros that collapse the FromPyObject / IntoPyObject boilerplate for each
//! wrapper into a single invocation. See `bounds.rs` (capsule shape) and
//! `types/vtype.rs` (enum shape) for representative call sites.
//!
//! `public` is passed as an ident (e.g. `Bounds`); the macro derives both the
//! Python-facing class name (`"Bounds"`) and the `_lm` inner class name
//! (`"PyBounds"`) from it.

/// Declares a Python conversion wrapper around an owning `rs-lunamodel`
/// facade handle.
///
/// The generated conversion transfers only an opaque C handle through a named
/// PyCapsule. Calls on the wrapper therefore continue to dispatch through the
/// installed LunaModel C API and never link or copy the host implementation.
macro_rules! capi_wrapper {
    (
        wrapper: $wrapper:ident,
        public: $public:ident,
        inner: $inner:ty,
        raw: $raw:ty,
        attr: $attr:literal,
        from_py: $from_py:literal $(,)?
    ) => {
        pub struct $wrapper {
            inner: $inner,
            origin: ::core::option::Option<::pyo3::Py<::pyo3::PyAny>>,
        }

        impl $wrapper {
            pub fn new(inner: $inner) -> Self {
                Self {
                    inner,
                    origin: ::core::option::Option::None,
                }
            }

            pub fn inner(&self) -> &$inner {
                &self.inner
            }

            pub fn inner_mut(&mut self) -> &mut $inner {
                &mut self.inner
            }

            pub fn into_inner(self) -> $inner {
                self.inner
            }
        }

        impl ::core::convert::From<$inner> for $wrapper {
            fn from(inner: $inner) -> Self {
                Self::new(inner)
            }
        }

        impl ::core::ops::Deref for $wrapper {
            type Target = $inner;

            fn deref(&self) -> &Self::Target {
                &self.inner
            }
        }

        impl ::core::ops::DerefMut for $wrapper {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.inner
            }
        }

        impl<'a, 'py> ::pyo3::FromPyObject<'a, 'py> for $wrapper {
            type Error = ::pyo3::PyErr;

            fn extract(
                obj: ::pyo3::Borrowed<'a, 'py, ::pyo3::PyAny>,
            ) -> ::core::result::Result<Self, Self::Error> {
                use ::pyo3::types::PyAnyMethods;
                use $crate::utils::TypeCheck;

                $crate::ensure_c_api(obj.py())?;
                obj.check_type(::core::stringify!($public))?;

                let inner = match obj.getattr($attr) {
                    Ok(inner) => inner,
                    Err(_) => obj.to_owned(),
                };
                let capsule = inner
                    .call_method0("_to_c_capsule")?
                    .cast_into::<::pyo3::types::PyCapsule>()?;
                let handle = ::lunamodel_cext_pyffi::take_handle::<$raw>(&capsule)?;
                let inner =
                    unsafe { ::rs_lunamodel::ffi::adopt::<$inner>(handle) }.map_err(|error| {
                        ::pyo3::exceptions::PyValueError::new_err(error.to_string())
                    })?;

                Ok(Self {
                    inner,
                    origin: Some(obj.to_owned().unbind()),
                })
            }
        }

        impl<'py> ::pyo3::IntoPyObject<'py> for $wrapper {
            type Target = ::pyo3::PyAny;
            type Output = ::pyo3::Bound<'py, ::pyo3::PyAny>;
            type Error = ::pyo3::PyErr;

            fn into_pyobject(
                self,
                py: ::pyo3::Python<'py>,
            ) -> ::core::result::Result<Self::Output, Self::Error> {
                use ::pyo3::types::PyAnyMethods;

                let Self { inner, origin } = self;
                if let Some(origin) = origin {
                    // The original Python object observes mutations performed
                    // through the retained synchronized C handle.
                    return Ok(origin.into_bound(py));
                }

                $crate::ensure_c_api(py)?;
                let handle = <$inner as ::rs_lunamodel::ffi::OwnedHandle>::into_raw_owned(inner);
                let capsule = ::lunamodel_cext_pyffi::new_capsule::<$raw>(
                    py,
                    handle,
                    ::rs_lunamodel::ffi::release::<$inner>,
                )?;

                let luna_model = $crate::luna_model(py)?;
                let inner = luna_model
                    .getattr("_lm")?
                    .getattr(::core::concat!("Py", ::core::stringify!($public)))?
                    .call_method1("_from_c_capsule", (capsule,))?;
                luna_model
                    .getattr(::core::stringify!($public))?
                    .call_method1($from_py, (inner,))
            }
        }
    };
}

/// Generates `FromPyObject` and `IntoPyObject` impls for a capsule-shaped
/// wrapper. Requires the wrapper to be a tuple struct `pub struct $wrapper($inner)`,
/// where `$inner` implements `CapsuleFFI<'py, R>` for exactly one `R`
/// (which is inferred).
#[macro_export]
macro_rules! capsule_wrapper {
    (
        wrapper: $wrapper:ident,
        public: $public:ident,
        inner: $inner:ty,
        attr: $attr:literal,
        from_py: $from_py:literal $(,)?
    ) => {
        impl<'a, 'py> ::pyo3::FromPyObject<'a, 'py> for $wrapper {
            type Error = ::pyo3::PyErr;

            fn extract(
                obj: ::pyo3::Borrowed<'a, 'py, ::pyo3::PyAny>,
            ) -> ::std::result::Result<Self, Self::Error> {
                use ::pyo3::types::PyAnyMethods;
                use $crate::utils::TypeCheck;
                obj.check_type(::std::stringify!($public))?;
                let capsule_obj = if let Ok(x) = obj.getattr($attr) {
                    x.call_method0("_to_capsule")
                } else {
                    obj.call_method0("_to_capsule")
                }?;
                Ok($wrapper(
                    <$inner as ::luna_model::python::ffi::CapsuleFFI<'py, _>>::from_capsule(
                        capsule_obj.extract()?,
                    )?,
                ))
            }
        }

        impl<'py> ::pyo3::IntoPyObject<'py> for $wrapper {
            type Target = ::pyo3::PyAny;
            type Output = ::pyo3::Bound<'py, Self::Target>;
            type Error = ::pyo3::PyErr;

            fn into_pyobject(
                self,
                py: ::pyo3::Python<'py>,
            ) -> ::std::result::Result<Self::Output, Self::Error> {
                use ::pyo3::types::PyAnyMethods;
                let capsule =
                    <$inner as ::luna_model::python::ffi::CapsuleFFI<'py, _>>::to_capsule(
                        &self.0, py,
                    )?;
                let lm = $crate::luna_model(py)?;
                let pye = lm
                    .getattr("_lm")?
                    .getattr(::std::concat!("Py", ::std::stringify!($public)))?
                    .call_method1("_from_capsule", (capsule,))?;
                lm.getattr(::std::stringify!($public))?
                    .call_method1($from_py, (pye,))
            }
        }
    };
}

/// Generates `FromPyObject` and `IntoPyObject` impls for an enum-shaped
/// wrapper. Inner is a Rust enum with `From<$bridge>`/`Into<$bridge>` impls;
/// the bridge implements `CapsuleFFI<'py, String>`. The Python-side attribute
/// is always `_val`.
#[macro_export]
macro_rules! enum_wrapper {
    (
        wrapper: $wrapper:ident,
        public: $public:ident,
        inner: $inner:ty,
        bridge: $bridge:ty,
        from_py: $from_py:literal $(,)?
    ) => {
        impl<'a, 'py> ::pyo3::FromPyObject<'a, 'py> for $wrapper {
            type Error = ::pyo3::PyErr;

            fn extract(
                obj: ::pyo3::Borrowed<'a, 'py, ::pyo3::PyAny>,
            ) -> ::std::result::Result<Self, Self::Error> {
                use ::pyo3::types::PyAnyMethods;
                use $crate::utils::TypeCheck;
                obj.check_type(::std::stringify!($public))?;
                let capsule: ::std::string::String = if let Ok(x) = obj.getattr("_val") {
                    x.call_method0("_to_capsule")
                } else {
                    obj.call_method0("_to_capsule")
                }?
                .extract()?;
                let bridge = <$bridge as ::luna_model::python::ffi::CapsuleFFI<
                    'py,
                    ::std::string::String,
                >>::from_capsule(capsule)?;
                Ok($wrapper(<$inner as ::core::convert::From<$bridge>>::from(
                    bridge,
                )))
            }
        }

        impl<'py> ::pyo3::IntoPyObject<'py> for $wrapper {
            type Target = ::pyo3::PyAny;
            type Output = ::pyo3::Bound<'py, Self::Target>;
            type Error = ::pyo3::PyErr;

            fn into_pyobject(
                self,
                py: ::pyo3::Python<'py>,
            ) -> ::std::result::Result<Self::Output, Self::Error> {
                use ::pyo3::types::PyAnyMethods;
                let bridge: $bridge = <$bridge as ::core::convert::From<$inner>>::from(self.0);
                let capsule = <$bridge as ::luna_model::python::ffi::CapsuleFFI<
                    'py,
                    ::std::string::String,
                >>::to_capsule(&bridge, py)?;
                let lm = $crate::luna_model(py)?;
                let pyv = lm
                    .getattr("_lm")?
                    .getattr(::std::concat!("Py", ::std::stringify!($public)))?
                    .call_method1("_from_capsule", (capsule,))?;
                lm.getattr(::std::stringify!($public))?
                    .call_method1($from_py, (pyv,))
            }
        }
    };
}

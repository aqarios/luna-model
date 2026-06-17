//! Macros that collapse the FromPyObject / IntoPyObject boilerplate for each
//! wrapper into a single invocation. See `bounds.rs` (capsule shape) and
//! `types/vtype.rs` (enum shape) for representative call sites.
//!
//! `public` is passed as an ident (e.g. `Bounds`); the macro derives both the
//! Python-facing class name (`"Bounds"`) and the `_lm` inner class name
//! (`"PyBounds"`) from it.

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
                    <$inner as ::lunamodel::python::ffi::CapsuleFFI<'py, _>>::from_capsule(
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
                let capsule = <$inner as ::lunamodel::python::ffi::CapsuleFFI<'py, _>>::to_capsule(
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
                let bridge = <$bridge as ::lunamodel::python::ffi::CapsuleFFI<
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
                let capsule = <$bridge as ::lunamodel::python::ffi::CapsuleFFI<
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

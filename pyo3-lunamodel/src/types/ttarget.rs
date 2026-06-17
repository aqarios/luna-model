use derive_more::Deref;
use lunamodel::python::{PyTranslationTarget as PyTT, TranslationTarget};

#[repr(transparent)]
/// A wrapper around a [`TranslationTarget`] that can be converted to and from python with `pyo3`.
#[derive(Deref)]
pub struct PyTranslationTarget(pub TranslationTarget);

enum_wrapper! {
    wrapper: PyTranslationTarget,
    public: TranslationTarget,
    inner: TranslationTarget,
    bridge: PyTT,
    from_py: "_from_pyttarget",
}

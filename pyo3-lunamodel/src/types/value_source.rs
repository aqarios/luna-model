use derive_more::Deref;
use lunamodel_core::ValueSource;
use lunamodel_python::PyValueSource as PyVS;

#[repr(transparent)]
/// A wrapper around a [`ValueSource`] that can be converted to and from python with `pyo3`.
#[derive(Deref)]
pub struct PyValueSource(pub ValueSource);

enum_wrapper! {
    wrapper: PyValueSource,
    public: ValueSource,
    inner: ValueSource,
    bridge: PyVS,
    from_py: "_from_pysrc",
}

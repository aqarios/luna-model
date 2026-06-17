use derive_more::Deref;
use luna_model::python::PyCtype as PyC;
use luna_model::types::Ctype;

#[repr(transparent)]
/// A wrapper around a [`Ctype`] that can be converted to and from python with `pyo3`.
#[derive(Deref)]
pub struct PyCtype(pub Ctype);

enum_wrapper! {
    wrapper: PyCtype,
    public: Ctype,
    inner: Ctype,
    bridge: PyC,
    from_py: "_from_pyctype",
}

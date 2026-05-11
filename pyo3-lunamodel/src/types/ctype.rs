use derive_more::Deref;
use lunamodel_python::PyCtype as PyC;
use lunamodel_types::Ctype;

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

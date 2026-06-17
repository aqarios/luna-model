use derive_more::Deref;
use luna_model::python::PyVtype as PyV;
use luna_model::types::Vtype;

#[repr(transparent)]
/// A wrapper around a [`Vtype`] that can be converted to and from python with `pyo3`.
#[derive(Deref)]
pub struct PyVtype(pub Vtype);

enum_wrapper! {
    wrapper: PyVtype,
    public: Vtype,
    inner: Vtype,
    bridge: PyV,
    from_py: "_from_pyvtype",
}

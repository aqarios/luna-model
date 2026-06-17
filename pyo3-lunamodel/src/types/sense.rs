use derive_more::Deref;
use luna_model::python::PySense as PyS;
use luna_model::types::Sense;

#[repr(transparent)]
/// A wrapper around a [`Sense`] that can be converted to and from python with `pyo3`.
#[derive(Deref)]
pub struct PySense(pub Sense);

enum_wrapper! {
    wrapper: PySense,
    public: Sense,
    inner: Sense,
    bridge: PyS,
    from_py: "_from_pysense",
}

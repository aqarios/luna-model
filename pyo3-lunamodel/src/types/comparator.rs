use derive_more::Deref;
use luna_model::python::PyComparator as PyCmp;
use luna_model::types::Comparator;

#[repr(transparent)]
/// A wrapper around a [`Comparator`] that can be converted to and from python with `pyo3`.
#[derive(Deref)]
pub struct PyComparator(pub Comparator);

enum_wrapper! {
    wrapper: PyComparator,
    public: Comparator,
    inner: Comparator,
    bridge: PyCmp,
    from_py: "_from_pycmp",
}

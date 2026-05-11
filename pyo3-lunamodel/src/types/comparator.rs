use derive_more::Deref;
use lunamodel_python::PyComparator as PyCmp;
use lunamodel_types::Comparator;

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

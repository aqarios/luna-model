use luna_model::python::prelude::PyConstraintCollectionContent as PyCCC;

#[repr(transparent)]
/// A wrapper around a [`PyCCC`] that can be converted to and from python with `pyo3`.
pub struct PyConstraintCollection(pub PyCCC);

capsule_wrapper! {
    wrapper: PyConstraintCollection,
    public: ConstraintCollection,
    inner: PyCCC,
    attr: "_cc",
    from_py: "_from_pycc",
}

use luna_model::core::prelude::VarRef;

#[repr(transparent)]
/// A wrapper around a [`VarRef`] that can be converted to and from python with `pyo3`.
pub struct PyVariable(pub VarRef);

capsule_wrapper! {
    wrapper: PyVariable,
    public: Variable,
    inner: VarRef,
    attr: "_v",
    from_py: "_from_pyvar",
}

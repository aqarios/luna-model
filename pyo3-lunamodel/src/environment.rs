use luna_model::core::prelude::ArcEnv;

#[repr(transparent)]
/// A wrapper around a [`ArcEnv`] that can be converted to and from python with `pyo3`.
pub struct PyEnvironment(pub ArcEnv);

capsule_wrapper! {
    wrapper: PyEnvironment,
    public: Environment,
    inner: ArcEnv,
    attr: "_env",
    from_py: "_from_pyenv",
}

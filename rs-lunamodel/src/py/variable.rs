//! C-API-backed PyO3 variable conversion.

capi_wrapper! {
    wrapper: PyVariable,
    public: Variable,
    inner: crate::VarRef,
    raw: ::lunamodel_capi::LmVref,
    attr: "_v",
    from_py: "_from_pyvar",
}

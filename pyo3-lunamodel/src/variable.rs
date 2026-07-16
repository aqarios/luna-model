//! C-API-backed PyO3 variable conversion.

capi_wrapper! {
    wrapper: PyVariable,
    public: Variable,
    inner: ::rs_lunamodel::VarRef,
    raw: ::lunamodel_cext_vtable::LmVref,
    attr: "_v",
    from_py: "_from_pyvar",
}

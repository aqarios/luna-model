//! C-API-backed PyO3 solution conversion.

capi_wrapper! {
    wrapper: PySolution,
    public: Solution,
    inner: ::rs_lunamodel::Solution,
    raw: ::lunamodel_cext_vtable::LmSol,
    attr: "_s",
    from_py: "_from_pys",
}

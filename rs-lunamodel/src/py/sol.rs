//! C-API-backed PyO3 solution conversion.

capi_wrapper! {
    wrapper: PySolution,
    public: Solution,
    inner: crate::Solution,
    raw: ::lunamodel_capi::LmSol,
    attr: "_s",
    from_py: "_from_pys",
}

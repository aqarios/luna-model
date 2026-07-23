//! C-API-backed PyO3 bounds conversion.

capi_wrapper! {
    wrapper: PyBounds,
    public: Bounds,
    inner: ::rs_lunamodel::LazyBounds,
    raw: ::lunamodel_capi::LmLazyBounds,
    attr: "_b",
    from_py: "_from_pyb",
}

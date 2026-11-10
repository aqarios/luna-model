//! C-API-backed PyO3 bounds conversion.

capi_wrapper! {
    wrapper: PyBounds,
    public: Bounds,
    inner: crate::LazyBounds,
    raw: ::lunamodel_capi::LmLazyBounds,
    attr: "_b",
    from_py: "_from_pyb",
}

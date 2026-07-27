//! C-API-backed PyO3 model-specs conversion.

capi_wrapper! {
    wrapper: PyModelSpecs,
    public: ModelSpecs,
    inner: crate::Specs,
    raw: ::lunamodel_capi::LmSpecs,
    attr: "_sp",
    from_py: "_from_pysp",
}

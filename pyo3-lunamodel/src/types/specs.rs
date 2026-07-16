//! C-API-backed PyO3 model-specs conversion.

capi_wrapper! {
    wrapper: PyModelSpecs,
    public: ModelSpecs,
    inner: ::rs_lunamodel::Specs,
    raw: ::lunamodel_cext_vtable::LmSpecs,
    attr: "_sp",
    from_py: "_from_pysp",
}

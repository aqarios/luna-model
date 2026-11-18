//! C-API-backed PyO3 model conversion.

capi_wrapper! {
    wrapper: PyModel,
    public: Model,
    inner: crate::Model,
    raw: ::lunamodel_capi::LmModel,
    attr: "_m",
    from_py: "_from_pym",
}

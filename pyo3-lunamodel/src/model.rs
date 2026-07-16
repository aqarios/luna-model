//! C-API-backed PyO3 model conversion.

capi_wrapper! {
    wrapper: PyModel,
    public: Model,
    inner: ::rs_lunamodel::Model,
    raw: ::lunamodel_cext_vtable::LmModel,
    attr: "_m",
    from_py: "_from_pym",
}

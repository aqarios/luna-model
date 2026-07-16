//! C-API-backed PyO3 constraint-collection conversion.

capi_wrapper! {
    wrapper: PyConstraintCollection,
    public: ConstraintCollection,
    inner: ::rs_lunamodel::ConstraintCollection,
    raw: ::lunamodel_cext_vtable::LmCC,
    attr: "_cc",
    from_py: "_from_pycc",
}

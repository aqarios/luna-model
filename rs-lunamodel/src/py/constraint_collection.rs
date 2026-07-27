//! C-API-backed PyO3 constraint-collection conversion.

capi_wrapper! {
    wrapper: PyConstraintCollection,
    public: ConstraintCollection,
    inner: crate::ConstraintCollection,
    raw: ::lunamodel_capi::LmCC,
    attr: "_cc",
    from_py: "_from_pycc",
}

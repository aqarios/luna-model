//! C-API-backed PyO3 constraint conversion.

capi_wrapper! {
    wrapper: PyConstraint,
    public: Constraint,
    inner: crate::Constraint,
    raw: ::lunamodel_capi::LmConstraint,
    attr: "_c",
    from_py: "_from_pyc",
}

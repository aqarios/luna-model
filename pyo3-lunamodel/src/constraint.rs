//! C-API-backed PyO3 constraint conversion.

capi_wrapper! {
    wrapper: PyConstraint,
    public: Constraint,
    inner: ::rs_lunamodel::Constraint,
    raw: ::lunamodel_cext_vtable::LmConstraint,
    attr: "_c",
    from_py: "_from_pyc",
}

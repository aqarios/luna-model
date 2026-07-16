//! C-API-backed PyO3 expression conversion.

capi_wrapper! {
    wrapper: PyExpression,
    public: Expression,
    inner: ::rs_lunamodel::Expression,
    raw: ::lunamodel_cext_vtable::LmExpr,
    attr: "_expr",
    from_py: "_from_pyexpr",
}

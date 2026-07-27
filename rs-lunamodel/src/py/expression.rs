//! C-API-backed PyO3 expression conversion.

capi_wrapper! {
    wrapper: PyExpression,
    public: Expression,
    inner: crate::Expression,
    raw: ::lunamodel_capi::LmExpr,
    attr: "_expr",
    from_py: "_from_pyexpr",
}

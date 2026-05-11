use lunamodel_python::prelude::PyExprContent;

#[repr(transparent)]
pub struct PyExpression(pub PyExprContent);

capsule_wrapper! {
    wrapper: PyExpression,
    public: Expression,
    inner: PyExprContent,
    attr: "_expr",
    from_py: "_from_pyexpr",
}

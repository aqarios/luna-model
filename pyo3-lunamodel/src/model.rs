use lunamodel_python::prelude::PyModelContent;

#[repr(transparent)]
pub struct PyModel(pub PyModelContent);

capsule_wrapper! {
    wrapper: PyModel,
    public: Model,
    inner: PyModelContent,
    attr: "_m",
    from_py: "_from_pym",
}

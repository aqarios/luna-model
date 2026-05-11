use lunamodel_types::Specs;

#[repr(transparent)]
pub struct PyModelSpecs(pub Specs);

capsule_wrapper! {
    wrapper: PyModelSpecs,
    public: ModelSpecs,
    inner: Specs,
    attr: "_sp",
    from_py: "_from_pysp",
}

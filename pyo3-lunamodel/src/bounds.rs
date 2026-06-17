use lunamodel::python::PyBoundsContent;

#[repr(transparent)]
/// A wrapper around [`PyBoundsContent`] that can be converted to and from python with `pyo3`.
pub struct PyBounds(pub PyBoundsContent);

capsule_wrapper! {
    wrapper: PyBounds,
    public: Bounds,
    inner: PyBoundsContent,
    attr: "_b",
    from_py: "_from_pyb",
}

#[cfg(feature = "py")]
use pyo3::prelude::*;

#[cfg_attr(feature = "py", pyclass(subclass))]
#[derive(Clone)]
struct Test {
    data: usize,
}

#[cfg(feature = "py")]
#[pymethods]
impl Test {
    #[new]
    fn new(data: usize) -> Self {
        Self { data }
    }

    fn __add__(&self, py: Python, other: PyObject) -> PyResult<Test> {
        let t_rhs: Test = other.extract(py)?;
        unimplemented!()
    }
}

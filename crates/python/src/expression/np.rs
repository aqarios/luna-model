use pyo3::{
    Bound, Py, PyAny, PyResult, pymethods, types::{PyAnyMethods, PyDict, PyTuple}
};

use super::PyExpression;

#[pymethods]
impl PyExpression {
    fn __array_ufunc__(
        &self,
        ufunc: &Bound<'_, PyAny>,
        method: &str,
        inputs: &Bound<'_, PyTuple>,
        kwargs: Option<&Bound<'_, PyDict>>,
    ) -> PyResult<Py<PyAny>> {
        // Handle comparison operations
        let ufunc_name = ufunc.getattr("__name__")?.extract::<String>()?;
        todo!("{}, {}", ufunc_name, method)
        
        // match ufunc_name.as_str() {
        //     "less_equal" => {
        //         // Custom logic for <=
        //         Ok(PyObject::extract(...))
        //     },
        //     _ => Ok(Python::with_gil(|py| py.NotImplemented()).into()),
        // }
    }
}

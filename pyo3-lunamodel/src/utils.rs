use pyo3::{
    PyErr,
    exceptions::PyValueError,
    types::{PyAnyMethods, PyTypeMethods},
};

pub trait TypeCheck {
    fn check_type(&self, base: &str) -> Result<(), PyErr>;
}

impl<'a, 'py> TypeCheck for pyo3::Borrowed<'a, 'py, pyo3::PyAny> {
    fn check_type(&self, base: &str) -> Result<(), PyErr> {
        let typestr = &self.get_type().name()?.to_string();
        if !(typestr == base || typestr == &format!("Py{base}")) {
            return Err(PyValueError::new_err(format!(
                "Argument must be of type '{base}' or 'Py{base}'. Found '{typestr}'."
            )));
        }
        Ok(())
    }
}

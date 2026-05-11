use pyo3::{
    PyErr,
    exceptions::PyValueError,
    types::{PyAnyMethods, PyTypeMethods},
};

pub trait TypeCheck {
    fn check_type(&self, base: &str) -> Result<(), PyErr>;
    fn check_type_literal(&self, base: &str) -> Result<(), PyErr>;
}

impl<'a, 'py> TypeCheck for pyo3::Borrowed<'a, 'py, pyo3::PyAny> {
    fn check_type(&self, base: &str) -> Result<(), PyErr> {
        let typestr = self.get_type().name()?.to_string();
        if !(typestr == base || typestr == format!("Py{base}")) {
            return Err(PyValueError::new_err(format!(
                "Argument must be of type '{base}' or 'Py{base}'. Found '{typestr}'."
            )));
        }
        Ok(())
    }

    fn check_type_literal(&self, base: &str) -> Result<(), PyErr> {
        let typestr = self.get_type().name()?.to_string();
        if typestr != "type" {
            return Err(PyValueError::new_err(format!(
                "Argument must be the type '{base}' or 'Py{base}'. Found instance of '{typestr}'."
            )));
        }

        let namestr: String = self.getattr("__name__")?.extract()?;
        if !(namestr == base || namestr == format!("Py{base}")) {
            return Err(PyValueError::new_err(format!(
                "Argument must be the type '{base}' or 'Py{base}'. Found type '{namestr}'."
            )));
        }
        Ok(())
    }
}

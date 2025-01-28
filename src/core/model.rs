use std::ops::{AddAssign, MulAssign, SubAssign};

use pyo3::exceptions::PyRuntimeError;
#[cfg(feature = "py")]
use pyo3::prelude::*;

use super::{constraint::Constraints, expression::Expression};

#[cfg_attr(feature = "py", pyclass)]
pub struct Model {
    pub name: String,
    pub objective: Expression,
    pub constraints: Constraints,
    // pub variables: VariableStorage,
}

impl Model {
    fn new(name: Option<String>) -> Self {
        Self {
            name: name.unwrap_or(String::from("unnamed")),
            constraints: Constraints::empty(),
            objective: Expression::empty(),
            // variables: VariableStorage::empty(),
        }
    }
}

impl AddAssign<f64> for Model {
    fn add_assign(&mut self, rhs: f64) {
        self.objective += rhs
    }
}

impl SubAssign<f64> for Model {
    fn sub_assign(&mut self, rhs: f64) {
        self.objective -= rhs
    }
}

impl MulAssign<f64> for Model {
    fn mul_assign(&mut self, rhs: f64) {
        self.objective *= rhs
    }
}

// Python glue code
#[cfg(feature = "py")]
#[pymethods]
impl Model {
    #[new]
    #[pyo3(signature=(name=None))]
    fn py_new(name: Option<String>) -> PyResult<Self> {
        Ok(Self::new(name))
    }

    #[getter(name)]
    fn get_name(&self) -> PyResult<&String> {
        Ok(&self.name)
    }

    fn __iadd__(&mut self, py: Python, other: PyObject) -> PyResult<()> {
        if let Ok(value) = other.extract::<f64>(py) {
            *self += value;
            // println!("{}", value);
            Ok(())
        } else {
            Err(PyRuntimeError::new_err("other type not recognized"))
        }
    }

    fn __isub__(&mut self, py: Python, other: PyObject) -> PyResult<()> {
        if let Ok(value) = other.extract::<f64>(py) {
            *self -= value;
            Ok(())
        } else {
            Err(PyRuntimeError::new_err("other type not recognized"))
        }
    }

    fn __imul__(&mut self, py: Python, other: PyObject) -> PyResult<()> {
        if let Ok(value) = other.extract::<f64>(py) {
            *self *= value;
            Ok(())
        } else {
            Err(PyRuntimeError::new_err("other type not recognized"))
        }
    }

    fn __str__(&self) -> String {
        format!(
            "Model: {}\nobjective: todo",
            self.name,
            // self.objective.make_string()
        )
    }
}

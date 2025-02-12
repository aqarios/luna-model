// use pyo3::exceptions::PyRuntimeError;
#[cfg(feature = "py")]
use pyo3::prelude::*;

use super::{Environment, Expression};

#[cfg_attr(feature = "py", pyclass)]
pub struct Model {
    pub name: String,
    pub objective: Expression,
    // a model has it's own environment. This allows us to define
    // the operations more easily on the model. Getting rid of the
    // problems involving environment passing for multiplication etc.
    pub environment: Environment,
    // pub constraints: Constraints,
    // pub variables: VariableStorage,
}

impl Model {
    pub fn new(name: Option<String>) -> Self {
        let environment = Environment::new();
        Self {
            name: name.unwrap_or(String::from("unnamed")),
            objective: Expression::empty(environment.id),
            environment,
            // constraints: Constraints::empty(),
            // variables: VariableStorage::empty(),
        }
    }
}

#[cfg(feature = "py")]
#[pymethods]
impl Model {
    fn __str__(&self) -> String {
        self.objective.as_string(&self.environment)
    }
}

// impl Addition<f64> for Model {
//     fn add_assign(&mut self, rhs: &f64) {
//         self.objective.add_assign(rhs);
//     }
// }
//
// impl Subtraction<f64> for Model {
//     fn sub_assign(&mut self, rhs: &f64) {
//         self.objective.sub_assign(rhs);
//     }
// }
//
// impl Multiplication<f64> for Model {
//     fn mul_assign(&mut self, rhs: &f64) {
//         // self.objective.mul_assign(rhs)
//         unimplemented!()
//     }
// }
//
// // impl AddAssign<f64> for Model {
// //     fn add_assign(&mut self, rhs: f64) {
// //         self.objective.add_assign(rhs)
// //     }
// // }
// //
// // impl SubAssign<f64> for Model {
// //     fn sub_assign(&mut self, rhs: f64) {
// //         self.objective.sub_assign(rhs);
// //     }
// // }
// //
// // impl MulAssign<f64> for Model {
// //     fn mul_assign(&mut self, rhs: f64) {
// //         self.objective *= rhs
// //     }
// // }
//
// // Python glue code
// #[cfg(feature = "py")]
// #[pymethods]
// impl Model {
//     #[new]
//     #[pyo3(signature=(name=None))]
//     fn py_new(name: Option<String>) -> PyResult<Self> {
//         Ok(Self::new(name))
//     }
//
//     #[getter(name)]
//     fn get_name(&self) -> PyResult<&String> {
//         Ok(&self.name)
//     }
//
//     fn __iadd__(&mut self, py: Python, other: PyObject) -> PyResult<()> {
//         if let Ok(value) = &other.extract::<f64>(py) {
//             Ok(self.add_assign(value))
//         } else {
//             Err(PyRuntimeError::new_err("other type not recognized"))
//         }
//     }
//
//     fn __isub__(&mut self, py: Python, other: PyObject) -> PyResult<()> {
//         if let Ok(value) = &other.extract::<f64>(py) {
//             Ok(self.sub_assign(value))
//         } else {
//             Err(PyRuntimeError::new_err("other type not recognized"))
//         }
//     }
//
//     fn __imul__(&mut self, py: Python, other: PyObject) -> PyResult<()> {
//         if let Ok(value) = &other.extract::<f64>(py) {
//             Ok(self.mul_assign(value))
//         } else {
//             Err(PyRuntimeError::new_err("other type not recognized"))
//         }
//     }
//
//     fn __str__(&self) -> String {
//         format!(
//             "Model: {}\nobjective: todo",
//             self.name,
//             // self.objective.make_string()
//         )
//     }
// }

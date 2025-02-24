use std::cell::RefCell;
use std::rc::Rc;

use super::environment::add_variable;
use super::expression::{
    BiasConstraints, ExpressionBase, ExpressionBaseInternal, IndexConstraints,
};
use super::{Environment, Expression, Vtype};

pub struct Model<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    pub name: String,
    pub objective: Expression<Index, Bias>,
    // a model has it's own environment. This allows us to define
    // the operations more easily on the model. Getting rid of the
    // problems involving environment passing for multiplication etc.
    pub environment: Rc<RefCell<Environment<Index>>>,
    // pub constraints: Constraints,
    // pub variables: VariableStorage,
}

impl<Index, Bias> Model<Index, Bias>
where
    Index: IndexConstraints,
    Bias: BiasConstraints,
{
    pub fn new(name: Option<String>) -> Self {
        let rcenv = Rc::new(RefCell::new(Environment::new()));
        Self {
            name: name.unwrap_or(String::from("unnamed")),
            objective: Expression::new(rcenv.clone()),
            environment: rcenv,
        }
    }

    pub fn new_from_dense(
        name: Option<String>,
        dense: &[Bias],
        num_variables: Index,
        vtype: Vtype,
    ) -> Self {
        let mut model = Model::new(name);
        // We also need to add the varaibles to the model...
        (0..num_variables.into()).into_iter().for_each(|idx| {
            let _ = add_variable(
                model.environment.clone(),
                &idx.to_string(),
                Some(&vtype),
                None,
            );
        });

        model.objective.resize(num_variables);
        model
            .objective
            .add_quadratic_from_dense(dense, num_variables);
        model
    }
}

// impl PyModel {
//     pub fn new_from_dense(name: Option<String>, dense: &[f64], num_variables: VarId) -> Self {
//         let inner = Model::new_from_dense(name, dense, num_variables);
//         PyModel { inner }
//     }
// }

// #[cfg(feature = "py")]
// #[pymethods]
// impl PyModel {
//     // fn __str__(&self) -> String {
//     //     self.objective.as_string(&self.environment)
//     // }
// }

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

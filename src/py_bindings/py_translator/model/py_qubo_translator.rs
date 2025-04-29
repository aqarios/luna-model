use std::rc::Rc;

use crate::core::environment::get_vrefs_in_order;
use crate::core::{ConcreteRcVarRef, ConcreteVarRef};
use crate::py_bindings::py_model::PyModel;
use crate::py_bindings::py_var::PyVariable;
use crate::{core::Vtype, translator::MatrixTranslator};
use numpy::{PyArray2, PyArrayMethods, PyReadonlyArray2, PyUntypedArrayMethods, ToPyArray};
use pyo3::prelude::*;

#[pyclass(unsendable, name = "Qubo", module = "aqmodels.translator")]
pub struct PyQubo {
    matrix_flat: Vec<f64>,
    vars_num: usize,
    offset: f64,
    variables: Vec<ConcreteRcVarRef>,
}

impl PyQubo {
    fn new(
        matrix_flat: Vec<f64>,
        vars_num: usize,
        offset: f64,
        variables: Vec<ConcreteVarRef>,
    ) -> Self {
        Self {
            matrix_flat,
            vars_num,
            offset,
            variables: variables.into_iter().map(|v| Rc::new(v)).collect()
        }
    }
}

#[pymethods]
impl PyQubo {
    #[getter(matrix)]
    fn get_np_matrix<'a>(&self, py: Python<'a>) -> PyResult<Bound<'a, PyArray2<f64>>> {
        Ok(self
            .matrix_flat
            .to_pyarray(py)
            .reshape((self.vars_num, self.vars_num))?)
    }

    #[getter(variable_ordering)]
    fn get_variable_ordering(&self) -> Vec<PyVariable> {
        self.variables.iter().map(|vr| {
            PyVariable(Rc::clone(vr))
        }).collect()
    }

    #[getter(offset)]
    fn get_offset(&self) -> f64 {
        self.offset
    }
}

#[pyclass(unsendable, name = "QuboTranslator", module = "aqmodels.translator")]
pub struct PyQuboTranslator {}

#[pymethods]
impl PyQuboTranslator {
    #[staticmethod]
    #[pyo3(signature=(qubo, name=None, vtype=None))]
    fn to_aq(qubo: PyReadonlyArray2<f64>, name: Option<String>, vtype: Option<Vtype>) -> PyModel {
        let dense = qubo.as_slice().expect("failed to convert to slice");
        PyModel(MatrixTranslator::model_from_dense(
            name,
            dense,
            qubo.shape()[0].into(),
            vtype.unwrap_or(Vtype::Binary),
        ))
    }

    #[staticmethod]
    #[pyo3(signature=(model))]
    fn from_aq(model: &PyModel) -> PyResult<PyQubo> {
        let (vec, nvars) = MatrixTranslator::model_to_dense(&model.0)?;
        Ok(PyQubo::new(
            vec,
            nvars,
            model.objective.borrow().offset,
            get_vrefs_in_order(Rc::clone(&model.environment)),
        ))
    }
}

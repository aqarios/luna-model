use std::rc::Rc;

use crate::core::{Expression, ExpressionBase, VarId};

use derive_more::{Deref, DerefMut};
use pyo3::prelude::*;

use super::py_var::PyVariable;

#[pyclass(unsendable, name = "Expression")]
#[derive(Deref, DerefMut, Clone)]
pub struct PyExpression(pub Rc<Expression<VarId, f64>>);

#[pymethods]
impl PyExpression {
    fn get_linear(&self, var: &PyVariable) -> f64 {
        self.linear(var.id)
    }

    #[pyo3(name = "num_variables")]
    fn get_num_variables(&self) -> usize {
        self.num_variables()
    }

    fn __add__(&self, py: Python, other: PyObject) -> PyResult<PyExpression> {
        todo!()
    }
    fn __radd__(&self, py: Python, other: PyObject) -> PyResult<PyExpression> {
        todo!()
    }
    fn __sub__(&self, py: Python, other: PyObject) -> PyResult<PyExpression> {
        todo!()
    }
    fn __rsub__(&self, py: Python, other: PyObject) -> PyResult<PyExpression> {
        todo!()
    }
    fn __mul__(&self, py: Python, other: PyObject) -> PyResult<PyExpression> {
        todo!()
    }
    fn __rmul__(&self, py: Python, other: PyObject) -> PyResult<PyExpression> {
        todo!()
    }
    // In place assignment
    fn __iadd__(&mut self, py: Python, other: PyObject) {
        todo!()
    }
    fn __isub__(&mut self, py: Python, other: PyObject) {
        todo!()
    }
    fn __imul__(&mut self, py: Python, other: PyObject) {
        todo!()
    }
    // Unary operations
    fn __pos__(&mut self) {
        todo!()
    }
    fn __new__(&mut self) {
        todo!()
    }
    // Comparison
    fn __eq__(&self, other: &Self) -> bool {
        todo!()
    }
    fn __ne__(&self, other: &Self) -> bool {
        todo!()
    }
}

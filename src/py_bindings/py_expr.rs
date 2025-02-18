use crate::core::{Expression, VarId};

use derive_more::{Deref, DerefMut};
use pyo3::prelude::*;

#[pyclass(unsendable, name = "Expression")]
#[derive(Deref, DerefMut)]
pub struct PyExpression(pub Expression<VarId, f64>);

#[pymethods]
impl PyExpression {
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
    // check
    // fn __pow__(&self, other: usize) -> PyResult<PyExpression> {
    //     todo!()
    // }
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
    // fn __ipow__(&mut self, other: usize) {
    //     todo!()
    // }
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

use crate::core::{
    ConcreteAssignmentTypes, ConcreteBias, ConcreteIndex, ResultIterator, ResultView,
    SampleIterator, Solution, VarAssignment,
};
use crate::py_bindings::py_timing::PyTiming;
use derive_more::{Deref, DerefMut};
use numpy::{PyArray1, ToPyArray};
use pyo3::prelude::*;
use std::rc::Rc;

#[pyclass(unsendable, name = "Assignment")]
#[derive(Deref, DerefMut)]
pub struct PyVarAssignment(VarAssignment<ConcreteAssignmentTypes>);

#[pyclass(unsendable, name = "ResultView")]
#[derive(Deref, DerefMut)]
pub struct PyResultView(ResultView<ConcreteIndex, ConcreteBias, ConcreteAssignmentTypes>);

#[pyclass(unsendable, name = "Results")]
#[derive(Deref, DerefMut)]
pub struct PyResultIterator(ResultIterator<ConcreteIndex, ConcreteBias, ConcreteAssignmentTypes>);

#[pyclass(unsendable, name = "Sample")]
#[derive(Deref, DerefMut)]
pub struct PySampleIterator(SampleIterator<ConcreteIndex, ConcreteBias, ConcreteAssignmentTypes>);

#[pyclass(unsendable, name = "Solution")]
#[derive(Deref, DerefMut)]
pub struct PySolution(pub Rc<Solution<ConcreteBias, ConcreteAssignmentTypes>>);

impl Into<ResultView<ConcreteIndex, ConcreteBias, ConcreteAssignmentTypes>> for PyResultView {
    fn into(self) -> ResultView<ConcreteIndex, ConcreteBias, ConcreteAssignmentTypes> {
        self.0
    }
}

impl Into<ResultIterator<ConcreteIndex, ConcreteBias, ConcreteAssignmentTypes>>
    for PyResultIterator
{
    fn into(self) -> ResultIterator<ConcreteIndex, ConcreteBias, ConcreteAssignmentTypes> {
        self.0
    }
}

impl Into<SampleIterator<ConcreteIndex, ConcreteBias, ConcreteAssignmentTypes>>
    for PySampleIterator
{
    fn into(self) -> SampleIterator<ConcreteIndex, ConcreteBias, ConcreteAssignmentTypes> {
        self.0
    }
}

impl Into<Solution<ConcreteBias, ConcreteAssignmentTypes>> for PySolution {
    fn into(self) -> Rc<Solution<ConcreteBias, ConcreteAssignmentTypes>> {
        self.0
    }
}

impl PySolution {
    pub fn iter<Idx>(&self) -> PyResultIterator {
        PyResultIterator(ResultIterator::new(Rc::clone(&self)))
    }
}

#[pymethods]
impl PySolution {
    #[getter]
    fn results<'a>(&self) -> PyResultIterator {
        self.iter::<ConcreteIndex>()
    }

    #[getter]
    fn obj_values<'a>(&self, py: Python<'a>) -> Bound<'a, PyArray1<ConcreteBias>> {
        self.obj_values.to_pyarray(py)
    }

    #[getter]
    fn num_occurrences<'a>(&self, py: Python<'a>) -> Bound<'a, PyArray1<usize>> {
        self.num_occurrences.to_pyarray(py)
    }

    #[getter]
    fn runtime(&self) -> Option<PyTiming> {
        self.timing.map(|t| PyTiming(t))
    }

    // TODO: implement human-readable solution representation
    fn __str__(&self) -> String {
        format!("{:?}", self.0)
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0)
    }

    fn __iter__(slf: PyRef<'_, Self>) -> PyResultIterator {
        slf.iter::<ConcreteIndex>()
    }
}

#[pymethods]
impl PyResultView {
    #[getter]
    fn sample(&self) -> PySampleIterator {
        PySampleIterator(self.iter())
    }

    #[getter]
    fn obj_value(&self) -> Option<ConcreteBias> {
        self.0.obj_value()
    }

    #[getter]
    fn constraint_satisfaction<'a>(&self, py: Python<'a>) -> Option<Bound<'a, PyArray1<bool>>> {
        match self.0.constraint_satisfaction() {
            None => None,
            Some(c) => Some(c.to_pyarray(py)),
        }
    }

    #[getter]
    fn feasible(&self) -> Option<bool> {
        self.0.feasible()
    }
}

#[pymethods]
impl PySampleIterator {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<PyVarAssignment> {
        slf.next().map(|res| PyVarAssignment(res))
    }
}

#[pymethods]
impl PyResultIterator {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<PyResultView> {
        slf.next().map(|res| PyResultView(res))
    }
}

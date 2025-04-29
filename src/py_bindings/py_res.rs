use crate::core::{ConcreteAssignmentTypes, ConcreteBias, OwnedResult, ResultIterator, ResultView};
use crate::py_bindings::py_sample::PySample;
use derive_more::{Deref, DerefMut};
use numpy::{PyArray1, ToPyArray};
use pyo3::{pyclass, pymethods, Bound, PyRef, PyRefMut, Python};

#[pyclass(unsendable, name = "ResultView", module = "aqmodels")]
#[derive(Deref, DerefMut)]
pub struct PyResultView(pub ResultView<ConcreteBias, ConcreteAssignmentTypes>);

#[pyclass(unsendable, name = "Result", module = "aqmodels")]
#[derive(Deref, DerefMut)]
pub struct PyOwnedResult(pub OwnedResult<ConcreteBias, ConcreteAssignmentTypes>);

#[pyclass(unsendable, name = "ResultIterator", module = "aqmodels")]
#[derive(Deref, DerefMut)]
pub struct PyResultIterator(pub ResultIterator<ConcreteBias, ConcreteAssignmentTypes>);

impl Into<ResultView<ConcreteBias, ConcreteAssignmentTypes>> for PyResultView {
    fn into(self) -> ResultView<ConcreteBias, ConcreteAssignmentTypes> {
        self.0
    }
}

impl Into<OwnedResult<ConcreteBias, ConcreteAssignmentTypes>> for PyOwnedResult {
    fn into(self) -> OwnedResult<ConcreteBias, ConcreteAssignmentTypes> {
        self.0
    }
}

impl Into<ResultIterator<ConcreteBias, ConcreteAssignmentTypes>> for PyResultIterator {
    fn into(self) -> ResultIterator<ConcreteBias, ConcreteAssignmentTypes> {
        self.0
    }
}

#[pymethods]
impl PyResultView {
    fn __str__(&self) -> String {
        format!("{}", self.0)
    }

    fn __repr__(&self) -> String {
        format!("{:#?}", self.0)
    }

    #[getter]
    fn sample(&self) -> PySample {
        PySample(self.get_sample())
    }

    #[getter]
    fn counts(&self) -> usize {
        self.sol.counts[self.row_idx]
    }

    #[getter]
    fn obj_value(&self) -> Option<ConcreteBias> {
        self.0.obj_value()
    }

    #[getter]
    fn raw_energy(&self) -> Option<ConcreteBias> {
        self.0.raw_energy()
    }

    #[getter]
    fn constraints<'a>(&self, py: Python<'a>) -> Option<Bound<'a, PyArray1<bool>>> {
        self.constraint_satisfaction()
            .as_ref()
            .map(|c| c.to_pyarray(py))
    }

    #[getter]
    fn feasible(&self) -> Option<bool> {
        self.0.feasible()
    }
}

#[pymethods]
impl PyOwnedResult {
    #[getter]
    fn sample(&self) -> PySample {
        PySample(self.get_sample())
    }

    #[getter]
    fn obj_value(&self) -> Option<ConcreteBias> {
        self.obj_value
    }

    #[getter]
    fn constraints<'a>(&self, py: Python<'a>) -> Option<Bound<'a, PyArray1<bool>>> {
        self.constraint_satisfaction
            .as_ref()
            .map(|c| c.to_pyarray(py))
    }

    #[getter]
    fn feasible(&self) -> Option<bool> {
        self.feasible
    }

    fn __str__(&self) -> String {
        format!("{}", self.0)
    }

    fn __repr__(&self) -> String {
        format!("{:#?}", self.0)
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

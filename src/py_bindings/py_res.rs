use crate::core::{OwnedResult, ResultIterator, ResultView};
use crate::py_bindings::py_sample::PySample;
use crate::types::Bias;
use derive_more::{Deref, DerefMut};
use numpy::{PyArray1, ToPyArray};
use pyo3::{pyclass, pymethods, Bound, PyRef, PyRefMut, Python};

/// A result view object serves as a view into one row of a solution object.
///
/// The ``Result`` class is readonly as it's merely a helper class for looking into a
/// solution's row, i.e., a single sample and this sample's metadata.
///
/// Most properties available for the solution object are also available for a result,
/// but in the singular form. For example, you can call ``solution.obj_values``, but
/// ``result.obj_value``.
///
/// Examples
/// --------
/// >>> from luna_quantum import ResultView, Solution
/// >>> solution: Solution = ...
/// >>> result: ResultView = solution[0]
/// >>> result.obj_value
/// -109.42
/// >>> result.sample
/// [0, -5, 0.28]
/// >>> result.constraints
/// [True, False]
/// >>> result.feasible
/// False
#[pyclass(unsendable, name = "ResultView", module = "aqmodels")]
#[derive(Deref, DerefMut)]
pub struct PyResultView(pub ResultView);

/// A result object can be understood as a solution with only one sample.
///
/// It can be obtained by calling ``model.evaluate_sample`` for a single sample.
///
/// Most properties available for the solution object are also available for a result,
/// but in the singular form. For example, you can call ``solution.obj_values``, but
/// ``result.obj_value``.
///
/// Examples
/// --------
/// >>> from luna_quantum import Model, Result, Solution
/// >>> model: Model = ...
/// >>> solution: Solution = ...
/// >>> sample = solution.samples[0]
/// >>> result = model.evaluate_sample(sample)
/// >>> result.obj_value
/// -109.42
/// >>> result.sample
/// [0, -5, 0.28]
/// >>> result.constraints
/// [True, False]
/// >>> result.feasible
/// False
#[pyclass(unsendable, name = "Result", module = "aqmodels")]
#[derive(Deref, DerefMut)]
pub struct PyOwnedResult(pub OwnedResult);

/// An iterator over a solution's results.
///
/// Examples
/// --------
/// >>> from luna_quantum import ResultIterator, Solution
/// >>> solution: Solution = ...
/// >>> results: ResultIterator = solution.results
/// >>> for result in results:
/// ...     result.sample
/// [0, -5, 0.28]
/// [1, -4, -0.42]
#[pyclass(unsendable, name = "ResultIterator", module = "aqmodels")]
#[derive(Deref, DerefMut)]
pub struct PyResultIterator(pub ResultIterator);

impl Into<ResultView> for PyResultView {
    fn into(self) -> ResultView {
        self.0
    }
}

impl Into<OwnedResult> for PyOwnedResult {
    fn into(self) -> OwnedResult {
        self.0
    }
}

impl Into<ResultIterator> for PyResultIterator {
    fn into(self) -> ResultIterator {
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

    /// Get the sample of the result.
    #[getter]
    fn sample(&self) -> PySample {
        PySample(self.get_sample())
    }

    /// Return how often this result appears in the solution.
    #[getter]
    fn counts(&self) -> usize {
        self.sol.counts[self.row_idx]
    }

    /// Get the objective value of this sample if present. This is the value computed
    /// by the corresponding AqModel.
    #[getter]
    fn obj_value(&self) -> Option<Bias> {
        self.0.obj_value()
    }

    /// Get the raw energy returned by the algorithm if present. This value is not
    /// guaranteed to be accurate under consideration of the corresponding AqModel.
    #[getter]
    fn raw_energy(&self) -> Option<Bias> {
        self.0.raw_energy()
    }

    /// Get this result's feasibility values of all constraints. Note that
    /// `results.constraints[i]` iff. `model.constraints[i]` is feasible for
    /// this result.
    #[getter]
    fn constraints<'a>(&self, py: Python<'a>) -> Option<Bound<'a, PyArray1<bool>>> {
        self.constraint_satisfaction()
            .as_ref()
            .map(|c| c.to_pyarray(py))
    }

    /// Get this result's feasibility values of all variable bounds.
    #[getter]
    fn variable_bounds<'a>(&self, py: Python<'a>) -> Option<Bound<'a, PyArray1<bool>>> {
        self.variable_bounds_satisfaction()
            .as_ref()
            .map(|c| c.to_pyarray(py))
    }
    /// Return whether all constraint results are feasible for this result.
    #[getter]
    fn feasible(&self) -> Option<bool> {
        self.0.feasible()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

#[pymethods]
impl PyOwnedResult {
    /// Get the sample of the result.
    #[getter]
    fn sample(&self) -> PySample {
        PySample(self.get_sample())
    }

    /// Get the objective value of the result.
    #[getter]
    fn obj_value(&self) -> Option<Bias> {
        self.obj_value
    }

    /// Get this result's feasibility values of all constraints. Note that
    /// `results.constraints[i]` iff. `model.constraints[i]` is feasible for
    /// this result.
    #[getter]
    fn constraints<'a>(&self, py: Python<'a>) -> Option<Bound<'a, PyArray1<bool>>> {
        self.constraint_satisfaction
            .as_ref()
            .map(|c| c.to_pyarray(py))
    }

    /// Get this result's feasibility values of all variable bounds.
    #[getter]
    fn variable_bounds<'a>(&self, py: Python<'a>) -> Option<Bound<'a, PyArray1<bool>>> {
        self.variable_bounds_satisfaction
            .as_ref()
            .map(|c| c.to_pyarray(py))
    }

    /// Return whether all constraint results are feasible for this result.
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

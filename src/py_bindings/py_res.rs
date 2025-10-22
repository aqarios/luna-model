use super::unwind;
use crate::core::solution::result::OwnedResult;
use crate::types::Bias;
use derive_more::{Deref, DerefMut};
use numpy::{PyArray1, ToPyArray};
use pyo3::{pyclass, pymethods, Bound, PyRef, PyRefMut, Python};
use unwind_macros::unwindable;

use super::py_sample::PySample;
use super::py_sol::PySolution;

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
/// >>> from luna_model import ResultView, Solution
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
#[pyclass(name = "ResultView", module = "luna_model._core")]
pub struct PyResultView {
    pub sol: PySolution,
    pub idx: usize,
}

impl PyResultView {
    pub fn new(sol: PySolution, idx: usize) -> Self {
        Self { sol, idx }
    }
}

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
/// >>> from luna_model import Model, Result, Solution
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
#[pyclass(name = "Result", module = "luna_model._core")]
#[derive(DerefMut, Deref)]
pub struct PyOwnedResult(pub OwnedResult);

impl PyOwnedResult {
    pub fn new(owned: OwnedResult) -> Self {
        Self(owned)
    }
}

/// An iterator over a solution's results.
///
/// Examples
/// --------
/// >>> from luna_model import ResultIterator, Solution
/// >>> solution: Solution = ...
/// >>> results: ResultIterator = solution.results
/// >>> for result in results:
/// ...     result.sample
/// [0, -5, 0.28]
/// [1, -4, -0.42]
#[pyclass(name = "ResultIterator", module = "luna_model._core")]
pub struct PyResultIterator {
    sol: PySolution,
    idx: usize,
}

impl PyResultIterator {
    pub fn new(sol: PySolution) -> Self {
        Self { sol, idx: 0 }
    }
}

// impl Into<ResultView> for PyResultView {
//     fn into(self) -> ResultView {
//         self.0
//     }
// }
//
// impl Into<OwnedResult> for PyOwnedResult {
//     fn into(self) -> OwnedResult {
//         self.0
//     }
// }
//
// impl Into<ResultIterator> for PyResultIterator {
//     fn into(self) -> ResultIterator {
//         self.0
//     }
// }

#[unwindable]
#[pymethods]
impl PyResultView {
    fn __str__(&self) -> String {
        let binding = self.sol.access();
        let res = binding.get_result_view(self.idx).unwrap();
        format!("{}", res)
    }

    fn __repr__(&self) -> String {
        let binding = self.sol.access();
        let res = binding.get_result_view(self.idx).unwrap();
        format!("{:#?}", res)
    }

    /// Get the sample of the result.
    #[getter]
    fn sample(&self) -> PySample {
        PySample::new(self.sol.clone(), self.idx)
    }

    /// Return how often this result appears in the solution.
    #[getter]
    fn counts(&self) -> usize {
        self.sol.access().counts[self.idx]
    }

    /// Get the objective value of this sample if present. This is the value computed
    /// by the corresponding LunaModel.
    #[getter]
    fn obj_value(&self) -> Option<Bias> {
        let binding = self.sol.access();
        let res = binding.get_result_view(self.idx).unwrap();
        res.obj_value()
    }

    /// Get the raw energy returned by the algorithm if present. This value is not
    /// guaranteed to be accurate under consideration of the corresponding LunaModel.
    #[getter]
    fn raw_energy(&self) -> Option<Bias> {
        let binding = self.sol.access();
        let res = binding.get_result_view(self.idx).unwrap();
        res.raw_energy()
    }

    /// Get this result's feasibility values of all constraints. Note that
    /// `results.constraints[i]` iff. `model.constraints[i]` is feasible for
    /// this result.
    #[getter]
    fn constraints<'a>(&self, py: Python<'a>) -> Option<Bound<'a, PyArray1<bool>>> {
        let binding = self.sol.access();
        let res = binding.get_result_view(self.idx).unwrap();
        res.constraint_satisfaction()
            .as_ref()
            .map(|c| c.to_pyarray(py))
    }

    /// Get this result's feasibility values of all variable bounds.
    #[getter]
    fn variable_bounds<'a>(&self, py: Python<'a>) -> Option<Bound<'a, PyArray1<bool>>> {
        let binding = self.sol.access();
        let res = binding.get_result_view(self.idx).unwrap();
        res.variable_bounds_satisfaction()
            .as_ref()
            .map(|c| c.to_pyarray(py))
    }
    /// Return whether all constraint results are feasible for this result.
    #[getter]
    fn feasible(&self) -> Option<bool> {
        let binding = self.sol.access();
        let res = binding.get_result_view(self.idx).unwrap();
        res.feasible()
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.idx == other.idx && *self.sol.access() == *other.sol.access()
    }
}

#[unwindable]
#[pymethods]
impl PyOwnedResult {
    /// Get the sample of the result.
    #[getter]
    fn sample(&self) -> PySample {
        PySample::owned(self.0.sample.clone())
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

#[unwindable]
#[pymethods]
impl PyResultIterator {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<PyResultView> {
        let binding = slf.sol.access();
        let samples = binding.samples();
        let res = samples
            .get_result_view(slf.idx)
            .map(|_| PyResultView::new(slf.sol.clone(), slf.idx));
        drop(binding);
        slf.idx += 1;
        res
    }
}

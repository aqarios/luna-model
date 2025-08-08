use crate::core::solution::sample::SampleOwned;
use crate::core::VarAssignment;
use crate::py_bindings::py_sol::PyVarAssignment;
use derive_more::{Deref, DerefMut};
use itertools::Itertools;
use pyo3::exceptions::{PyIndexError, PyTypeError, PyValueError};
use pyo3::prelude::*;
use pyo3::types::PyDict;
use pyo3::IntoPyObjectExt;
use super::py_sol::PySolution;
use unwind_macros::unwindable;
use super::unwind;
use super::py_var::PyVariable;

/// An iterator over a solution's samples.
///
/// Examples
/// --------
/// >>> from luna_quantum import Solution
/// >>> solution: Solution = ...
///
/// Note: ``solution.samples`` is automatically converted into a ``SamplesIterator``.
///
/// >>> for sample in solution.samples:
/// ...     sample
/// [0, -5, 0.28]
/// [1, -4, -0.42]
#[cfg_attr(
    not(feature = "lq"),
    pyclass(name = "SamplesIterator", module = "aqmodels._core")
)]
#[cfg_attr(
    feature = "lq",
    pyclass(name = "SamplesIterator", module = "luna_quantum._core")
)]
pub struct PySamplesIterator {
    sol: PySolution,
    idx: usize,
}

impl PySamplesIterator {
    fn new(sol: PySolution) -> Self {
        Self { sol, idx: 0 }
    }
}

// impl PySamplesIterator {
//
// }

/// An iterator over the variable assignments of a solution's sample.
///
/// Examples
/// --------
/// >>> from luna_quantum import Solution
/// >>> solution: Solution = ...
/// >>> sample = solution.samples[0]
///
/// Note: ``sample`` is automatically converted into a ``SampleIterator``.
///
/// >>> for var in sample:
/// ...     var
/// 0
/// -5
/// 0.28
#[cfg_attr(
    not(feature = "lq"),
    pyclass(name = "SampleIterator", module = "aqmodels._core")
)]
#[cfg_attr(
    feature = "lq",
    pyclass(name = "SampleIterator", module = "luna_quantum._core")
)]
pub struct PySampleIterator {
    sample: PySample,
    idx: usize,
}

impl PySampleIterator {
    fn new(sample: PySample) -> Self {
        Self { sample, idx: 0 }
    }
}

/// A samples object is simply a set-like object that contains every different sample
/// of a solution.
///
/// The ``Samples`` class is readonly as it's merely a helper class for looking into a
/// solution's different samples.
///
/// Examples
/// --------
/// >>> from luna_quantum import Model, Sample, Solution
/// >>> model: Model = ...
/// >>> solution: Solution = ...
/// >>> samples: Samples = solution.samples
/// >>> samples
/// [0, -5, 0.28]
/// [1, -4, -0.42]
#[cfg_attr(
    not(feature = "lq"),
    pyclass(name = "Samples", module = "aqmodels._core")
)]
#[cfg_attr(
    feature = "lq",
    pyclass(name = "Samples", module = "luna_quantum._core")
)]
#[derive(Deref, DerefMut)]
pub struct PySamples(pub PySolution);

/// A sample object is an assignment of an actual value to each of the models'
/// variables.
///
/// The ``Sample`` class is readonly as it's merely a helper class for looking into a
/// single sample of a solution.
///
/// Note: a ``Sample`` can be converted to ``list[int | float]`` simply by calling
/// ``list(sample)``.
///
/// Examples
/// --------
/// >>> from luna_quantum import Model, Sample, Solution
/// >>> model: Model = ...
/// >>> solution: Solution = ...
/// >>> sample: Sample = solution.samples[0]
/// >>> sample
/// [0, -5, 0.28]
#[cfg_attr(
    not(feature = "lq"),
    pyclass(name = "Sample", module = "aqmodels._core")
)]
#[cfg_attr(
    feature = "lq",
    pyclass(name = "Sample", module = "luna_quantum._core")
)]
#[derive(Clone)]
pub struct PySample(pub PySampleInner);

#[derive(Clone)]
pub enum PySampleInner {
    View(PySView),
    Owned(PySOwned),
}
#[derive(Clone)]
pub struct PySView {
    pub sol: PySolution,
    pub row: usize,
}
impl PySView {
    fn new(sol: PySolution, row: usize) -> Self {
        Self { sol, row }
    }
}
#[cfg_attr(
    not(feature = "lq"),
    pyclass(name = "OwnedSample", module = "aqmodels._core")
)]
#[cfg_attr(
    feature = "lq",
    pyclass(name = "OwnedSample", module = "luna_quantum._core")
)]
#[derive(Clone, Deref)]
pub struct PySOwned(pub SampleOwned);

impl PySample {
    pub fn new(sol: PySolution, row: usize) -> Self {
        Self(PySampleInner::View(PySView::new(sol, row)))
    }
    pub fn owned(sample: SampleOwned) -> Self {
        Self(PySampleInner::Owned(PySOwned(sample)))
    }

    fn get_assignment(&self, col: usize) -> Option<VarAssignment> {
        match &self.0 {
            PySampleInner::View(view) => {
                let binding = view.sol.0.access();
                let samples = binding.samples();
                samples.get_assignment(view.row, col)
            }
            PySampleInner::Owned(owned) => owned.actual.get(col).copied(),
        }
    }

    fn index_for_variable_name(&self, varname: &str) -> Option<usize> {
        match &self.0 {
            PySampleInner::View(view) => {
                let binding = view.sol.0.access();
                let samples = binding.samples();
                let sample = samples.get_sample(view.row);
                sample.map(|s| s.index_for_variable_name(varname))?
            }
            PySampleInner::Owned(owned) => owned.variable_names.iter().position(|n| n == varname),
        }
    }
}

// impl Into<SamplesIterator> for PySamplesIterator {
//     fn into(self) -> SamplesIterator {
//         self.0
//     }
// }
//
// impl Into<SampleIterator> for PySampleIterator {
//     fn into(self) -> SampleIterator {
//         self.0
//     }
// }

#[unwindable]
#[pymethods]
impl PySamples {
    /// Convert the sample into a 2-dimensional list where a row constitutes a single
    /// sample, and a column constitutes all assignments for a single variable.
    ///
    /// Returns
    /// -------
    /// list[list[int | float]]
    ///     The samples object as a 2-dimensional list.
    fn tolist(&self, py: Python) -> Vec<Vec<PyObject>> {
        let b = self.access();
        let samples = b.samples();
        samples
            .iter()
            .map(|s| {
                s.iter()
                    .map(|v| PyVarAssignment(v).into_pyobject(py).unwrap().unbind())
                    .collect_vec()
            })
            .collect_vec()
    }

    fn __str__<'a>(&'a self) -> String {
        let b = self.access();
        let samples = b.samples();
        format!("{}", samples)
    }

    /// Extract a sample or variable assignment from the ``Samples`` object.
    /// If ``item`` is an int, returns the sample in this row. If ``item`` is a tuple
    /// of ints `(i, j)`, returns the variable assignment in row `i` and column `j`.
    ///
    /// Returns
    /// -------
    /// Sample or int or float
    ///
    /// Raises
    /// ------
    /// TypeError
    ///     If ``item`` has the wrong type.
    /// IndexError
    ///     If the row or column index is out of bounds for the variable environment.
    fn __getitem__(&self, py: Python, item: PyObject) -> PyResult<PyObject> {
        if let Ok(res_idx) = item.extract::<isize>(py) {
            if res_idx < 0 {
                return Err(PyValueError::new_err(format!(
                    "Expected a non-negative number, received: {res_idx}"
                )))?;
            }
            match self.access().samples().get_sample(res_idx as usize) {
                None => Err(PyIndexError::new_err(format!(
                    "Index {res_idx} out of bounds"
                ))),
                Some(_) => PySample::new(self.0.clone(), res_idx as usize)
                    .into_pyobject(py)?
                    .into_py_any(py),
            }
        } else if let Ok((res_idx, var_idx)) = item.extract::<(isize, isize)>(py) {
            if res_idx < 0 {
                return Err(PyValueError::new_err(format!(
                    "Expected a non-negative number, received: {res_idx}"
                )))?;
            }
            if var_idx < 0 {
                return Err(PyValueError::new_err(format!(
                    "Expected a non-negative number, received: {var_idx}"
                )))?;
            }
            match self
                .access()
                .samples()
                .get_assignment(res_idx as usize, var_idx as usize)
            {
                None => Err(PyIndexError::new_err(format!(
                    "Index ({res_idx}, {var_idx}) out of bounds"
                ))),
                Some(v) => Ok(PyVarAssignment(v).into_pyobject(py)?.unbind()),
            }
        } else {
            Err(PyTypeError::new_err("unsupported type for indexing"))
        }
    }

    /// Get the number of samples present in this sample set.
    ///
    /// Returns
    /// -------
    /// int
    fn __len__(&self) -> usize {
        self.access().n_samples
    }

    /// Iterate over all samples of this sample set.
    ///
    /// Returns
    /// -------
    /// SamplesIterator
    fn __iter__(&self) -> PySamplesIterator {
        PySamplesIterator::new(self.0.clone())
    }
}

#[unwindable]
#[pymethods]
impl PySample {
    fn __str__(&self) -> String {
        match &self.0 {
            PySampleInner::View(view) => {
                let binding = view.sol.0.access();
                let samples = binding.samples();
                let sample = samples.get_sample(view.row).unwrap();
                format!("{}", sample)
            }
            PySampleInner::Owned(owned) => format!("{}", owned.0),
        }
    }

    /// Extract a variable assignment from the ``Sample`` object.
    ///
    /// Returns
    /// -------
    /// Sample or int or float
    ///
    /// Raises
    /// ------
    /// TypeError
    ///     If ``item`` has the wrong type.
    /// IndexError
    ///     If the row or column index is out of bounds for the variable environment.
    fn __getitem__(&self, py: Python, item: PyObject) -> PyResult<PyVarAssignment> {
        if let Ok(var) = item.extract::<PyVariable>(py) {
            match self.get_assignment(var.id.into()) {
                None => Err(PyIndexError::new_err(format!(
                    "Index {:?} out of bounds",
                    var.id
                ))),
                Some(v) => Ok(PyVarAssignment(v)),
            }
        } else if let Ok(var_name) = item.extract::<String>(py) {
            if let Some(var_idx) = self.index_for_variable_name(&var_name) {
                match self.get_assignment(var_idx) {
                    None => Err(PyIndexError::new_err(format!(
                        "Index {var_idx} out of bounds"
                    ))),
                    Some(v) => Ok(PyVarAssignment(v)),
                }
            } else {
                Err(PyValueError::new_err(format!(
                    "unknown variable name: '{var_name}'"
                )))
            }
        } else if let Ok(var_idx) = item.extract::<isize>(py) {
            if var_idx < 0 {
                return Err(PyValueError::new_err(format!(
                    "Expected a non-negative number, received: {var_idx}"
                )))?;
            }
            match self.get_assignment(var_idx as usize) {
                None => Err(PyIndexError::new_err(format!(
                    "Index {var_idx} out of bounds"
                ))),
                Some(v) => Ok(PyVarAssignment(v)),
            }
        } else {
            Err(PyTypeError::new_err("unsupported type for indexing"))
        }
    }

    /// Get the number of variables present in this sample.
    ///
    /// Returns
    /// -------
    /// int
    fn __len__(&self) -> usize {
        match &self.0 {
            PySampleInner::View(view) => {
                let binding = view.sol.access();
                let samples = binding.samples();
                let sample = samples.get_sample(view.row).unwrap();
                sample.len()
            }
            PySampleInner::Owned(owned) => owned.variable_names.len(),
        }
    }

    /// Iterate over all variable assignments of this sample.
    ///
    /// Returns
    /// -------
    /// SampleIterator
    fn __iter__(slf: PyRef<'_, Self>) -> PySampleIterator {
        PySampleIterator::new(slf.clone())
    }

    /// Convert the sample to a dictionary.

    /// Returns
    /// -------
    /// dict
    ///     A dictionary representation of the sample, where the keys are the
    ///     variable names and the values are the variables' assignments.
    fn to_dict<'py>(&'py self, py: Python<'py>) -> Bound<'py, PyDict> {
        match &self.0 {
            PySampleInner::View(view) => {
                let binding = view.sol.access();
                let samples = binding.samples();
                let sample = samples.get_sample(view.row).unwrap();
                let py_dict = PyDict::new(py);
                for (k, v) in sample.to_map() {
                    py_dict.set_item(k, PyVarAssignment(v)).unwrap()
                }
                py_dict
            }
            PySampleInner::Owned(owned) => {
                let py_dict = PyDict::new(py);
                for (k, v) in owned.to_map() {
                    py_dict.set_item(k, PyVarAssignment(*v)).unwrap()
                }
                py_dict
            }
        }
    }
}

#[unwindable]
#[pymethods]
impl PySampleIterator {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<PyVarAssignment> {
        let out = match &slf.sample.0 {
            PySampleInner::View(view) => {
                let b = view.sol.access().get_assignment(view.row, slf.idx);
                let out = b.map(|v| PyVarAssignment(v));
                out
            }
            PySampleInner::Owned(owned) => {
                let out = owned.actual.get(slf.idx);
                out.map(|v| PyVarAssignment(*v))
            }
        };
        slf.idx += 1;
        out
    }
}

#[unwindable]
#[pymethods]
impl PySamplesIterator {
    fn __iter__(slf: PyRef<'_, Self>) -> PyRef<'_, Self> {
        slf
    }

    fn __next__(mut slf: PyRefMut<'_, Self>) -> Option<PySample> {
        let binding = slf.sol.access();
        let samples = binding.samples();
        let sample = samples
            .get_sample(slf.idx)
            .map(|_| PySample::new(slf.sol.clone(), slf.idx));
        drop(binding);
        slf.idx += 1;
        sample
    }
}

use super::{ColElement, Column, Solution};
use crate::{
    core::{traits::FilterByMask, ResultView, Sense},
    errors::ComputationErr,
};

#[cfg(feature = "py")]
use {
    crate::py_bindings::py_res::PyResultIterator,
    crate::py_bindings::py_res::PyResultView,
    crate::py_bindings::py_sol::PySolution,
    pyo3::prelude::*,
    pyo3::{exceptions::PyTypeError, IntoPyObjectExt},
};

pub type FilterFn = fn(&ResultView) -> bool;

#[cfg(feature = "py")]
#[cfg_attr(feature = "py", pyclass(unsendable))]
struct PyFilterFn {
    callback: FilterFn,
}

#[cfg(feature = "py")]
#[cfg_attr(feature = "py", pymethods)]
impl PyFilterFn {
    fn __call__(&self, pyresview: &PyResultView) -> PyResult<bool> {
        let sol = pyresview.sol.0.access();
        let resview = ResultView::new(&sol, pyresview.idx);
        Ok((self.callback)(&resview))
    }
}

#[derive(Debug)]
pub enum Filter {
    RsFilter(FilterFn),
    #[cfg(feature = "py")]
    PyFilter(Py<PyAny>),
}

fn filter_sol_rs(rs_fn: &FilterFn, sol: &Solution) -> Vec<bool> {
    sol.iter_result_views().map(|x| rs_fn(&x)).collect()
}

#[cfg(feature = "py")]
fn filter_sol_py(py_fn: &Py<PyAny>, sol: &Solution) -> Result<Vec<bool>, ComputationErr> {
    Python::attach(|py| {
        let pyresiter = PyResultIterator::new(PySolution::new(sol.clone()));

        pyresiter
            .map(|x| {
                let r = py_fn
                    .call1(py, (x,))
                    .map_err(|e| ComputationErr(e.to_string()))?;
                r.extract::<bool>(py)
                    .map_err(|e| ComputationErr(e.to_string()))
            })
            .collect::<Result<Vec<bool>, ComputationErr>>()
    })
}

impl Filter {
    fn call(&self, sol: &Solution) -> Result<Vec<bool>, ComputationErr> {
        match self {
            Self::RsFilter(rs_fn) => Ok(filter_sol_rs(rs_fn, sol)),
            #[cfg(feature = "py")]
            Self::PyFilter(py_fn) => filter_sol_py(py_fn, sol),
        }
    }
}

#[cfg(feature = "py")]
impl<'py> IntoPyObject<'py> for Filter {
    type Error = PyErr;
    type Target = PyAny;
    type Output = Bound<'py, PyAny>;

    fn into_pyobject(self, py: Python<'py>) -> Result<Self::Output, Self::Error> {
        match self {
            Filter::PyFilter(cb) => Ok(cb.into_pyobject(py)?),
            Filter::RsFilter(rs) => {
                let wrapper = Py::new(py, PyFilterFn { callback: rs })?;
                Ok(wrapper.into_py_any(py)?.into_pyobject(py)?)
            }
        }
    }
}

#[cfg(feature = "py")]
impl<'py> FromPyObject<'py> for Filter {
    fn extract_bound(ob: &Bound<'py, PyAny>) -> PyResult<Self> {
        let py = ob.py();
        if ob.is_callable() {
            let cb: Py<PyAny> = ob.into_py_any(py)?;
            Ok(Filter::PyFilter(cb))
        } else {
            Err(PyTypeError::new_err("Filter must be callable"))
        }
    }
}

impl Clone for Filter {
    fn clone(&self) -> Self {
        match self {
            Self::RsFilter(inner) => Self::RsFilter(inner.clone()),
            #[cfg(feature = "py")]
            Self::PyFilter(pyany) => Self::PyFilter(Python::attach(|py| pyany.clone_ref(py))),
        }
    }
}

impl Solution {
    pub fn filter(&self, f: Filter) -> Result<Solution, ComputationErr> {
        let mask = f.call(&self)?;
        Ok(self.filter_samples(&mask))
    }

    pub fn filter_samples(&self, mask: &Vec<bool>) -> Self {
        if self.n_samples != mask.len() {
            panic!(
                "Filter sample should only be called internally and provide mask with correct len"
            )
        }
        let mut sol = Self::default();
        sol.samples = self
            .samples
            .iter()
            .map(|col| match col {
                Column::Binary(b) => {
                    Column::Binary(ColElement::new(b.varid, b.data.filter_by_mask(mask)))
                }
                Column::Spin(s) => Column::Spin(ColElement::new(s.varid, s.filter_by_mask(mask))),
                Column::Integer(i) => {
                    Column::Integer(ColElement::new(i.varid, i.filter_by_mask(mask)))
                }
                Column::Real(r) => Column::Real(ColElement::new(r.varid, r.filter_by_mask(mask))),
            })
            .collect();
        sol.sense = self.sense;
        sol.timing = self.timing;
        sol.variable_names = self.variable_names.clone();
        sol.counts = self.counts.filter_by_mask(mask);
        sol.obj_values = self.obj_values.as_ref().map(|o| o.filter_by_mask(mask));
        sol.raw_energies = self.raw_energies.as_ref().map(|e| e.filter_by_mask(mask));
        sol.constraints = self.constraints.as_ref().map(|c| c.filter_by_mask(mask));
        sol.variable_bounds = self
            .variable_bounds
            .as_ref()
            .map(|b| b.filter_by_mask(mask));
        sol.feasible = self.feasible.as_ref().map(|f| f.filter_by_mask(mask));
        sol.n_samples = sol.counts.len();
        sol.ensure_best_sample_idx();
        sol
    }
}

impl Solution {
    fn ensure_best_sample_idx(&mut self) {
        match (&self.feasible, &self.obj_values) {
            (Some(f), Some(ov)) => {
                self.best_sample_idx =
                    f.iter()
                        .zip(ov)
                        .enumerate()
                        .fold(None, |acc, (idx, (&feas, &obj))| match acc {
                            None => Some(idx),
                            Some(a) => {
                                let best_obj = ov[a];
                                if feas
                                    && (self.sense == Sense::Min && obj < best_obj
                                        || self.sense == Sense::Max && obj > best_obj)
                                {
                                    Some(idx)
                                } else {
                                    acc
                                }
                            }
                        })
            }
            _ => self.best_sample_idx = None,
        }
    }
}

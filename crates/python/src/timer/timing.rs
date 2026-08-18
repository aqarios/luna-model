//! Python wrappers around timing records.

use std::time::SystemTime;

use indexmap::IndexMap;
use lunamodel_core::Timing;
use lunamodel_io::{CustomFormat, FormatOpt};
use lunamodel_unwind::*;
use pyo3::{Bound, FromPyObject, pymethods, types::PyType};

use super::PyTiming;

#[derive(Debug, Clone, FromPyObject)]
enum Item {
    Float(f64),
    List(Vec<f64>),
}

#[unwindable]
#[pymethods]
impl PyTiming {
    #[new]
    fn new(total: Option<f64>) -> Self {
        match total {
            Some(t) => Timing::new(t).into(),
            None => Timing::default().into(),
        }
    }

    #[classmethod]
    fn from_dict(
        _cls: &Bound<'_, PyType>,
        timings: IndexMap<String, Item>,
        total: Option<f64>,
    ) -> Self {
        let mut timing = Timing::default();
        timing.timings = timings
            .into_iter()
            .map(|(key, e)| match e {
                Item::List(values) => (key, values),
                Item::Float(value) => (key, vec![value]),
            })
            .collect();

        timing.total = match total {
            Some(t) => t,
            None => timing
                .timings
                .iter()
                .map(|(_, values)| values.iter().sum::<f64>())
                .sum(),
        };

        timing.into()
    }

    #[getter]
    fn start(&self) -> Option<SystemTime> {
        self.0.read().start
    }

    #[getter]
    fn end(&self) -> Option<SystemTime> {
        self.0.read().end
    }

    /// The total time in seconds an algorithm needed to run. Computed as the
    /// difference of end and start time.
    ///
    /// Raises
    /// ------
    /// RuntimeError
    ///     If total_seconds cannot be computed due to an inconsistent start or end time.
    #[getter]
    fn get_total(&self) -> f64 {
        self.0.read().total
    }

    fn total_for(&self, timing: String) -> Option<f64> {
        self.0.read().total_for(timing)
    }

    fn get(&self, key: String) -> Vec<f64> {
        self.0.read().get(key).to_vec()
    }

    fn __setitem__(&mut self, key: String, value: f64) {
        self.0.write().set(key, value);
    }

    fn __getitem__(&mut self, key: String) -> f64 {
        self.0.read().total_for(key).unwrap_or(0.0)
    }

    fn __eq__(&self, other: &Self) -> bool {
        self.0.read().eq(&other.0.read())
    }

    fn __str__(&self) -> String {
        format!("{}", self.0.read().format(FormatOpt::Py))
    }

    fn __repr__(&self) -> String {
        format!("{:?}", self.0.read().format(FormatOpt::Py))
    }
}

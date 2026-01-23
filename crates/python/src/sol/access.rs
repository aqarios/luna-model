use lunamodel_types::Sense;
use lunamodel_unwind::*;
use numpy::{PyArray1, ToPyArray};
use pyo3::{Bound, Python, pymethods};

use super::PySolution;
use super::result::PyResultIterator;
use super::sample::PySamplesIterator;
use crate::timer::PyTiming;

#[unwindable]
#[pymethods]
impl PySolution {
    // The setter is defined here [super::modification::PySolution::set_obj_values]
    #[getter]
    fn get_obj_values<'py>(&self, py: Python<'py>) -> Option<Bound<'py, PyArray1<f64>>> {
        self.s
            .read_arc()
            .obj_values
            .as_ref()
            .map(|e| e.to_pyarray(py))
    }

    // The setter is defined here [super::modification::PySolution::set_raw_energies]
    #[getter]
    fn get_raw_energies<'py>(&self, py: Python<'py>) -> Option<Bound<'py, PyArray1<f64>>> {
        self.s
            .read_arc()
            .raw_energies
            .as_ref()
            .map(|e| e.to_pyarray(py))
    }

    #[getter]
    fn get_counts<'py>(&self, py: Python<'py>) -> Bound<'py, PyArray1<usize>> {
        self.s.read_arc().counts.to_pyarray(py)
    }

    #[getter]
    fn get_runtime(&self) -> Option<PyTiming> {
        self.s.read_arc().timing.map(|t| t.into())
    }

    #[getter]
    fn get_sense(&self) -> Sense {
        self.s.read_arc().sense
    }

    #[getter]
    fn get_results(&self) -> PyResultIterator {
        PyResultIterator::new(self.clone())
    }

    #[getter]
    fn get_samples(&self) -> PySamplesIterator {
        PySamplesIterator::new(self.clone())
    }

    #[getter]
    fn get_variable_names(&self) -> Vec<String> {
        self.s.read_arc().variable_names()
    }

    #[getter]
    fn get_best_sample_idx<'py>(&self) -> Option<()> {
        unimplemented!("SET RETURN TYPE")
    }
}

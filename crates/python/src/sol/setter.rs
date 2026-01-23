use lunamodel_unwind::unwindable;
use numpy::{PyArray1, PyArrayMethods};
use pyo3::{Bound, PyResult, pymethods};

use super::PySolution;
use crate::timer::PyTiming;
use crate::unwind::unwind;

#[unwindable]
#[pymethods]
impl PySolution {
    // The getter is defined here [super::access::PySolution::get_obj_values]
    #[setter]
    fn set_obj_values<'py>(&mut self, values: Option<Bound<'py, PyArray1<f64>>>) -> PyResult<()> {
        self.s.write_arc().obj_values = match values {
            Some(arr) => Some(arr.to_vec()?),
            None => None,
        };
        Ok(())
    }

    // The getter is defined here [super::access::PySolution::get_raw_energies]
    #[setter]
    fn set_raw_energies<'py>(&mut self, values: Option<Bound<'py, PyArray1<f64>>>) -> PyResult<()> {
        self.s.write_arc().raw_energies = match values {
            Some(arr) => Some(arr.to_vec()?),
            None => None,
        };
        Ok(())
    }

    // The set runtime is defined here [super::access::PySolution::get_raw_energies]
    #[setter]
    fn set_runtime<'py>(&mut self, timing: PyTiming) {
        self.s.write_arc().timing = Some(*timing)
    }
}

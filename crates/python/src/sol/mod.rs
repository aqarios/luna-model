//! Python wrapper for solutions, result views, and sample views.
mod access;
mod convenience;
mod creation;
mod filter;
mod general;
mod io;
mod modification;
pub mod result;
pub mod sample;
mod ser;
mod setter;
mod utils;

use std::sync::Arc;

use lunamodel_core::Solution;
use parking_lot::RwLock;
use pyo3::pyclass;

#[pyclass(from_py_object)]
#[repr(C)]
#[derive(Clone, Debug)]
pub struct PySolution {
    /// Shared core solution handle.
    pub s: Arc<RwLock<Solution>>,
}

impl PySolution {
    pub fn inner(&self) -> Arc<RwLock<Solution>> {
        Arc::clone(&self.s)
    }
}

impl From<Solution> for PySolution {
    /// Wraps an owned core solution for Python.
    fn from(sol: Solution) -> Self {
        PySolution {
            s: Arc::new(RwLock::new(sol)),
        }
    }
}

mod access;
mod convenience;
mod filter;
mod io;
mod modification;
mod result;
mod sample;
mod setter;
mod timing;
mod general;
mod creation;
mod utils;

use std::sync::Arc;

use lunamodel_core::Solution;
use parking_lot::RwLock;
use pyo3::pyclass;

#[pyclass]
#[repr(C)]
#[derive(Clone)]
pub struct PySolution {
    pub s: Arc<RwLock<Solution>>,
}

impl From<Solution> for PySolution {
    fn from(sol: Solution) -> Self {
        PySolution {
            s: Arc::new(RwLock::new(sol)),
        }
    }
}

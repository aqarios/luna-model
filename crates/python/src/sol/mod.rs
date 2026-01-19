mod access;
mod convenience;
mod creation;
mod filter;
mod general;
mod io;
mod modification;
mod result;
pub mod sample;
mod ser;
mod setter;
mod utils;

use std::sync::Arc;

use lunamodel_core::Solution;
use parking_lot::RwLock;
use pyo3::pyclass;

#[pyclass]
#[repr(C)]
#[derive(Clone, Debug)]
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

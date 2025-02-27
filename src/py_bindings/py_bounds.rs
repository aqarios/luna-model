use crate::core::Bounds;
use derive_more::{Deref, DerefMut};
use pyo3::prelude::*;

#[pyclass(name = "Bounds")]
#[derive(Clone, Copy, Deref, DerefMut)]
pub struct PyBounds(Bounds);

// impl PyBounds {
//     pub fn map_option(b: Option<Self>) -> Option<Bounds> {
//         match b {
//             Some(e) => Some(e.0),
//             None => None,
//         }
//     }
// }

#[pymethods]
impl PyBounds {
    #[new]
    #[pyo3(signature=(lower, upper))]
    fn py_new(lower: f64, upper: f64) -> PyResult<Self> {
        Ok(PyBounds(Bounds::new(Some(lower), Some(upper))))
    }
}

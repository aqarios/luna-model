use crate::core::Vtype;

use derive_more::{Deref, DerefMut};
use pyo3::prelude::*;

#[pyclass(eq, name = "Vtype")]
#[derive(Copy, Clone, PartialEq, Deref, DerefMut)]
pub struct PyVtype(Vtype);

// impl PyVtype {
//     pub fn map_option(a: Option<Self>) -> Option<Vtype> {
//         match a {
//             Some(e) => Some(e.0),
//             None => None,
//         }
//     }
// }

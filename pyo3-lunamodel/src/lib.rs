pub mod types;

use once_cell::sync::Lazy;
use pyo3::prelude::*;

// pub use lunamodel_types as lmtypes;

pub use lunamodel_types::Vtype;
pub use lunamodel_python::PyExprContent;

pub(crate) static LUNA_MODEL: Lazy<Py<PyModule>> =
    Lazy::new(|| Python::attach(|py| PyModule::import(py, "lm").unwrap().unbind()));

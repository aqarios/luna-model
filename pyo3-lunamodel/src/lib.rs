mod bounds;
mod constraint;
mod constraint_collection;
mod environment;
mod expression;
mod model;
mod sol;
mod types;
mod variable;
mod utils;

pub mod prelude;

use once_cell::sync::Lazy;
use pyo3::prelude::*;

// pub use lunamodel_types as lmtypes;

pub use lunamodel_core as core;
pub use lunamodel_python::PyExprContent;

pub(crate) static LUNA_MODEL: Lazy<Py<PyModule>> =
    Lazy::new(|| Python::attach(|py| PyModule::import(py, "luna_model").unwrap().unbind()));

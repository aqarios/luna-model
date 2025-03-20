#[cfg(feature = "py")]
use pyo3::prelude::*;
use strum_macros::{Display, EnumIter};

/// The type a variable can represent.
#[cfg_attr(feature = "py", pyclass(eq, eq_int))] // we require the python config here, since wrapping an enum in the py_bindings is a tedious task.
#[derive(Debug, Copy, Clone, PartialEq, EnumIter, Display)]
pub enum Vtype {
    Binary,
    Spin,
    Integer,
    Real,
}

impl Vtype {
    /// The default variable type.
    pub fn default() -> Self {
        Vtype::Binary
    }
}

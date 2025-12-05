use strum_macros::{Display, EnumString};

#[cfg(feature = "py")]
use pyo3::prelude::pyclass;

/// Enumeration of optimization senses supported by the optimization system.
///
/// This enum defines the type of optimization used for a model. The type influences
/// the domain and behavior of the model during optimization.
#[derive(Display, Copy, PartialEq, Hash, Clone, Debug, Eq, EnumString)]
#[cfg_attr(
    feature = "py",
    pyclass(eq, eq_int, name = "PySense") // , module = "luna_model.Vtype")
)]
pub enum Sense {
    /// Indicate the objective function to be minimized.
    #[strum(to_string = "Minimize", serialize = "Min")]
    Min,
    /// Indicate the objective function to be maximized.
    #[strum(to_string = "Maximize", serialize = "Max")]
    Max,
}

impl Sense {
    /// Convenience function to check if the sense is `Sense::Min`.
    pub fn is_min(&self) -> bool {
        self == &Self::Min
    }
}

impl Default for Sense {
    fn default() -> Self {
        Self::Min
    }
}

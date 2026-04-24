use std::ops::Not;

use strum_macros::{Display, EnumString};

/// Enumeration of optimization senses supported by the optimization system.
///
/// This enum defines the type of optimization used for a model. The type influences
/// the domain and behavior of the model during optimization.
#[derive(Display, Copy, PartialEq, Hash, Clone, Debug, Eq, EnumString, Default)]
pub enum Sense {
    /// Indicate the objective function to be minimized.
    #[strum(to_string = "Minimize", serialize = "Min")]
    #[default]
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

impl Not for Sense {
    type Output = Self;

    /// Flips minimization to maximization and vice versa.
    fn not(self) -> Self::Output {
        match self {
            Self::Min => Self::Max,
            Self::Max => Self::Min,
        }
    }
}

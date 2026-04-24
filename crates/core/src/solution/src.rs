use std::fmt::Display;

/// Selects which scalar value series a convenience statistic should use.
#[derive(Debug, Clone)]
pub enum ValueSource {
    /// Use the raw solver-provided energies.
    Raw,
    /// Use the model-evaluated objective values.
    Obj,
}
impl Display for ValueSource {
    /// Formats the corresponding field name used in error messages.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Raw => f.write_str("raw_energies"),
            Self::Obj => f.write_str("obj_values"),
        }
    }
}

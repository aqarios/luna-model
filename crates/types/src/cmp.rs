use strum_macros::Display;

#[cfg(feature = "py")]
use pyo3::prelude::pyclass;

/// Comparison operators used to define constraints.
///
/// This enum represents the logical relation between the left-hand side (LHS)
/// and the right-hand side (RHS) of a [Constraint].
#[derive(Debug, Copy, Clone, PartialEq, Display, Eq, Hash)]
#[cfg_attr(
    feature = "py",
    pyclass(eq, eq_int, name = "PyComparator") // , module = "luna_model.Vtype")
)]
pub enum Comparator {
    /// The Equality comparison (==) for a constraint where LHS == RHS.
    #[strum(to_string = "==")]
    Eq,
    /// The Less-than or equal comparison (<=) for a constraint where LHS <= RHS.
    #[strum(to_string = "<=")]
    Le,
    /// The Greater-than or equal comparison (>=) for a constraint where LHS >= RHS.
    #[strum(to_string = ">=")]
    Ge,
}

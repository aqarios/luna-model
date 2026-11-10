//! Model-part enum used to track which parts of a model passes read and write.

use enumset::EnumSetType;
use strum_macros::Display;

/// A part of a model that a transformation may modify or an analysis may depend on.
///
/// Analyses declare the parts they read and transformations declare the parts they write; an
/// analysis result is invalidated by a transformation exactly when the two sets overlap.
#[derive(EnumSetType, Display, Debug, Hash)]
pub enum ModelPart {
    /// The objective expression.
    Objective,
    /// The constraint collection.
    Constraints,
    /// The environment: variables, their types and their bounds.
    Variables,
    /// The optimization sense.
    Sense,
}

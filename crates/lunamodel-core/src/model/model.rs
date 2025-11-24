use super::sense::Sense;
use crate::{constraint::ConstraintCollection, environment::Environment, expression::Expression};

/// The default name for a model.
pub static DEFAULT_MODEL_NAME: &str = "unnamed";

/// A model describing some function to be optimized (objective) and restrictions
/// on this objective (constraints).
#[derive(Clone)]
pub struct Model {
    /// The name of the model, by default is [DEFAULT_MODEL_NAME].
    pub name: String,
    /// The environment of the model, constaining the information for each variable
    /// used in both the objective and it's constraints.
    pub environment: Environment,
    /// The objective of the model describing some optimization problem. The objective
    /// is an expression that can be linear, quadratic or higher order.
    pub objective: Expression,
    /// The constraints of the model describing the restrictions on the model.
    pub constraints: ConstraintCollection,
    /// The sense of the model, i.e., the direction to be optimized at.
    /// By default is set to [Sense::Min].
    pub sense: Sense,
}

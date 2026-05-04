//! Equality implementations for models.

use super::Model;
use crate::traits::ContentEquality;

impl PartialEq for Model {
    /// Compares models including environment identity, name, and sense.
    ///
    /// Two models are equal under `==` when they share the same environment
    /// instance and have matching name, objective, constraints, and sense.
    /// Use [`ContentEquality::equal_contents`] for an environment- and
    /// name-agnostic content comparison.
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
            && self.environment.id() == other.environment.id()
            && self.objective == other.objective
            && self.constraints == other.constraints
            && self.sense == other.sense
    }
}

impl ContentEquality for Model {
    /// Compares models by semantic contents instead of shared environment identity.
    ///
    /// The model `name` is treated as user-facing metadata and is **not** part of
    /// the comparison: two models with identical environments, objectives,
    /// constraints, and sense are content-equal even if their names differ. Use
    /// [`PartialEq`] when name equality also matters.
    fn equal_contents(&self, other: &Self) -> bool {
        self.environment.equal_contents(&other.environment)
            && self.objective.equal_contents(&other.objective)
            && self.constraints.equal_contents(&other.constraints)
            && self.sense == other.sense
    }
}

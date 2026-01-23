mod access;
mod creation;
mod equality;
mod specs;
mod modification;
mod substitution;
mod evaluation;
mod deepclone;
mod sense;

use std::fmt::{Debug, Display, Formatter};

use lunamodel_types::Sense;

use crate::{ArcEnv, ConstraintCollection, Expression};

/// A model describing some function to be optimized (objective) and restrictions
/// on this objective (constraints).
#[derive(Clone, Default)]
pub struct Model {
    /// The name of the model, by default is [DEFAULT_MODEL_NAME].
    pub name: String,
    /// The environment of the model, constaining the information for each variable
    /// used in both the objective and it's constraints.
    pub environment: ArcEnv,
    /// The objective of the model describing some optimization problem. The objective
    /// is an expression that can be linear, quadratic or higher order.
    pub objective: Expression,
    /// The constraints of the model describing the restrictions on the model.
    pub constraints: ConstraintCollection,
    /// The sense of the model, i.e., the direction to be optimized at.
    /// By default is set to [Sense::Min].
    pub sense: Sense,
}

impl Debug for Model {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Model")
            .field("name", &self.name)
            .field("objective", &self.objective)
            .field("constraints", &self.constraints)
            .field("environment_id", &self.environment.id())
            .finish()
    }
}

impl Display for Model {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        _ = f;
        unimplemented!()
        // f.write_str(ModelWriter::new(&self).to_string())
    }
}

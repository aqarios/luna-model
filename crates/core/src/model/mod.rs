mod access;
mod creation;
mod deepclone;
mod equality;
mod evaluation;
mod modification;
mod sense;
mod specs;
mod substitution;

use std::fmt::{Debug, Display, Formatter};

use lunamodel_types::Sense;

use crate::{ArcEnv, ConstraintCollection, Expression};

/// A model describing some function to be optimized (objective) and restrictions
/// on this objective (constraints).
#[derive(Clone)]
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

impl Default for Model {
    fn default() -> Self {
        let env = ArcEnv::default();
        Self {
            name: "unnamed".to_string(),
            objective: Expression::new(env.clone()),
            environment: env,
            constraints: ConstraintCollection::default(),
            sense: Sense::default(),
        }
    }
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

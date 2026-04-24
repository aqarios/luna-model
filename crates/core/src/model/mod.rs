//! Optimization models built from expressions, constraints, and an environment.

mod access;
mod creation;
mod deepclone;
mod equality;
mod evaluation;
mod modification;
mod specs;
mod substitution;

use std::fmt::{Debug, Formatter};

use lunamodel_types::Sense;

use crate::{ArcEnv, ConstraintCollection, Expression};

/// A complete optimization model.
///
/// A `Model` ties together the shared environment, objective expression,
/// constraints, and optimization sense. The environment is stored directly on
/// the model so any variable references created from the model continue to point
/// at the same underlying variable registry as the objective and constraints.
#[derive(Clone)]
pub struct Model {
    /// Human-readable model name. Defaults to `"unnamed"`.
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
    /// Creates an empty unnamed minimization model with a fresh environment.
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

use super::Model;
use crate::{ArcEnv, ConstraintCollection, Expression, model::Sense};

/// The default name for a model.
pub static DEFAULT_MODEL_NAME: &str = "unnamed";

impl Model {
    pub fn with_env(name: Option<String>, sense: Option<Sense>, env: ArcEnv) -> Self {
        Self {
            name: name.unwrap_or(String::from(DEFAULT_MODEL_NAME)),
            objective: Expression::empty(env.clone()),
            constraints: ConstraintCollection::default(),
            sense: sense.unwrap_or_else(|| Sense::default()),
            environment: env,
        }
    }

    pub fn new(name: Option<String>, sense: Option<Sense>) -> Self {
        Self::with_env(name, sense, ArcEnv::default())
    }
}

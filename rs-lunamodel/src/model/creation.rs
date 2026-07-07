use lunamodel_types::Sense;

use crate::{Env, Model, Result};

impl Model {
    /// Creates a new model with an optional name and optimization sense.
    /// `None` values fall back to the model defaults.
    pub fn new(name: Option<String>, sense: Option<Sense>) -> Result<Self> {
        out!(model_new(?!name, ?~sense))
    }

    pub fn with_env(name: Option<String>, sense: Option<Sense>, env: &Env) -> Result<Self> {
        out!(model_with_env(?!name, ?~sense, @env))
    }
}

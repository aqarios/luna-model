use lunamodel_cext_vtable::LmSense;
use lunamodel_types::Sense;

use crate::{
    Env, Model, Result,
    traits::{LmInto, TryAsOptPtr},
};

impl Model {
    /// Creates a new model with an optional name and optimization sense.
    /// `None` values fall back to the model defaults.
    pub fn new(name: Option<String>, sense: Option<Sense>) -> Result<Self> {
        let sense: Option<LmSense> = sense.map(|s| s.lm_into());
        out!(model_new(name.try_as_opt_ptr()?, ?sense))
    }

    pub fn with_env(name: Option<String>, sense: Option<Sense>, env: &Env) -> Result<Self> {
        let sense: Option<LmSense> = sense.map(|s| s.lm_into());
        out!(model_with_env(name.try_as_opt_ptr()?, ?sense, @env))
    }
}

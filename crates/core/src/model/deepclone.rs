//! Deep-cloning helpers for models.

use super::Model;

impl Model {
    /// Deep-clones the model into a fresh environment identity.
    ///
    /// Unlike ordinary `Clone`, this does not share the underlying environment.
    /// Instead, the environment, objective, and constraints are all re-rooted
    /// into a newly allocated environment with a different environment id.
    pub fn deep_clone(&self) -> Self {
        let new_env = self.environment.deep_clone();
        Self {
            name: self.name.clone(),
            sense: self.sense,
            objective: self.objective.deep_clone(new_env.clone()),
            constraints: self.constraints.deep_clone(new_env.clone()),
            environment: new_env,
        }
    }
}

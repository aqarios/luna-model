use super::Model;

impl Model {
    /// Deep clone a Model.
    ///
    /// This creates a new Model with a deep clone of the contained data.
    /// The SharedEnvironment is not just an increase of the reference counted environment
    /// but a new SharedEnvironment object with a deep cloned environment having a new
    /// environment id that is guaranteed to be different from all other possibly
    /// exisiting environments.
    pub fn deep_clone(&self) -> Self {
        let new_env = self.environment.deep_clone();
        Self {
            name: self.name.clone(),
            sense: self.sense.clone(),
            objective: self.objective.deep_clone(new_env.clone()),
            constraints: self.constraints.deep_clone(new_env.clone()),
            environment: new_env,
        }
    }
}

use std::{
    collections::HashMap,
    sync::{Mutex, OnceLock},
};

use lunamodel_core::Solution;
use lunamodel_error::LunaModelResult;

use crate::{artifact::ErasedArtifact, error::TransformationError, pass::ReversiblePass};

type BackwardFn = fn(&ErasedArtifact, Solution) -> LunaModelResult<Solution>;

/// Global registry of backwards functions.
///
/// Each pass type registers its backwards function at program startup.
#[derive(Default)]
pub struct BackwardRegistry {
    functions: HashMap<String, BackwardFn>,
}

impl BackwardRegistry {
    /// Register a backwards function for a pass type
    pub fn register<P: ReversiblePass>(&mut self, pass_name: &str) {
        self.functions
            .insert(pass_name.to_string(), |artifact, solution| {
                let typed_artifact = artifact.restore::<P::Artifact>()?;
                P::backward(&typed_artifact, solution)
            });
    }

    /// Apply the backwards transformation for a pass
    pub fn apply(
        &self,
        pass_name: &str,
        artifact: &ErasedArtifact,
        solution: Solution,
    ) -> LunaModelResult<Solution> {
        let backward_fn =
            self.functions
                .get(pass_name)
                .ok_or_else(|| TransformationError::UnregisteredPass {
                    name: pass_name.to_string(),
                })?;

        backward_fn(artifact, solution)
    }
}

/// Global singleton registry
static BACKWARD_REGISTRY: OnceLock<Mutex<BackwardRegistry>> = OnceLock::new();

pub fn register_backward<P: ReversiblePass>() {
    BACKWARD_REGISTRY
        .get_or_init(|| Mutex::new(BackwardRegistry::default()))
        .lock()
        .unwrap()
        .register::<P>(P::ID);
}

pub fn apply(
    pass_name: &str,
    artifact: &ErasedArtifact,
    solution: Solution,
) -> LunaModelResult<Solution> {
    BACKWARD_REGISTRY
        .get_or_init(|| Mutex::new(BackwardRegistry::default()))
        .lock()
        .unwrap()
        .apply(pass_name, artifact, solution)
}

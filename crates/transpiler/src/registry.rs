//! Global registry used to dispatch backwards handlers.

use std::{
    collections::HashMap,
    sync::{Mutex, OnceLock},
};

use lunamodel_core::Solution;

use crate::{
    artifact::ErasedArtifact,
    error::{TranspileErrorKind, TranspileKindResult},
    reversible::Reversible,
};

type BackwardFn = fn(&ErasedArtifact, Solution) -> TranspileKindResult<Solution>;

/// Global registry of backwards functions.
///
/// Each pass type registers its backwards function at program startup.
#[derive(Default)]
pub struct BackwardRegistry {
    functions: HashMap<String, BackwardFn>,
}

impl BackwardRegistry {
    /// Register a backwards function for a pass type
    pub fn register<P: Reversible>(&mut self, pass_name: &str) {
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
    ) -> TranspileKindResult<Solution> {
        let backward_fn = self.functions.get(pass_name).ok_or_else(|| {
            TranspileErrorKind::UnregisteredPass {
                name: pass_name.to_string(),
            }
        })?;

        backward_fn(artifact, solution)
    }
}

/// Global singleton registry
static BACKWARD_REGISTRY: OnceLock<Mutex<BackwardRegistry>> = OnceLock::new();

/// Registers the backward function for pass type `P` in the global registry.
pub fn register_backward<P: Reversible>() {
    BACKWARD_REGISTRY
        .get_or_init(|| Mutex::new(BackwardRegistry::default()))
        .lock()
        .unwrap()
        .register::<P>(P::ID);
}

/// Applies a registered backward function from the global registry.
pub fn apply(
    pass_name: &str,
    artifact: &ErasedArtifact,
    solution: Solution,
) -> TranspileKindResult<Solution> {
    BACKWARD_REGISTRY
        .get_or_init(|| Mutex::new(BackwardRegistry::default()))
        .lock()
        .unwrap()
        .apply(pass_name, artifact, solution)
}

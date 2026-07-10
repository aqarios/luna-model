//! Pass logic for eliminating inverted binary variables.

use lunamodel_core::{Environment, Model, Solution, prelude::VarRef};
use lunamodel_error::LunaModelResult;
use lunamodel_transpiler::{
    PassContext, PipelineStep, Reversible, TransformationPass, TranspileKindResult, transformation,
};
use lunamodel_types::{VarIdx, Vtype};

use super::ReduceInvertedBinaryPassArtifact;

#[transformation]
#[derive(Clone, Default)]
pub struct ReduceInvertedBinaryPass {}

impl ReduceInvertedBinaryPass {
    /// Creates a new reduction pass instance.
    pub fn new() -> Self {
        Self::default()
    }
}

impl TransformationPass for ReduceInvertedBinaryPass {
    fn name(&self) -> &str {
        "reduce-inverted-binary"
    }

    fn forward(
        &self,
        model: &mut Model,
        _ctx: &PassContext,
    ) -> TranspileKindResult<Self::Artifact> {
        let invs = inverted_varrefs(&model.environment.read_arc())?;
        for (partner_id, inverted_id) in invs {
            let partner = VarRef::new(partner_id, model.environment.clone());
            let inverted = VarRef::new(inverted_id, model.environment.clone());
            model.substitute(&inverted, &(1 - partner)?)?;
        }
        Ok(Self::Artifact {})
    }
}

impl Reversible for ReduceInvertedBinaryPass {
    type Artifact = ReduceInvertedBinaryPassArtifact;

    const ID: &'static str = "luna_model::reduce-inverted-binary";

    fn backward(_artifact: &Self::Artifact, solution: Solution) -> TranspileKindResult<Solution> {
        Ok(solution)
    }
}

fn inverted_varrefs(env: &Environment) -> LunaModelResult<Vec<(VarIdx, VarIdx)>> {
    let mut vpairs = Vec::new();

    for vid in env.vars() {
        let var = env.get(vid)?;
        if let Vtype::InvertedBinary = var.vtype() {
            vpairs.push((var.inverted.unwrap(), vid));
        }
    }

    Ok(vpairs)
}

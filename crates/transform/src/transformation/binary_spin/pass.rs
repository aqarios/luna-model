//! Pass logic for binary-to-spin conversion.

use std::ops::{Add, Mul};

use lunamodel_core::{Environment, Model, Solution, solution::Column};
use lunamodel_transpiler::{
    PassContext, PipelineStep, Reversible, TransformationPass, TranspileKindResult, transformation,
};
use lunamodel_types::{EnumSet, ModelPart, Vtype};
use sqids::Sqids;

use crate::error::{TransformError, TransformResult};

use super::artifact::BinarySpinPassArtifact;

#[transformation]
#[derive(Clone)]
pub struct BinarySpinPass {
    vtype: Vtype,
    prefix: Option<String>,
}

impl BinarySpinPass {
    pub fn vtype(&self) -> Vtype {
        self.vtype
    }

    pub fn prefix(&self) -> Option<String> {
        self.prefix.clone()
    }
}

impl BinarySpinPass {
    pub fn new(vtype: Vtype, prefix: Option<String>) -> Self {
        Self { vtype, prefix }
    }

    fn fill_artifact(
        &self,
        model: &Model,
        artifact: &mut <Self as Reversible>::Artifact,
    ) -> TransformResult<()> {
        let pref = self.prefix.clone().unwrap_or_else(|| match self.vtype {
            Vtype::Binary => "x".to_string(),
            Vtype::Spin => "s".to_string(),
            _ => unreachable!(),
        });

        let env: &mut Environment = &mut model.environment.write_arc();
        for x in env.vars() {
            let v_old = env.get(x)?;
            let mut new_name = format!("{}_{}", pref, v_old.name());
            if env.lookup(&new_name).is_ok() {
                // New name already exists
                let suffix = Sqids::default().encode(&[x.into()]).map_err(|e| {
                    TransformError::Transformation {
                        name: self.name().to_owned(),
                        msg: e.to_string(),
                    }
                })?;
                new_name = format!("{}_{}", new_name, suffix);
            }

            match (v_old.vtype(), self.vtype) {
                (Vtype::Binary, Vtype::Spin) | (Vtype::Spin, Vtype::Binary) => {
                    artifact.map.insert(v_old.name().to_string(), new_name);
                }
                _ => (),
            }
        }

        Ok(())
    }
}

impl TransformationPass for BinarySpinPass {
    fn name(&self) -> &str {
        "binary-spin"
    }

    fn modifies(&self) -> EnumSet<ModelPart> {
        ModelPart::Objective | ModelPart::Constraints | ModelPart::Variables
    }

    fn forward(
        &self,
        model: &mut Model,
        _ctx: &PassContext,
    ) -> TranspileKindResult<Self::Artifact> {
        let mut artifact = BinarySpinPassArtifact::try_new(self.vtype)?;
        Self::fill_artifact(self, model, &mut artifact)?;
        if artifact.map.is_empty() {
            return Ok(artifact);
        }

        // Create all new variables first, each at its old variable's position, so
        // the transformed model keeps the variable order of the input model.
        let old_vars = model.environment.read_arc().vars().collect::<Vec<_>>();
        let mut replacements = Vec::with_capacity(artifact.map.len());
        for x in old_vars {
            let v = model.environment.get(x);
            let Some(t) = artifact.map.get(&v.name()?) else {
                continue;
            };
            let var = model
                .environment
                .insert_before(&v, t, artifact.new_vtype, None)
                .map_err(|e| TransformError::Transformation {
                    name: self.name().to_owned(),
                    msg: format!("binary-spin insert ('{t}'): {e}"),
                })?;
            replacements.push((v, var));
        }

        // Then substitute the old variables with the existing new ones.
        for (v, var) in replacements {
            let expr = match artifact.new_vtype {
                Vtype::Spin => (&var).mul(-0.5)?.add(0.5)?,
                Vtype::Binary => (&var).mul(-2.0)?.add(1.0)?,
                _ => unreachable!("unexpected target vtype in binary-spin"),
            };

            model
                .substitute(&v, &expr)
                .map_err(|e| TransformError::Transformation {
                    name: self.name().to_owned(),
                    msg: format!("binary-spin substitute: {e}"),
                })?;
        }

        Ok(artifact)
    }
}

impl Reversible for BinarySpinPass {
    type Artifact = BinarySpinPassArtifact;

    const ID: &'static str = "luna_model::binary-spin";

    fn backward(
        artifact: &Self::Artifact,
        mut solution: Solution,
    ) -> TranspileKindResult<Solution> {
        if artifact.map.is_empty() {
            return Ok(solution);
        }

        for (old_name, new_name) in artifact.map.iter() {
            let old_col = match (artifact.old_vtype, solution.samples.get(new_name)) {
                (Vtype::Spin, Some(Column::Binary(inner))) => Some(Column::spin(
                    inner.iter().map(|x| (1 - 2 * (x as i8)) as f64).collect(),
                )),
                (Vtype::Binary, Some(Column::Spin(inner))) => Some(Column::binary(
                    inner.iter().map(|x| ((1 - x) as u8 / 2) as f64).collect(),
                )),
                (Vtype::Spin | Vtype::Binary, _) => None,
                _ => unreachable!("unexpected vtype"),
            };
            // Put the restored column at the position of the column it replaces.
            if let Some(col) = old_col
                && let Some(pos) = solution.samples.get_index_of(new_name)
            {
                solution.samples.shift_insert(pos, old_name.clone(), col);
            }
            solution.remove_col(new_name);
        }

        Ok(solution)
    }
}

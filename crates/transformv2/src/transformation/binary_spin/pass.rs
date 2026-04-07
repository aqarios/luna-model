use std::ops::{Add, Mul};

use lunamodel_core::{Environment, Model, Solution, solution::Column};
use lunamodel_error::{LunaModelError, LunaModelResult};
use lunamodel_transpiler::{PassContext, ReversiblePass};
use lunamodel_types::Vtype;
use sqids::Sqids;

use super::artifact::BinarySpinPassArtifact;

pub struct BinarySpinPass {
    vtype: Vtype,
    prefix: Option<String>,
}

impl BinarySpinPass
where
    Self: ReversiblePass,
{
    pub fn new(vtype: Vtype, prefix: Option<String>) -> Self {
        Self { vtype, prefix }
    }

    fn fill_artifact(
        &self,
        model: &Model,
        artifact: &mut <Self as ReversiblePass>::Artifact,
    ) -> LunaModelResult<()> {
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
                    LunaModelError::TransformationPass(self.name().into(), e.to_string().into())
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

impl ReversiblePass for BinarySpinPass {
    type Artifact = BinarySpinPassArtifact;

    const ID: &'static str = "lunamodel::binary-spin";

    fn name(&self) -> &str {
        "binary-spin"
    }

    fn forward(&self, model: &mut Model, _ctx: &PassContext) -> LunaModelResult<Self::Artifact> {
        let mut artifact = BinarySpinPassArtifact::try_new(self.vtype)?;
        Self::fill_artifact(&self, model, &mut artifact)?;
        if artifact.map.is_empty() {
            return Ok(artifact);
        }

        for (s, t) in artifact.map.iter() {
            let v = model.environment.lookup(s).map_err(|e| {
                LunaModelError::Compilation(format!("binary-spin lookup ('{s}'): {e}").into())
            })?;
            let var = model
                .environment
                .insert(t, artifact.new_vtype, None)
                .map_err(|e| {
                    LunaModelError::Compilation(format!("binary-spin insert ('{t}'): {e}").into())
                })?;

            let expr = match artifact.new_vtype {
                Vtype::Spin => (&var).mul(-0.5)?.add(0.5)?,
                Vtype::Binary => (&var).mul(-0.5)?.add(0.5)?,
                _ => unreachable!("unexpected target vtype in binary-spin"),
            };

            model.substitute(&v, &expr).map_err(|e| {
                LunaModelError::Compilation(format!("binary-spin substitute: {e}").into())
            })?;
        }

        Ok(artifact)
    }

    fn backward(artifact: &Self::Artifact, mut solution: Solution) -> LunaModelResult<Solution> {
        if artifact.map.is_empty() {
            return Ok(solution);
        }

        for (old_name, new_name) in artifact.map.iter() {
            match artifact.old_vtype {
                Vtype::Spin => {
                    if let Some(Column::Binary(inner)) = solution.samples.get(new_name) {
                        solution.samples.insert(
                            old_name.clone(),
                            Column::spin(
                                inner.iter().map(|x| (1 - 2 * (x as i8)) as f64).collect(),
                            ),
                        );
                    }
                }
                Vtype::Binary => {
                    if let Some(Column::Spin(inner)) = solution.samples.get(new_name) {
                        solution.samples.insert(
                            old_name.clone(),
                            Column::binary(
                                inner.iter().map(|x| ((1 - x) as u8 / 2) as f64).collect(),
                            ),
                        );
                    }
                }
                _ => unreachable!("unexpected vtype"),
            }
            solution.remove_col(new_name);
        }

        Ok(solution)
    }
}

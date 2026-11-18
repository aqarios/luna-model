//! Static requirement validation for pipeline step trees.

use std::{collections::HashMap, sync::Arc};

use lunamodel_types::{EnumSet, ModelPart};

use crate::{
    PassManager, PipelineStep, TranspileErrorKind, TranspileKindResult, TranspilerResult,
    erased::ErasedControlFlowPass, error::InvalidatedRequirement,
};

/// An analysis result that is available at a point in the pipeline.
#[derive(Clone)]
struct Provided {
    computed_by: String,
    depends_on: EnumSet<ModelPart>,
}

/// An analysis result that was available but got invalidated by a transformation.
#[derive(Clone)]
struct Invalidated {
    computed_by: String,
    invalidated_by: String,
    parts: EnumSet<ModelPart>,
}

/// Analysis results tracked while walking the pipeline.
#[derive(Clone, Default)]
pub(crate) struct Availability {
    valid: HashMap<String, Provided>,
    invalidated: HashMap<String, Invalidated>,
}

impl Availability {
    fn require(&self, pass_name: &str, requirement: &str) -> TranspileKindResult<()> {
        if self.valid.contains_key(requirement) {
            return Ok(());
        }
        Err(match self.invalidated.get(requirement) {
            Some(inv) => {
                TranspileErrorKind::InvalidatedRequirement(Box::new(InvalidatedRequirement {
                    pass_name: pass_name.to_owned(),
                    requirement: requirement.to_owned(),
                    computed_by: inv.computed_by.clone(),
                    invalidated_by: inv.invalidated_by.clone(),
                    parts: inv.parts,
                }))
            }
            None => TranspileErrorKind::UnsatisfiedRequirement {
                pass_name: pass_name.to_owned(),
                requirement: requirement.to_owned(),
            },
        })
    }

    fn modify(&mut self, pass_name: &str, modified: EnumSet<ModelPart>) {
        let (stale, valid) = std::mem::take(&mut self.valid)
            .into_iter()
            .partition(|(_, p)| !p.depends_on.is_disjoint(modified));
        self.valid = valid;
        for (name, p) in stale {
            let inv = Invalidated {
                computed_by: p.computed_by,
                invalidated_by: pass_name.to_owned(),
                parts: p.depends_on & modified,
            };
            self.invalidated.insert(name, inv);
        }
    }

    fn provide(&mut self, pass_name: &str, name: &str, depends_on: EnumSet<ModelPart>) {
        self.invalidated.remove(name);
        let provided = Provided {
            computed_by: pass_name.to_owned(),
            depends_on,
        };
        self.valid.insert(name.to_owned(), provided);
    }

    /// Keeps only results valid after every branch.
    fn merge(branches: Vec<Self>) -> Option<Self> {
        branches.into_iter().reduce(|mut acc, branch| {
            acc.valid.retain(|name, _| branch.valid.contains_key(name));
            for (name, inv) in branch.invalidated {
                acc.invalidated.entry(name).or_insert(inv);
            }
            acc
        })
    }
}

impl PassManager {
    pub(crate) fn validate_steps(
        &self,
        steps: &[PipelineStep],
        state: &mut Availability,
    ) -> TranspilerResult<()> {
        for step in steps {
            match step {
                PipelineStep::Transform(p) => {
                    require_all(state, p.name(), p.requires())?;
                    state.modify(p.name(), p.modifies());
                }
                PipelineStep::Analysis(p) => {
                    require_all(state, p.name(), p.requires())?;
                    state.provide(p.name(), p.provides(), p.depends_on());
                }
                PipelineStep::Composite(p) => {
                    require_all(state, p.name(), p.requires())?;
                    state.modify(p.name(), p.modifies());
                    state.provide(p.name(), p.provides(), p.depends_on());
                }
                PipelineStep::MetaAnalysis(p) => {
                    state.provide(p.name(), p.provides(), EnumSet::empty());
                }
                // Container steps, need recursing
                PipelineStep::Pipeline(p) => self.validate_steps(&p.steps, state)?,
                PipelineStep::ControlFlow(p) => self.validate_control_flow(p, state)?,
            }
        }
        Ok(())
    }

    fn validate_control_flow(
        &self,
        pass: &Arc<dyn ErasedControlFlowPass>,
        state: &mut Availability,
    ) -> TranspilerResult<()> {
        let branches = pass.branches();

        if branches.is_empty() {
            require_all(state, pass.name(), pass.requires())?;
            state.modify(pass.name(), pass.modifies());
            for name in pass.provides() {
                state.provide(pass.name(), name, EnumSet::all());
            }
            return Ok(());
        }

        let mut outcomes = Vec::with_capacity(branches.len());
        for branch in branches {
            let mut branch_state = state.clone();
            self.validate_steps(branch, &mut branch_state)?;
            outcomes.push(branch_state);
        }
        if let Some(merged) = Availability::merge(outcomes) {
            *state = merged;
        }
        Ok(())
    }
}

fn require_all(state: &Availability, pass_name: &str, requires: &[String]) -> TranspilerResult<()> {
    for req in requires {
        state.require(pass_name, req)?;
    }
    Ok(())
}

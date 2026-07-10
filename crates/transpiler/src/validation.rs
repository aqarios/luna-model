//! Static requirement validation for pipeline step trees.

use std::{collections::HashSet, sync::Arc};

use lunamodel_error::LunaModelResult;

use crate::{
    PassManager, PipelineStep, TranspileErrorKind, TranspilerResult, erased::ErasedControlFlowPass,
};

impl PassManager {
    pub(crate) fn validate_steps(
        &self,
        steps: &[PipelineStep],
        satisfied: &mut HashSet<String>,
    ) -> TranspilerResult<()> {
        for step in steps {
            match step {
                // Trivial steps
                PipelineStep::Transform(p) => check_leaf(
                    p.name(),
                    p.requires(),
                    p.invalidates(),
                    std::iter::empty(),
                    satisfied,
                )?,
                PipelineStep::Analysis(p) => check_leaf(
                    p.name(),
                    p.requires(),
                    std::iter::empty(),
                    std::iter::once(p.provides()),
                    satisfied,
                )?,
                PipelineStep::Composite(p) => check_leaf(
                    p.name(),
                    p.requires(),
                    p.invalidates(),
                    std::iter::once(p.provides()),
                    satisfied,
                )?,
                PipelineStep::MetaAnalysis(p) => check_leaf(
                    p.name(),
                    std::iter::empty(),
                    std::iter::empty(),
                    std::iter::once(p.provides()),
                    satisfied,
                )?,
                // Container steps, need recursing
                PipelineStep::Pipeline(p) => self.validate_steps(&p.steps, satisfied)?,
                PipelineStep::ControlFlow(p) => self.validate_control_flow(p, satisfied)?,
            }
        }
        Ok(())
    }

    fn validate_control_flow(
        &self,
        pass: &Arc<dyn ErasedControlFlowPass>,
        satisfied: &mut HashSet<String>,
    ) -> LunaModelResult<()> {
        let branches = pass.branches();

        if branches.is_empty() {
            return check_leaf(
                pass.name(),
                pass.requires(),
                pass.invalidates(),
                pass.provides().iter().map(|s| s.as_str()),
                satisfied,
            );
        }

        // Check each branch.
        // Keep only branch intersection guarantees.
        let mut merged: Option<HashSet<String>> = None;
        for branch in branches {
            let mut branch_sat = satisfied.clone();
            self.validate_steps(branch, &mut branch_sat)?;
            merged = Some(match merged {
                None => branch_sat,
                Some(acc) => &acc & &branch_sat,
            });
        }
        if let Some(m) = merged {
            *satisfied = m;
        }
        for inv in pass.invalidates() {
            satisfied.remove(inv);
        }
        satisfied.insert(pass.name().to_string());
        Ok(())
    }
}

fn check_leaf<'l>(
    name: &str,
    requires: impl IntoIterator<Item = &'l String>,
    invalidates: impl IntoIterator<Item = &'l String>,
    provides: impl IntoIterator<Item = &'l str>,
    satisfied: &mut HashSet<String>,
) -> LunaModelResult<()> {
    for req in requires {
        if !satisfied.contains(req) {
            return Err(TranspileErrorKind::UnsatisfiedRequirement {
                pass_name: name.to_owned(),
                requirement: req.to_string(),
            }
            .into());
        }
    }
    for inv in invalidates {
        satisfied.remove(inv);
    }
    satisfied.insert(name.to_string());
    satisfied.extend(provides.into_iter().map(|s| s.to_owned()));
    Ok(())
}

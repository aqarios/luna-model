use lunamodel_core::Model;
use lunamodel_transpiler::{
    AnalysisKey, AnalysisPass, PassContext, PipelineStep, TranspileKindResult, analysis,
};
use lunamodel_types::{Bound, Comparator};

use crate::{
    analysis::utils::{compute_maxvalue, compute_minvalue},
    error::TransformError,
};

#[analysis]
#[derive(Clone, Default)]
pub struct CheckInfeasibleConstraintsAnalysis;

pub struct Ignore;

impl AnalysisPass for CheckInfeasibleConstraintsAnalysis {
    type Result = ();

    const PROVIDES: &'static str = "luna_model::check-infeasibile-constraints";

    fn name(&self) -> &str {
        "check-infeasibile-constraints"
    }

    fn key<Ignore>() -> AnalysisKey<Ignore> {
        AnalysisKey::new(Self::PROVIDES.to_string())
    }

    fn run(&self, model: &Model, _: &PassContext) -> TranspileKindResult<Self::Result> {
        for (name, constr) in model.constraints.iter() {
            let infeasible = match constr.comparator {
                // lhs <= rhs : lower is important; infeasible if lower > rhs
                Comparator::Le => match compute_minvalue(constr.lhs.linear_items())? {
                    Bound::Bounded(lower) => lower > constr.rhs,
                    Bound::Unbounded => false,
                },

                // lhs >= rhs : upper is important; infeasible if upper < rhs
                Comparator::Ge => match compute_maxvalue(constr.lhs.linear_items())? {
                    Bound::Bounded(upper) => upper < constr.rhs,
                    Bound::Unbounded => false,
                },

                // lhs == rhs : infeasible if rhs < lower || rhs > upper
                Comparator::Eq => {
                    let lo = compute_minvalue(constr.lhs.linear_items())?;
                    let hi = compute_maxvalue(constr.lhs.linear_items())?;
                    match (lo, hi) {
                        (Bound::Bounded(lower), Bound::Bounded(upper)) => {
                            constr.rhs < lower || constr.rhs > upper
                        }
                        _ => false,
                    }
                }
            };

            if infeasible {
                return Err(TransformError::Infeasible {
                    location: self.name().to_owned(),
                    reason: format!(
                        "constraint '{name}' is infeasible: LHS range cannot satisfy '{} {}'",
                        constr.comparator, constr.rhs
                    ),
                })?;
            }
        }
        Ok(())
    }
}

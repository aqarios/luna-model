use lunamodel_error::{LunaModelError, LunaModelResult};

use super::Solution;
use crate::{Model, Timing};

impl Solution {
    /// Merges multiple solutions that share the same variable schema and sense.
    ///
    /// The merge is performed by appending the column-oriented storage for each
    /// participating solution and then aggregating duplicate rows. If `model` is
    /// provided, the merged solution is re-evaluated afterwards so derived fields
    /// such as objective values and feasibility reflect the final merged state.
    pub fn merge_many(solutions: &[Solution], model: &Option<Model>) -> LunaModelResult<Solution> {
        if solutions.is_empty() {
            return Ok(Solution::default());
        }

        let mut merged = solutions[0].clone();

        for solution in solutions.iter().skip(1) {
            // Let's check if compatible.
            if merged.sense != solution.sense {
                return Err(LunaModelError::UnsupportedOperation(
                    "solutions with different sense cannot be merged.".into(),
                ));
            }
            let mut merged_names = merged.variable_names();
            merged_names.sort();
            let mut sol_names = solution.variable_names();
            sol_names.sort();
            if merged_names != sol_names {
                return Err(LunaModelError::UnsupportedOperation(
                    "solutions with different variables cannot be merged.".into(),
                ));
            }

            for (var, col) in solution.samples.iter() {
                if let Some(curr) = merged.samples.get_mut(var) {
                    for val in col.as_assignments() {
                        curr.push(val)?;
                    }
                } else {
                    merged.samples[var] = col.clone();
                }
            }

            for (constr, oks) in solution.constraints.iter() {
                if let Some(curr) = merged.constraints.get_mut(constr) {
                    for &val in oks {
                        curr.push(val);
                    }
                } else {
                    merged.constraints.insert(constr.clone(), oks.clone());
                }
            }

            for (var, bounds) in solution.variable_bounds.iter() {
                if let Some(curr) = merged.variable_bounds.get_mut(var) {
                    for &val in bounds {
                        curr.push(val);
                    }
                } else {
                    merged.variable_bounds.insert(var.clone(), bounds.clone());
                }
            }

            merged.counts.append(&mut solution.counts.clone());

            match solution.raw_energies.as_ref() {
                None => merged.raw_energies = None,
                Some(e) => {
                    if let Some(me) = merged.raw_energies.as_mut() {
                        me.append(&mut e.clone());
                    }
                }
            }

            match solution.obj_values.as_ref() {
                None => merged.obj_values = None,
                Some(e) => {
                    if let Some(me) = merged.obj_values.as_mut() {
                        me.append(&mut e.clone());
                    }
                }
            }

            match solution.feasible.as_ref() {
                None => merged.feasible = None,
                Some(e) => {
                    if let Some(me) = merged.feasible.as_mut() {
                        me.append(&mut e.clone());
                    }
                }
            }

            match solution.timing {
                None => merged.timing = None,
                Some(t) => {
                    if let Some(mt) = merged.timing.as_mut() {
                        *mt = Timing::new(
                            t.start().min(mt.start()),
                            t.end().max(mt.end()),
                            match (t.qpu, mt.qpu) {
                                (Some(tq), Some(mtq)) => Some(tq + mtq),
                                (Some(tq), None) => Some(tq),
                                (None, Some(mtq)) => Some(mtq),
                                (None, None) => None,
                            },
                        )
                    }
                }
            }
        }

        merged.aggregate()?;
        if let Some(m) = model {
            merged = m.evaluate_solution(&merged)?;
        }
        Ok(merged)
    }
}

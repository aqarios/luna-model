//! Read-only accessors for models.

use lunamodel_error::LunaModelResult;
use lunamodel_types::Vtype;
use lunamodel_utils::{unique, unique_by};

use crate::{ConstraintCollection, prelude::VarRef, solution::sample::SampleView};

use super::Model;

impl Model {
    /// Returns the distinct variable types used by the model.
    ///
    /// This is derived from both the objective and all constraints, so it
    /// reflects the actual symbolic content of the model rather than every
    /// variable merely present in the environment.
    pub fn vtypes(&self) -> impl Iterator<Item = Vtype> {
        unique(self.objective.vtypes().chain(self.constraints.vtypes()))
    }

    /// Returns the number of distinct variables that actually participate in the model.
    ///
    /// This can be smaller than the number of variables stored in the
    /// environment because unused variables are ignored.
    pub fn num_variables(&self) -> usize {
        unique_by(self.objective.vars().chain(self.constraints.vars()), |e| {
            e.id
        })
        .count()
    }

    /// Iterates over the distinct variables referenced by the objective or constraints.
    pub fn vars(&self) -> impl Iterator<Item = VarRef> {
        let objvars = self.objective.vars();
        let constrvars = self.constraints.vars();

        unique_by(objvars.chain(constrvars), |e| e.id())
    }

    /// Looks up a variable by name in the model environment.
    pub fn var(&self, name: &str) -> LunaModelResult<VarRef> {
        self.environment.lookup(name)
    }

    /// Returns the subset of constraints violated by `sample`.
    ///
    /// The returned collection preserves the original constraint names.
    pub fn violated_constraints(
        &self,
        sample: &SampleView,
        tol: Option<f64>,
    ) -> LunaModelResult<ConstraintCollection> {
        let mut cs = ConstraintCollection::default();
        for (cname, c) in self.constraints.iter() {
            let ok = c.evaluate_sample(sample, tol)?;
            if !ok {
                cs.add_constraint(c.clone(), Some(cname.to_string()))?;
            }
        }
        Ok(cs)
    }
}

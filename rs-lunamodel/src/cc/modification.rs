use crate::{Constraint, Expression, Result, TryIndex, VarRef};

use super::{ConstraintCollection, editor::ConstraintCollectionEditor};

impl ConstraintCollection {
    pub fn iter_mut(&mut self) -> Result<ConstraintCollectionEditor<'_>> {
        let items = self.iter()?.collect();
        Ok(ConstraintCollectionEditor {
            target: self,
            items,
            committed: false,
        })
    }

    pub fn add_many_constraints<I>(&mut self, constraints: I) -> Result<Vec<String>>
    where
        I: IntoIterator<Item = (Constraint, Option<String>)>,
    {
        constraints
            .into_iter()
            .map(|(constraint, name)| self.add_constraint(&constraint, name.as_deref()))
            .collect()
    }

    pub fn evaluate_sample<S>(
        &self,
        sample: &S,
        tol: Option<f64>,
    ) -> Result<std::collections::HashMap<String, bool>>
    where
        for<'a> S: TryIndex<&'a str, Output = f64>,
    {
        let vars = self.complete_vars()?;
        let len = vars
            .iter()
            .map(|var| var.id())
            .collect::<Result<Vec<_>>>()?
            .into_iter()
            .max()
            .map_or(0, |id| id as usize + 1);
        let mut lookup = vec![0.0; len];
        for var in vars {
            lookup[var.id()? as usize] = sample.try_index(var.name()?.as_str())?;
        }
        self.evaluate_sample_quick(&lookup, tol)
    }

    pub fn evaluate_sample_quick(
        &self,
        lookup: &[f64],
        tol: Option<f64>,
    ) -> Result<std::collections::HashMap<String, bool>> {
        let values: std::collections::HashMap<String, Vec<bool>> = out!(
            cc_evaluate_sample_quick(@self, lookup.as_ptr(), lookup.len(), ?tol)
        )?;
        values
            .into_iter()
            .map(|(name, values)| {
                values
                    .into_iter()
                    .next()
                    .map(|value| (name, value))
                    .ok_or_else(|| crate::Error {
                        status: crate::Status::InvalidArgument,
                        message: "C API returned an empty constraint evaluation".to_owned(),
                    })
            })
            .collect()
    }

    pub fn add_constraint(&mut self, constr: &Constraint, name: Option<&str>) -> Result<String> {
        out!(cc_add_constraint(@mut self, @constr, ?!name))
    }

    pub fn add_collection(&mut self, other: &Self, prefix: Option<&str>) -> Result<Vec<String>> {
        out!(cc_add_collection(@mut self, @other, ?!prefix))
    }

    pub fn set_constraint(&mut self, key: &str, constr: &Constraint) -> Result<()> {
        check!(cc_set_constraint(@mut self, !key, @constr))
    }

    pub fn remove_constraint(&mut self, key: &str) -> Result<()> {
        check!(cc_remove_constraint(@mut self, !key))
    }

    pub fn substitute(&mut self, target: &VarRef, replacement: &Expression) -> Result<()> {
        check!(cc_substitute(@mut self, @target, @replacement))
    }
}

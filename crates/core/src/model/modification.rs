//! Mutation operations for models.

use lunamodel_error::{LunaModelError, LunaModelResult};
use lunamodel_types::{Sense, Vtype};

use std::ops::Deref;

use crate::{
    Constraint, ConstraintCollection, Expression,
    bounds::LazyBounds,
    environment::ArcEnv,
    ops::{
        EnvIdexable, LmAddAssign, LmMulAssign, LmPowAssign, LmSubAssign, check_constr_env,
        foreign_env_error,
    },
    variable::VarRef,
};

use super::Model;

impl Model {
    /// Adds a variable to the model environment.
    pub fn add_var(
        &mut self,
        name: &str,
        vtype: Vtype,
        bounds: Option<LazyBounds>,
    ) -> LunaModelResult<VarRef> {
        self.environment.insert(name, vtype, bounds)
    }

    /// Adds a variable, deriving a fallback name if the requested name is unavailable.
    pub fn add_var_with_fallback(
        &mut self,
        name: &str,
        vtype: Vtype,
        bounds: Option<LazyBounds>,
        enc: Option<&[u64]>,
    ) -> LunaModelResult<VarRef> {
        self.environment
            .insert_with_fallback(name, vtype, bounds, enc)
    }

    /// Replaces the objective expression and optionally updates the optimization sense.
    ///
    /// Fails if the expression belongs to a different environment than the model.
    pub fn set_objective(&mut self, expr: Expression, sense: Option<Sense>) -> LunaModelResult<()> {
        if expr.env_id() != self.env_id() {
            return Err(foreign_env_error("objective", &expr));
        }
        if let Some(s) = sense {
            self.sense = s;
        }
        self.objective = expr;
        Ok(())
    }

    /// Mutable access to the objective for in-place arithmetic.
    pub fn objective_mut(&mut self) -> ObjectiveMut<'_> {
        ObjectiveMut(&mut self.objective)
    }

    /// Replaces the constraints collection.
    ///
    /// Fails if any constraint belongs to a different environment than the model.
    pub fn set_constraints(&mut self, col: ConstraintCollection) -> LunaModelResult<()> {
        for (name, c) in col.iter() {
            check_constr_env(self, c, Some(name))?;
        }
        self.constraints = col;
        Ok(())
    }

    /// Adds a constraint and returns its final name.
    ///
    /// Fails if the constraint belongs to a different environment than the model.
    pub fn add_constraint(
        &mut self,
        constr: Constraint,
        name: Option<String>,
    ) -> LunaModelResult<String> {
        check_constr_env(self, &constr, name.as_deref())?;
        self.constraints.insert(constr, name)
    }

    /// Adds many constraints and returns their final names.
    ///
    /// Either all constraints are added or, on error, none are.
    pub fn add_many_constraints(
        &mut self,
        constrs: impl IntoIterator<Item = (Constraint, Option<String>)>,
    ) -> LunaModelResult<Vec<String>> {
        let constrs: Vec<_> = constrs.into_iter().collect();
        for (c, name) in &constrs {
            check_constr_env(self, c, name.as_deref())?;
        }
        self.constraints.insert_many(constrs)
    }

    /// Replaces the constraint stored under `key`.
    ///
    /// Fails if the constraint belongs to a different environment than the model.
    pub fn set_constraint(&mut self, key: &str, constr: Constraint) -> LunaModelResult<()> {
        if !self.constraints.contains(key) {
            return Err(LunaModelError::NoConstraintForKey(key.to_string().into()));
        }
        check_constr_env(self, &constr, Some(key))?;
        self.constraints.replace(key, constr)
    }

    /// Edits every constraint in place, e.g. for model transformations.
    ///
    /// The closure also gets the model environment, so it can add variables
    /// (such as slack variables). Each constraint is edited on a copy that only
    /// replaces the original once the closure succeeded and the result still
    /// belongs to the model environment. On error, the constraint that failed
    /// is left unchanged; constraints edited before it keep their changes.
    pub fn update_constraints<E: From<LunaModelError>>(
        &mut self,
        mut f: impl FnMut(&mut ArcEnv, &str, &mut Constraint) -> Result<(), E>,
    ) -> Result<(), E> {
        let env_id = self.environment.id();
        for (name, constr) in self.constraints.iter_mut() {
            let mut updated = constr.clone();
            f(&mut self.environment, name, &mut updated)?;
            if updated.env_id() != env_id {
                return Err(
                    foreign_env_error(&format!("constraint '{name}'"), &updated.lhs).into(),
                );
            }
            *constr = updated;
        }
        Ok(())
    }

    /// Removes the constraint stored under `key`.
    pub fn remove_constraint(&mut self, key: &str) -> LunaModelResult<()> {
        self.constraints.remove_constraint(key)
    }
}

/// Mutable handle to a model's objective.
///
/// Only allows in-place arithmetic, which keeps the objective in the model's
/// environment. Use [Model::set_objective] to replace the objective.
pub struct ObjectiveMut<'a>(&'a mut Expression);

impl Deref for ObjectiveMut<'_> {
    type Target = Expression;

    fn deref(&self) -> &Expression {
        self.0
    }
}

impl<T> LmAddAssign<T> for ObjectiveMut<'_>
where
    Expression: LmAddAssign<T>,
{
    fn add_assign(&mut self, rhs: T) -> LunaModelResult<()> {
        self.0.add_assign(rhs).map_err(objective_context)
    }
}

impl<T> LmSubAssign<T> for ObjectiveMut<'_>
where
    Expression: LmSubAssign<T>,
{
    fn sub_assign(&mut self, rhs: T) -> LunaModelResult<()> {
        self.0.sub_assign(rhs).map_err(objective_context)
    }
}

impl<T> LmMulAssign<T> for ObjectiveMut<'_>
where
    Expression: LmMulAssign<T>,
{
    fn mul_assign(&mut self, rhs: T) -> LunaModelResult<()> {
        self.0.mul_assign(rhs).map_err(objective_context)
    }
}

/// Adds "objective" to a bare different-environments error from in-place arithmetic.
fn objective_context(err: LunaModelError) -> LunaModelError {
    match err {
        LunaModelError::DifferentEnvironments => LunaModelError::DifferentEnvironmentsIn(
            "objective cannot be combined with a term from a different environment".into(),
        ),
        other => other,
    }
}

impl LmPowAssign for ObjectiveMut<'_> {
    fn pow_assign(&mut self, exp: usize) -> LunaModelResult<()> {
        self.0.pow_assign(exp)
    }
}

#[cfg(test)]
mod tests {
    use lunamodel_error::LunaModelError;
    use lunamodel_types::{Comparator, Vtype};

    use crate::{Constraint, Model};

    #[test]
    fn update_constraints_keeps_constraint_on_foreign_env() {
        let mut model = Model::default();
        let x = model.add_var("x", Vtype::Binary, None).unwrap();
        let lhs = (&x * 1.0).unwrap();
        model
            .add_constraint(
                Constraint::new(lhs, 1.0, Comparator::Le, None).unwrap(),
                Some("c".into()),
            )
            .unwrap();
        let before = model.constraints().get("c").unwrap().clone();

        let mut other = Model::default();
        let a = other.add_var("a", Vtype::Binary, None).unwrap();
        let res = model.update_constraints(|_, _, c| {
            *c = Constraint::new((&a * 1.0)?, 0.0, Comparator::Le, None)?;
            Ok::<_, LunaModelError>(())
        });

        assert!(matches!(
            res,
            Err(LunaModelError::DifferentEnvironmentsIn(_))
        ));
        let after = model.constraints().get("c").unwrap();
        assert_eq!(after.lhs, before.lhs);
        assert_eq!(after.rhs, before.rhs);
    }

    #[test]
    fn update_constraints_keeps_constraint_on_closure_error() {
        let mut model = Model::default();
        let x = model.add_var("x", Vtype::Binary, None).unwrap();
        let lhs = (&x * 1.0).unwrap();
        model
            .add_constraint(
                Constraint::new(lhs, 1.0, Comparator::Le, None).unwrap(),
                Some("c".into()),
            )
            .unwrap();

        let res = model.update_constraints(|_, _, c| {
            c.rhs = 5.0;
            Err(LunaModelError::Internal("fail".to_string().into()))
        });

        assert!(res.is_err());
        assert_eq!(model.constraints().get("c").unwrap().rhs, 1.0);
    }
}

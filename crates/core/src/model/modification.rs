//! Mutation operations for models.

use lunamodel_error::{LunaModelError, LunaModelResult};
use lunamodel_types::{Bias, Sense, VarIdx, Vtype};

use std::ops::Deref;

use crate::{
    Constraint, ConstraintCollection, Expression,
    bounds::LazyBounds,
    environment::ArcEnv,
    expression::term::{HigherOrder, Linear, Quadratic},
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
        expr.check_living()?;
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

    /// Adds every constraint of another collection, optionally prefixing the incoming names.
    ///
    /// Either all constraints are added or, on error, none are.
    pub fn add_collection(
        &mut self,
        other: ConstraintCollection,
        prefix: Option<String>,
    ) -> LunaModelResult<Vec<String>> {
        self.add_many_constraints(other.into_iter().map(|(name, constr)| {
            let name = match prefix.as_ref() {
                Some(p) => format!("{p}{name}"),
                None => name,
            };
            (constr, Some(name))
        }))
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
            updated.lhs.check_living()?;
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

/// Raw term edits by variable index.
///
/// Unlike arithmetic with [VarRef]s, raw indices carry no environment, so each
/// edit first checks that every index names a living variable of the model.
/// A failed edit leaves the objective unchanged.
impl ObjectiveMut<'_> {
    /// Replaces the constant offset.
    pub fn set_offset(&mut self, offset: Bias) {
        self.0.offset = offset;
    }

    /// Adds one linear contribution.
    pub fn add_linear_term(&mut self, var: VarIdx, bias: Bias) -> LunaModelResult<()> {
        self.check_ids([var])?;
        self.0.linear += (var, bias);
        Ok(())
    }

    /// Adds one quadratic contribution.
    pub fn add_quadratic_term(&mut self, u: VarIdx, v: VarIdx, bias: Bias) -> LunaModelResult<()> {
        self.check_ids([u, v])?;
        *self.0.quadratic.get_or_insert_default() += (u, v, bias);
        Ok(())
    }

    /// Adds one higher-order contribution.
    pub fn add_higher_order_term(&mut self, vars: &[VarIdx], bias: Bias) -> LunaModelResult<()> {
        self.check_ids(vars.iter().copied())?;
        *self.0.higher_order.get_or_insert_default() += (vars, bias);
        Ok(())
    }

    /// Replaces the linear term.
    pub fn set_linear(&mut self, linear: Linear) -> LunaModelResult<()> {
        self.check_ids(linear.iter().map(|(idx, _)| idx))?;
        self.0.linear = linear;
        Ok(())
    }

    /// Replaces the quadratic term, or removes it with `None`.
    pub fn set_quadratic(&mut self, quadratic: Option<Quadratic>) -> LunaModelResult<()> {
        if let Some(q) = &quadratic {
            self.check_ids(q.iter_flat().flat_map(|(u, v, _)| [u, v]))?;
        }
        self.0.quadratic = quadratic;
        Ok(())
    }

    /// Replaces the higher-order term, or removes it with `None`.
    pub fn set_higher_order(&mut self, higher_order: Option<HigherOrder>) -> LunaModelResult<()> {
        if let Some(h) = &higher_order {
            self.check_ids(h.iter().flat_map(|(vars, _)| vars.iter().copied()))?;
        }
        self.0.higher_order = higher_order;
        Ok(())
    }

    /// Fails if any index does not name a living variable of the objective's environment.
    fn check_ids(&self, ids: impl IntoIterator<Item = VarIdx>) -> LunaModelResult<()> {
        let env = self.0.env.env.read_arc();
        ids.into_iter().try_for_each(|id| env.get(id).map(|_| ()))
    }
}

#[cfg(test)]
mod tests {
    use lunamodel_error::LunaModelError;
    use lunamodel_types::{Comparator, Vtype};

    use crate::{Constraint, ConstraintCollection, Model};

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

    #[test]
    fn objective_raw_term_edits_reject_unknown_variables() {
        let mut model = Model::default();
        let x = model.add_var("x", Vtype::Binary, None).unwrap();
        let mut objective = model.objective_mut();
        objective.add_linear_term(x.id, 1.0).unwrap();

        assert!(matches!(
            objective.add_linear_term(99, 1.0),
            Err(LunaModelError::VariableNotExisting(_))
        ));
        assert!(objective.add_quadratic_term(x.id, 99, 1.0).is_err());
        assert!(
            objective
                .add_higher_order_term(&[x.id, 98, 99], 1.0)
                .is_err()
        );

        let mut linear = model.objective().linear.clone();
        linear += (99, 2.0);
        assert!(model.objective_mut().set_linear(linear).is_err());

        let objective = model.objective();
        assert_eq!(
            objective.linear.iter().collect::<Vec<_>>(),
            vec![(x.id, 1.0)]
        );
        assert!(objective.quadratic.is_none());
        assert!(objective.higher_order.is_none());
    }

    #[test]
    fn objective_raw_term_edits_reject_removed_variables() {
        let mut model = Model::default();
        let x = model.add_var("x", Vtype::Binary, None).unwrap();
        let y = model.add_var("y", Vtype::Binary, None).unwrap();
        model.substitute(&x, &(&y * 1.0).unwrap()).unwrap();

        assert!(model.objective_mut().add_linear_term(x.id, 1.0).is_err());
        model.objective_mut().add_linear_term(y.id, 1.0).unwrap();
    }

    #[test]
    fn add_collection_rejects_foreign_constraints_atomically() {
        let mut model = Model::default();
        let x = model.add_var("x", Vtype::Binary, None).unwrap();
        let mut other = Model::default();
        let a = other.add_var("a", Vtype::Binary, None).unwrap();

        let mut col = ConstraintCollection::default();
        col.insert(
            Constraint::new((&x * 1.0).unwrap(), 1.0, Comparator::Le, None).unwrap(),
            Some("own".into()),
        )
        .unwrap();
        let mut foreign = ConstraintCollection::default();
        foreign
            .insert(
                Constraint::new((&a * 1.0).unwrap(), 1.0, Comparator::Le, None).unwrap(),
                Some("foreign".into()),
            )
            .unwrap();

        assert!(matches!(
            model.add_collection(foreign, None),
            Err(LunaModelError::DifferentEnvironmentsIn(_))
        ));
        assert!(model.constraints().is_empty());

        let names = model.add_collection(col, Some("p_".into())).unwrap();
        assert_eq!(names, vec!["p_own".to_string()]);
    }
}

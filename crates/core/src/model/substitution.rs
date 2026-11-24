//! Variable-substitution helpers for models.

use std::collections::HashMap;

use lunamodel_error::{LunaModelError, LunaModelResult};

use crate::{
    expression::Expression,
    ops::{EnvIdexable, foreign_env_error},
    variable::VarRef,
};

use super::Model;

impl Model {
    /// Substitutes a variable throughout the objective and all constraints.
    ///
    /// If the replacement no longer references the original variable, the now
    /// unused variable is removed from the environment.
    ///
    /// Fails without changing the model if `target` or `replacement` belong to
    /// a different environment than the model.
    pub fn substitute(&mut self, target: &VarRef, replacement: &Expression) -> LunaModelResult<()> {
        self.check_substitution(target, replacement)?;
        self.objective = self.objective.substitute(target, replacement)?;
        self.constraints.substitute(target, replacement)?;
        if !replacement.contains(target) {
            self.environment.remove(target)?;
        }
        Ok(())
    }

    /// Substitutes many variables in one walk over the objective and the
    /// constraints, and removes every target from the environment.
    ///
    /// The result equals calling [`Self::substitute`] once per pair, at the cost
    /// of a single call. For that, no replacement may contain a target and no
    /// target may appear twice: resolve chains such as `x -> y, y -> 0` into
    /// `x -> 0, y -> 0` first.
    ///
    /// Fails without changing the model if a pair breaks these rules, belongs
    /// to a different environment than the model, or names a removed variable.
    pub fn substitute_many(&mut self, subs: &[(VarRef, Expression)]) -> LunaModelResult<()> {
        let mut map = HashMap::with_capacity(subs.len());
        for (target, replacement) in subs {
            self.check_substitution(target, replacement)?;
            target.check_living()?;
            if map.insert(target.id(), replacement).is_some() {
                return Err(LunaModelError::UnsupportedOperation(
                    format!("{target} is substituted more than once").into(),
                ));
            }
        }
        for (target, replacement) in subs {
            if let Some(v) = replacement.vars().find(|v| map.contains_key(&v.id())) {
                return Err(LunaModelError::UnsupportedOperation(
                    format!(
                        "the replacement of {target} contains the substituted variable {v}; \
                         substitute into the replacements first"
                    )
                    .into(),
                ));
            }
        }
        self.objective = self.objective.substitute_with(|id| map.get(&id).copied())?;
        for (_, constr) in self.constraints.iter_mut() {
            let lhs = constr.lhs.substitute_with(|id| map.get(&id).copied())?;
            constr.set_lhs(lhs);
        }
        self.environment
            .write_arc()
            .retain(|id| !map.contains_key(&id));
        Ok(())
    }

    /// Fails if `target` or `replacement` belong to another environment.
    fn check_substitution(&self, target: &VarRef, replacement: &Expression) -> LunaModelResult<()> {
        if target.env_id() != self.env_id() {
            let name = target.name().unwrap_or_default();
            return Err(LunaModelError::DifferentEnvironmentsIn(
                format!("substitution target '{name}' belongs to a different environment").into(),
            ));
        }
        if replacement.env_id() != self.env_id() {
            return Err(foreign_env_error("substitution replacement", replacement));
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use lunamodel_types::{Comparator, Vtype};

    use crate::{Constraint, Expression, Model, traits::ContentEquality};

    fn model() -> Model {
        let mut m = Model::default();
        let [a, b, c, d] =
            ["a", "b", "c", "d"].map(|n| m.add_var(n, Vtype::Integer, None).unwrap().id());
        let mut obj = m.objective_mut();
        obj.add_linear_term(a, 1.0).unwrap();
        obj.add_quadratic_term(a, b, 2.0).unwrap();
        obj.add_quadratic_term(b, b, 3.0).unwrap();
        obj.add_higher_order_term(&[a, c, d], 4.0).unwrap();
        let lhs = m.objective().clone();
        m.add_constraint(
            Constraint::new(lhs, 5.0, Comparator::Le, None).unwrap(),
            Some("c0".into()),
        )
        .unwrap();
        m
    }

    #[test]
    fn substitute_many_matches_one_substitute_per_pair() {
        let mut one = model();
        let mut many = one.deep_clone();
        let pairs = |m: &Model| {
            let [a, b, c, d] = ["a", "b", "c", "d"].map(|n| m.var(n).unwrap());
            vec![(a, (&c * 2.0).unwrap()), (b, (&d * 1.0).unwrap())]
        };
        for (target, replacement) in pairs(&one) {
            one.substitute(&target, &replacement).unwrap();
        }
        many.substitute_many(&pairs(&many)).unwrap();

        assert!(one.equal_contents(&many));
        assert_eq!(many.num_variables(), 2);
    }

    #[test]
    fn substitute_many_rejects_chains_and_duplicates_unchanged() {
        let mut m = model();
        let before = m.deep_clone();
        let [a, b] = ["a", "b"].map(|n| m.var(n).unwrap());
        let zero = Expression::constant(m.environment.clone(), 0.0);

        let chain = [(a.clone(), (&b * 1.0).unwrap()), (b.clone(), zero.clone())];
        assert!(m.substitute_many(&chain).is_err());
        let twice = [(a.clone(), zero.clone()), (a, zero)];
        assert!(m.substitute_many(&twice).is_err());

        assert!(m.equal_contents(&before));
    }

    #[test]
    fn substitution_moves_the_constant_to_the_rhs() {
        let mut one = Model::default();
        let [z, x] = ["z", "x"].map(|n| one.add_var(n, Vtype::Binary, None).unwrap());
        let lhs = (&(&z * 1.0).unwrap() + &x).unwrap();
        let c = Constraint::new(lhs, 1.0, Comparator::Le, None).unwrap();
        one.add_constraint(c, Some("c".into())).unwrap();
        let mut many = one.deep_clone();

        one.substitute(&z, &Expression::constant(one.environment.clone(), 1.0))
            .unwrap();
        let z = many.var("z").unwrap();
        let one_const = Expression::constant(many.environment.clone(), 1.0);
        many.substitute_many(&[(z, one_const)]).unwrap();

        for m in [one, many] {
            let c = m.constraints().get("c").unwrap();
            assert_eq!((c.lhs.offset, c.rhs), (0.0, 0.0));
        }
    }
}

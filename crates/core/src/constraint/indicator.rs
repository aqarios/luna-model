use lunamodel_error::{LunaModelError, LunaModelResult};
use lunamodel_types::{VarIdx, Vtype};

use super::Constraint;
use crate::variable::VarRef;

#[derive(Debug, Clone)]
pub struct Indicator {
    /// The binary variable the constraint depends on, never an inverted one.
    pub variable: VarRef,
    /// The value of `variable` the constraint holds for.
    pub value: bool,
    /// What holds then.
    pub constraint: Constraint,
}

impl Constraint {
    /// Reads `z * (g(x) - b) ⋈ 0` as the indicator `z = 1 => g(x) ⋈ b`.
    ///
    /// If `z` is an inverted binary `!y`, the result is stated on its base: `y = 0 => ...`.
    /// When several binaries are in every term, the one with the smallest index becomes `z`.
    ///
    /// Returns `None` if the right-hand side is not zero, no binary variable is in every term,
    /// or `g(x) - b` is constant once `z` is factored out.
    pub fn indicator(&self) -> LunaModelResult<Option<Indicator>> {
        if self.rhs != 0.0 {
            return Ok(None);
        }

        let env = &self.lhs.env;
        let mut terms = self.lhs.items();

        let Some((first_vars, _)) = terms.next() else {
            return Ok(None);
        };
        let mut candidates: Vec<VarIdx> = Vec::with_capacity(first_vars.len());
        {
            // One read lock for all lookups; released before `derivative` below.
            let guard = env.read_arc();
            for v in &first_vars {
                let id = v.id();
                if matches!(guard.get(id)?.vtype, Vtype::Binary | Vtype::InvertedBinary) {
                    candidates.push(id);
                }
            }
        }

        for (vars, _) in terms {
            if candidates.is_empty() {
                return Ok(None);
            }
            candidates.retain(|c| vars.iter().any(|v| v.id() == *c));
        }

        let Some(id) = candidates.into_iter().min() else {
            return Ok(None);
        };
        let z = VarRef::new(id, env.clone());
        let lhs = self.lhs.derivative(&z)?;
        if lhs.is_constant() {
            return Ok(None);
        }

        let (variable, value) = {
            let guard = env.read_arc();
            let var = guard.get(id)?;
            match var.vtype {
                Vtype::InvertedBinary => {
                    let base = var
                        .inverted
                        .ok_or_else(|| LunaModelError::InvalidInversion(var.name.clone().into()))?;
                    (VarRef::new(base, env.clone()), false)
                }
                _ => (z, true),
            }
        };

        let mut constraint = Constraint::new(lhs, 0.0, self.comparator, Some(self.name().into()))?;
        constraint.auto_name = self.auto_name;
        Ok(Some(Indicator {
            variable,
            value,
            constraint,
        }))
    }

    /// States this constraint only for the binary `z` having `value`: `z = 1` for `true`,
    /// `z = 0` for `false`. The result is the constraint `z * (lhs - rhs) ⋈ 0`, with the
    /// inverted binary `!z` in place of `z` for `z = 0` (creating `!z` in the environment if
    /// needed). It keeps the name. [`indicator`](Self::indicator) reads it back.
    ///
    /// `z` must be a binary variable. An inverted binary `!y` stands for the other value of
    /// its base, so `when(!y, true)` is `when(y, false)`. If the constraint itself contains
    /// `z`, the product simplifies (`z * z = z`), and what is read back is this constraint
    /// with `z` fixed to `value`.
    pub fn when(&self, z: &VarRef, value: bool) -> LunaModelResult<Constraint> {
        {
            let guard = z.env.read_arc();
            let var = guard.get(z.id())?;
            if !matches!(var.vtype, Vtype::Binary | Vtype::InvertedBinary) {
                return Err(LunaModelError::UnsupportedOperation(
                    format!(
                        "indicator on `{}`, which is not a binary variable",
                        var.name
                    )
                    .into(),
                ));
            }
        }

        // The factor that is 1 exactly when `z` has `value`; `!` of an inverted binary is
        // its base.
        let factor = if value { z.clone() } else { (!z)? };
        let lhs = ((&self.lhs - self.rhs)? * &factor)?;

        let mut constraint = Constraint::new(lhs, 0.0, self.comparator, Some(self.name().into()))?;
        constraint.auto_name = self.auto_name;
        Ok(constraint)
    }
}

#[cfg(test)]
mod tests {
    use lunamodel_types::{Comparator, Vtype};

    use crate::{Constraint, prelude::ArcEnv, traits::ContentEquality};

    #[test]
    fn binary_factor_becomes_indicator() {
        let mut env = ArcEnv::default();
        let x = env.insert("x", Vtype::Real, None).unwrap();
        let z = env.insert("z", Vtype::Binary, None).unwrap();

        // z * x - 3 z <= 0  ->  z = 1 => x <= 3
        let lhs = ((&z * &x).unwrap() - (&z * 3.0).unwrap()).unwrap();
        let c = Constraint::new(lhs, 0.0, Comparator::Le, None).unwrap();

        let ind = c.indicator().unwrap().expect("indicator");
        assert_eq!(ind.variable.id(), z.id());
        assert!(ind.value);
        assert_eq!(ind.constraint.comparator, Comparator::Le);
        assert_eq!(ind.constraint.rhs, 3.0);
        assert_eq!(ind.constraint.lhs.items().count(), 1);
        assert_eq!(ind.constraint.lhs.linear(x.id()), 1.0);
    }

    #[test]
    fn inverted_binary_is_stated_on_base() {
        let mut env = ArcEnv::default();
        let x = env.insert("x", Vtype::Real, None).unwrap();
        let y = env.insert("y", Vtype::Binary, None).unwrap();
        let ny = (!&y).unwrap();

        // !y * x - 2 !y >= 0  ->  y = 0 => x >= 2
        let lhs = ((&ny * &x).unwrap() - (&ny * 2.0).unwrap()).unwrap();
        let c = Constraint::new(lhs, 0.0, Comparator::Ge, None).unwrap();

        let ind = c.indicator().unwrap().expect("indicator");
        assert_eq!(ind.variable.id(), y.id());
        assert!(!ind.value);
        assert_eq!(ind.constraint.comparator, Comparator::Ge);
        assert_eq!(ind.constraint.rhs, 2.0);
        assert_eq!(ind.constraint.lhs.linear(x.id()), 1.0);
    }

    #[test]
    fn non_indicator_shapes_return_none() {
        let mut env = ArcEnv::default();
        let x = env.insert("x", Vtype::Real, None).unwrap();
        let z = env.insert("z", Vtype::Binary, None).unwrap();

        // Non-zero right-hand side.
        let lhs = (&z * &x).unwrap();
        let c = Constraint::new(lhs, 1.0, Comparator::Le, None).unwrap();
        assert!(c.indicator().unwrap().is_none());

        // `z` is missing from the `x` term.
        let lhs = ((&z * &x).unwrap() + (&x * 1.0).unwrap()).unwrap();
        let c = Constraint::new(lhs, 0.0, Comparator::Le, None).unwrap();
        assert!(c.indicator().unwrap().is_none());

        // Constant once `z` is factored out: 3 z <= 0.
        let lhs = (&z * 3.0).unwrap();
        let c = Constraint::new(lhs, 0.0, Comparator::Le, None).unwrap();
        assert!(c.indicator().unwrap().is_none());
    }

    #[test]
    fn smallest_binary_index_wins() {
        let mut env = ArcEnv::default();
        let x = env.insert("x", Vtype::Real, None).unwrap();
        let z = env.insert("z", Vtype::Binary, None).unwrap();
        let w = env.insert("w", Vtype::Binary, None).unwrap();
        assert!(z.id() < w.id());

        // w * z * x <= 0  ->  z = 1 => w * x <= 0
        let lhs = ((&w * &z).unwrap() * &x).unwrap();
        let c = Constraint::new(lhs, 0.0, Comparator::Le, None).unwrap();

        let ind = c.indicator().unwrap().expect("indicator");
        assert_eq!(ind.variable.id(), z.id());
        assert!(ind.value);
        assert_eq!(ind.constraint.comparator, Comparator::Le);
        assert_eq!(ind.constraint.rhs, 0.0);
        assert_eq!(ind.constraint.lhs.items().count(), 1);
        assert_eq!(ind.constraint.lhs.quadratic(x.id(), w.id()), 1.0);
    }

    #[test]
    fn higher_order_inner_constraint() {
        let mut env = ArcEnv::default();
        let x = env.insert("x", Vtype::Real, None).unwrap();
        let y = env.insert("y", Vtype::Real, None).unwrap();
        let z = env.insert("z", Vtype::Binary, None).unwrap();

        // z * x^2 * y <= 0  ->  z = 1 => x^2 * y <= 0 (three factors left, so it stays higher-order)
        let lhs = (((&z * &x).unwrap() * &x).unwrap() * &y).unwrap();
        let c = Constraint::new(lhs, 0.0, Comparator::Le, None).unwrap();

        let ind = c.indicator().unwrap().expect("indicator");
        assert_eq!(ind.variable.id(), z.id());
        assert!(ind.value);
        assert_eq!(ind.constraint.rhs, 0.0);
        assert_eq!(ind.constraint.lhs.degree(), 3);
        assert_eq!(ind.constraint.lhs.items().count(), 1);
        assert_eq!(
            ind.constraint.lhs.higher_order(&[x.id(), x.id(), y.id()]),
            1.0
        );
    }

    #[test]
    fn no_binary_returns_none() {
        let mut env = ArcEnv::default();
        let x = env.insert("x", Vtype::Integer, None).unwrap();
        let y = env.insert("y", Vtype::Integer, None).unwrap();

        // x * y <= 0 has no binary to condition on.
        let lhs = (&x * &y).unwrap();
        let c = Constraint::new(lhs, 0.0, Comparator::Le, None).unwrap();
        assert!(c.indicator().unwrap().is_none());
    }

    #[test]
    fn inner_constraint_keeps_outer_name() {
        let mut env = ArcEnv::default();
        let x = env.insert("x", Vtype::Real, None).unwrap();
        let z = env.insert("z", Vtype::Binary, None).unwrap();

        // Explicit name.
        let lhs = (&z * &x).unwrap();
        let c = Constraint::new(lhs, 0.0, Comparator::Le, Some("cap".into())).unwrap();
        let ind = c.indicator().unwrap().expect("indicator");
        assert_eq!(ind.constraint.name(), "cap");
        assert!(!ind.constraint.auto_name);

        // Auto name.
        let lhs = (&z * &x).unwrap();
        let c = Constraint::new(lhs, 0.0, Comparator::Le, None).unwrap();
        let ind = c.indicator().unwrap().expect("indicator");
        assert_eq!(ind.constraint.name(), c.name());
        assert!(ind.constraint.auto_name);
    }

    #[test]
    fn indicator_does_not_advance_auto_names() {
        let mut env = ArcEnv::default();
        let x = env.insert("x", Vtype::Real, None).unwrap();
        let z = env.insert("z", Vtype::Binary, None).unwrap();

        let lhs = (&z * &x).unwrap();
        let c = Constraint::new(lhs, 0.0, Comparator::Le, None).unwrap();

        let auto_named =
            || Constraint::new((&x * 1.0).unwrap(), 0.0, Comparator::Le, None).unwrap();
        let number = |c: &Constraint| c.name()[1..].parse::<u64>().unwrap();

        // Auto names come from a global counter that other tests advance concurrently, so retry
        // until one window sees no interference. Minting names in `indicator()` fails every try.
        let unchanged = (0..100).any(|_| {
            let before = auto_named();
            c.indicator().unwrap().expect("indicator");
            c.indicator().unwrap().expect("indicator");
            let after = auto_named();
            number(&after) == number(&before) + 1
        });
        assert!(unchanged, "indicator() advanced the auto-name counter");
    }

    #[test]
    fn when_active_round_trips() {
        let mut env = ArcEnv::default();
        let x = env.insert("x", Vtype::Real, None).unwrap();
        let z = env.insert("z", Vtype::Binary, None).unwrap();

        // x <= 3  ->  z * (x - 3) <= 0  ->  z = 1 => x <= 3
        let c = Constraint::new((&x * 1.0).unwrap(), 3.0, Comparator::Le, None).unwrap();
        let w = c.when(&z, true).unwrap();

        let ind = w.indicator().unwrap().expect("indicator");
        assert_eq!(ind.variable.id(), z.id());
        assert!(ind.value);
        assert!(ind.constraint.equal_contents(&c));
    }

    #[test]
    fn when_inactive_round_trips_on_base() {
        let mut env = ArcEnv::default();
        let x = env.insert("x", Vtype::Real, None).unwrap();
        let z = env.insert("z", Vtype::Binary, None).unwrap();

        // x >= 2  ->  !z * (x - 2) >= 0  ->  z = 0 => x >= 2
        let c = Constraint::new((&x * 1.0).unwrap(), 2.0, Comparator::Ge, None).unwrap();
        let w = c.when(&z, false).unwrap();

        let ind = w.indicator().unwrap().expect("indicator");
        let nz = (!&z).unwrap();
        assert_ne!(nz.id(), z.id());
        assert_eq!(ind.variable.id(), z.id());
        assert_ne!(ind.variable.id(), nz.id());
        assert!(!ind.value);
        assert!(ind.constraint.equal_contents(&c));
    }

    #[test]
    fn when_keeps_name() {
        let mut env = ArcEnv::default();
        let x = env.insert("x", Vtype::Real, None).unwrap();
        let z = env.insert("z", Vtype::Binary, None).unwrap();

        for value in [true, false] {
            // Explicit name.
            let c = Constraint::new((&x * 1.0).unwrap(), 1.0, Comparator::Le, Some("cap".into()))
                .unwrap();
            let w = c.when(&z, value).unwrap();
            assert_eq!(w.name(), "cap");
            assert!(!w.auto_name);
            let ind = w.indicator().unwrap().expect("indicator");
            assert_eq!(ind.constraint.name(), "cap");
            assert!(!ind.constraint.auto_name);

            // Auto name.
            let c = Constraint::new((&x * 1.0).unwrap(), 1.0, Comparator::Le, None).unwrap();
            let w = c.when(&z, value).unwrap();
            assert_eq!(w.name(), c.name());
            assert!(w.auto_name);
            let ind = w.indicator().unwrap().expect("indicator");
            assert_eq!(ind.constraint.name(), c.name());
            assert!(ind.constraint.auto_name);
        }
    }

    #[test]
    fn when_on_non_binary_is_error() {
        let mut env = ArcEnv::default();
        let x = env.insert("x", Vtype::Real, None).unwrap();
        let n = env.insert("n", Vtype::Integer, None).unwrap();
        let s = env.insert("s", Vtype::Spin, None).unwrap();

        let c = Constraint::new((&x * 1.0).unwrap(), 1.0, Comparator::Le, None).unwrap();
        for value in [true, false] {
            assert!(c.when(&n, value).is_err());
            assert!(c.when(&s, value).is_err());
        }
    }

    #[test]
    fn when_on_inverted_binary_is_the_other_value_of_its_base() {
        let mut env = ArcEnv::default();
        let x = env.insert("x", Vtype::Real, None).unwrap();
        let y = env.insert("y", Vtype::Binary, None).unwrap();
        let ny = (!&y).unwrap();

        let c = Constraint::new((&x * 1.0).unwrap(), 1.0, Comparator::Le, None).unwrap();
        for value in [true, false] {
            let ind = c.when(&ny, value).unwrap().indicator().unwrap();
            let ind = ind.expect("indicator");
            assert_eq!(ind.variable.id(), y.id());
            assert_eq!(ind.value, !value);
            assert!(ind.constraint.equal_contents(&c));
        }
    }

    #[test]
    fn when_on_contained_binary_collapses() {
        let mut env = ArcEnv::default();
        let x = env.insert("x", Vtype::Real, None).unwrap();
        let z = env.insert("z", Vtype::Binary, None).unwrap();

        // x + z <= 1  ->  z * (x + z - 1) = z * x <= 0  ->  z = 1 => x <= 0
        let lhs = ((&x * 1.0).unwrap() + (&z * 1.0).unwrap()).unwrap();
        let c = Constraint::new(lhs, 1.0, Comparator::Le, None).unwrap();
        let w = c.when(&z, true).unwrap();

        let ind = w.indicator().unwrap().expect("indicator");
        assert_eq!(ind.variable.id(), z.id());
        assert!(ind.value);
        assert_eq!(ind.constraint.comparator, Comparator::Le);
        assert_eq!(ind.constraint.rhs, 0.0);
        assert_eq!(ind.constraint.lhs.items().count(), 1);
        assert_eq!(ind.constraint.lhs.linear(x.id()), 1.0);
    }
}

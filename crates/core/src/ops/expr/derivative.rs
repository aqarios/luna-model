use lunamodel_error::LunaModelResult;

use crate::{
    Expression,
    expression::term::{HigherOrder, Linear, Quadratic},
    ops::utils::check_envs,
    variable::VarRef,
};

impl Expression {
    /// The partial derivative with respect to `var`: every term that contains it, without
    /// it. Fails if `var` is from another environment or no longer in it.
    pub fn derivative(&self, var: &VarRef) -> LunaModelResult<Self> {
        check_envs(self, var)?;
        let mut offset: f64 = 0.0;
        for (k, v) in self.linear_items() {
            if k.id() == var.id() {
                offset += v;
            }
        }

        let mut linear: Linear = Linear::default();
        for (k1, k2, v) in self.quadratic_items() {
            if k1.id() == var.id() || k2.id() == var.id() {
                let k = if k1.id() == var.id() { k2 } else { k1 };
                linear += (k.id(), v);
            }
        }

        let (quadratic, higher_order) = {
            let mut quadratic: Quadratic = Quadratic::default();
            let mut higher_order: HigherOrder = HigherOrder::default();
            for (ks, v) in self.higher_order_items() {
                if let Some((i, _)) = ks.iter().enumerate().find(|(_, x)| x.id() == var.id()) {
                    let idxs: Vec<u32> = ks
                        .iter()
                        .enumerate()
                        .filter(|(j, _)| *j != i)
                        .map(|(_, x)| x.id())
                        .collect();
                    if idxs.len() == 2 {
                        quadratic += (idxs[0], idxs[1], v);
                    } else {
                        higher_order += (idxs, v);
                    }
                }
            }

            (
                if !quadratic.is_empty() {
                    Some(quadratic)
                } else {
                    None
                },
                if !higher_order.is_empty() {
                    Some(higher_order)
                } else {
                    None
                },
            )
        };

        Ok(Expression {
            env: self.env.clone(),
            offset,
            linear,
            quadratic,
            higher_order,
        })
    }
}

#[cfg(test)]
mod tests {
    use lunamodel_error::LunaModelError;
    use lunamodel_types::Vtype;

    use crate::prelude::ArcEnv;

    #[test]
    fn removes_the_variable_from_its_terms() {
        let mut env = ArcEnv::default();
        let x = env.insert("x", Vtype::Integer, None).unwrap();
        let y = env.insert("y", Vtype::Integer, None).unwrap();

        // d/dx (x y + 3 x + y) = y + 3
        let expr = (((&x * &y).unwrap() + (&x * 3.0).unwrap()).unwrap() + &y).unwrap();
        let derivative = expr.derivative(&x).unwrap();
        assert_eq!(derivative.linear(y.id()), 1.0);
        assert_eq!(derivative.offset, 3.0);
        assert_eq!(derivative.items().count(), 2);
    }

    #[test]
    fn rejects_a_variable_of_another_environment() {
        let mut env = ArcEnv::default();
        let x = env.insert("x", Vtype::Integer, None).unwrap();
        let mut other = ArcEnv::default();
        let stranger = other.insert("x", Vtype::Integer, None).unwrap();
        assert_eq!(x.id(), stranger.id());

        let expr = (&x * 2.0).unwrap();
        let err = expr.derivative(&stranger).unwrap_err();
        assert!(matches!(err, LunaModelError::DifferentEnvironments));
    }
}

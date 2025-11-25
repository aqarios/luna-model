use lunamodel_error::LunaModelResult;
use lunamodel_types::Bias;

use crate::ops::utils::check_envs;
use crate::{Expression, ops::LmMulAssign, prelude::VarRef};
use crate::{muls, rmuls};

impl LmMulAssign<&Bias> for Expression {
    fn mul_assign(&mut self, rhs: &Bias) -> LunaModelResult<()> {
        self.offset *= rhs;
        self.linear *= *rhs;
        if let Some(q) = self.quadratic.as_mut() {
            *q *= *rhs;
        }
        if let Some(h) = self.higher_order.as_mut() {
            *h *= *rhs;
        }
        Ok(())
    }
}

impl LmMulAssign<&usize> for Expression {
    fn mul_assign(&mut self, rhs: &usize) -> LunaModelResult<()> {
        self.mul_assign(*rhs as Bias)
    }
}

impl LmMulAssign<&VarRef> for Expression {
    fn mul_assign(&mut self, rhs: &VarRef) -> LunaModelResult<()> {
        check_envs(self, rhs)?;

        let nl = rhs * self.offset;
        self.offset = Bias::default();
        _ = rhs;
        unimplemented!("implement expr *= &vref")
    }
}

impl LmMulAssign<&Expression> for Expression {
    fn mul_assign(&mut self, rhs: &Expression) -> LunaModelResult<()> {
        _ = rhs;
        unimplemented!("implement expr *= &expr")
    }
}

muls!(Expression => Bias, usize, VarRef, Expression);
rmuls!(Expression => Bias, usize, VarRef);

#[cfg(test)]
mod tests {
    use crate::prelude::{ArcEnv, Expression, LmMulAssign, VarRef};
    use lunamodel_types::{Bias, Vtype};

    #[test]
    fn mul_bias_to_expr() {
        let env = ArcEnv::default();

        let b: Bias = 12.34;
        let base = (Expression::empty(env.clone()) + 2).unwrap();
        let base_res = (base.clone() * b).unwrap();

        {
            let e = base.clone();
            let res = e * &b;
            assert_eq!(base_res, res.unwrap());
        }
        {
            let e = base.clone();
            let res = (&e) * b;
            assert_eq!(base_res, res.unwrap());
        }
        {
            let e = base.clone();
            let res = (&e) * (&b);
            assert_eq!(base_res, res.unwrap());
        }

        {
            let e = Expression::empty(env.clone());
            let res = b * e;
            assert_eq!(base_res, res.unwrap());
        }
        {
            let e = Expression::empty(env.clone());
            let res = b * (&e);
            assert_eq!(base_res, res.unwrap());
        }
        {
            let e = Expression::empty(env.clone());
            let res = (&b) * e;
            assert_eq!(base_res, res.unwrap());
        }
        {
            let e = Expression::empty(env.clone());
            let res = (&b) * (&e);
            assert_eq!(base_res, res.unwrap());
        }
    }

    #[test]
    fn mul_assign_bias_to_expr() {
        let b = 12.34;
        let mut e = Expression::empty(ArcEnv::default());
        e.mul_assign(&b).unwrap();
        e.mul_assign(b).unwrap();
    }

    #[test]
    fn mul_vref_to_expr() {
        let mut env = ArcEnv::default();
        let v: VarRef = env.insert("b".into(), Vtype::Binary, None).unwrap();
        let base = (Expression::empty(env.clone()) + 2).unwrap();
        let base_res = (base.clone() * v.clone()).unwrap();

        {
            let (v, e) = (v.clone(), base.clone());
            let res = e * &v;
            assert_eq!(base_res, res.unwrap());
        }
        {
            let (v, e) = (v.clone(), base.clone());
            let res = (&e) * v;
            assert_eq!(base_res, res.unwrap());
        }
        {
            let (v, e) = (v.clone(), base.clone());
            let res = (&e) * (&v);
            assert_eq!(base_res, res.unwrap());
        }
        {
            let (v, e) = (v.clone(), base.clone());
            let res = v * e;
            assert_eq!(base_res, res.unwrap());
        }
        {
            let (v, e) = (v.clone(), base.clone());
            let res = v * (&e);
            assert_eq!(base_res, res.unwrap());
        }
        {
            let (v, e) = (v.clone(), base.clone());
            let res = (&v) * e;
            assert_eq!(base_res, res.unwrap());
        }
        {
            let (v, e) = (v.clone(), base.clone());
            let res = (&v) * (&e);
            assert_eq!(base_res, res.unwrap());
        }
    }

    #[test]
    fn mul_assign_vref_to_expr() {
        let mut env = ArcEnv::default();
        let v: VarRef = env.insert("b".into(), Vtype::Binary, None).unwrap();
        let mut e = Expression::empty(env);
        e.mul_assign(&v).unwrap();
        e.mul_assign(v).unwrap();
    }
}

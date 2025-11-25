use crate::ops::LmAddAssign;
use crate::{expression::Expression, variable::VarRef};
use crate::{rsubs, subs};
use lunamodel_error::LunaModelResult;
use lunamodel_types::Bias;

use crate::ops::{LmSubAssign, utils::check_envs};

// Bias
impl LmSubAssign<&Bias> for Expression {
    fn sub_assign(&mut self, rhs: &Bias) -> LunaModelResult<()> {
        self.add_assign(-rhs)
    }
}

impl LmSubAssign<&usize> for Expression {
    fn sub_assign(&mut self, rhs: &usize) -> LunaModelResult<()> {
        self.sub_assign(*rhs as Bias)
    }
}

impl LmSubAssign<&VarRef> for Expression {
    fn sub_assign(&mut self, rhs: &VarRef) -> LunaModelResult<()> {
        check_envs(self, rhs)?;
        self.linear += (rhs.id(), -1.0);
        Ok(())
    }
}

impl LmSubAssign<&Expression> for Expression {
    fn sub_assign(&mut self, rhs: &Expression) -> LunaModelResult<()> {
        self.add_assign(-rhs)
    }
}

subs!(Expression => Bias, usize, VarRef, Expression);
rsubs!(Expression => Bias, usize, VarRef);

#[cfg(test)]
mod tests {
    use crate::prelude::{ArcEnv, Expression, LmSubAssign, VarRef};
    use lunamodel_types::{Bias, Vtype};

    #[test]
    fn sub_bias_to_expr() {
        let env = ArcEnv::default();

        let b: Bias = 12.34;
        let base = Expression::empty(env.clone());
        let base_res = (base.clone() - b).unwrap();

        {
            let e = base.clone();
            let res = e - &b;
            assert_eq!(base_res, res.unwrap());
        }
        {
            let e = base.clone();
            let res = (&e) - b;
            assert_eq!(base_res, res.unwrap());
        }
        {
            let e = base.clone();
            let res = (&e) - (&b);
            assert_eq!(base_res, res.unwrap());
        }

        {
            let e = base.clone();
            let res = b - e;
            assert_eq!(base_res, res.unwrap());
        }
        {
            let e = base.clone();
            let res = b - (&e);
            assert_eq!(base_res, res.unwrap());
        }
        {
            let e = base.clone();
            let res = (&b) - e;
            assert_eq!(base_res, res.unwrap());
        }
        {
            let e = base.clone();
            let res = (&b) - (&e);
            assert_eq!(base_res, res.unwrap());
        }
    }

    #[test]
    fn sub_assign_bias_to_expr() {
        let b = 12.34;
        let mut e = Expression::empty(ArcEnv::default());
        e.sub_assign(&b).unwrap();
        e.sub_assign(b).unwrap();
    }

    #[test]
    fn sub_vref_to_expr() {
        let mut env = ArcEnv::default();
        let v: VarRef = env.insert("b".into(), Vtype::Binary, None).unwrap();
        let base = Expression::empty(env.clone());
        let base_res = (base.clone() - v.clone()).unwrap();

        {
            let (v, e) = (v.clone(), base.clone());
            let res = e - &v;
            assert_eq!(base_res, res.unwrap());
        }
        {
            let (v, e) = (v.clone(), base.clone());
            let res = (&e) - v;
            assert_eq!(base_res, res.unwrap());
        }
        {
            let (v, e) = (v.clone(), base.clone());
            let res = (&e) - (&v);
            assert_eq!(base_res, res.unwrap());
        }
        {
            let (v, e) = (v.clone(), base.clone());
            let res = v - e;
            assert_eq!(base_res, res.unwrap());
        }
        {
            let (v, e) = (v.clone(), base.clone());
            let res = v - (&e);
            assert_eq!(base_res, res.unwrap());
        }
        {
            let (v, e) = (v.clone(), base.clone());
            let res = (&v) - e;
            assert_eq!(base_res, res.unwrap());
        }
        {
            let (v, e) = (v.clone(), base.clone());
            let res = (&v) - (&e);
            assert_eq!(base_res, res.unwrap());
        }
    }

    #[test]
    fn sub_assign_vref_to_expr() {
        let mut env = ArcEnv::default();
        let v: VarRef = env.insert("b".into(), Vtype::Binary, None).unwrap();
        let mut e = Expression::empty(env);
        e.sub_assign(&v).unwrap();
        e.sub_assign(v).unwrap();
    }
}

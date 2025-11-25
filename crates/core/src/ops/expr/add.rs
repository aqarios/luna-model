use crate::{adds, radds};
use crate::{expression::Expression, variable::VarRef};
use lunamodel_error::LunaModelResult;
use lunamodel_types::Bias;

use crate::ops::{LmAddAssign, utils::check_envs};

// Bias
impl LmAddAssign<&Bias> for Expression {
    fn add_assign(&mut self, rhs: &Bias) -> LunaModelResult<()> {
        self.offset += rhs;
        Ok(())
    }
}

impl LmAddAssign<&usize> for Expression {
    fn add_assign(&mut self, rhs: &usize) -> LunaModelResult<()> {
        self.offset += *rhs as Bias;
        Ok(())
    }
}

impl LmAddAssign<&VarRef> for Expression {
    fn add_assign(&mut self, rhs: &VarRef) -> LunaModelResult<()> {
        check_envs(self, rhs)?;
        self.linear += (rhs.id(), 1.0);
        Ok(())
    }
}

impl LmAddAssign<&Expression> for Expression {
    fn add_assign(&mut self, rhs: &Expression) -> LunaModelResult<()> {
        check_envs(self, rhs)?;
        self.offset += rhs.offset;
        self.linear += &rhs.linear;
        match (self.quadratic.as_mut(), rhs.quadratic.as_ref()) {
            (Some(lq), Some(rq)) => *lq += rq,
            (None, Some(rq)) => self.quadratic = Some(rq.clone()),
            (Some(_), None) | (None, None) => (),
        }
        match (self.higher_order.as_mut(), rhs.higher_order.as_ref()) {
            (Some(lh), Some(rh)) => *lh += rh,
            (None, Some(rh)) => self.higher_order = Some(rh.clone()),
            (Some(_), None) | (None, None) => (),
        }
        Ok(())
    }
}

adds!(Expression => Bias, usize, VarRef, Expression);
radds!(Expression => Bias, usize, VarRef);

#[cfg(test)]
mod tests {
    use crate::prelude::{ArcEnv, Expression, LmAddAssign, VarRef};
    use lunamodel_types::{Bias, Vtype};

    #[test]
    fn add_bias_to_expr() {
        let env = ArcEnv::default();

        let b: Bias = 12.34;
        let base = Expression::empty(env.clone());
        let base_res = (base.clone() + b).unwrap();

        {
            let e = base.clone();
            let res = e + &b;
            assert_eq!(base_res, res.unwrap());
        }
        {
            let e = base.clone();
            let res = (&e) + b;
            assert_eq!(base_res, res.unwrap());
        }
        {
            let e = base.clone();
            let res = (&e) + (&b);
            assert_eq!(base_res, res.unwrap());
        }

        {
            let e = base.clone();
            let res = b + e;
            assert_eq!(base_res, res.unwrap());
        }
        {
            let e = base.clone();
            let res = b + (&e);
            assert_eq!(base_res, res.unwrap());
        }
        {
            let e = base.clone();
            let res = (&b) + e;
            assert_eq!(base_res, res.unwrap());
        }
        {
            let e = base.clone();
            let res = (&b) + (&e);
            assert_eq!(base_res, res.unwrap());
        }
    }

    #[test]
    fn add_assign_bias_to_expr() {
        let b = 12.34;
        let mut e = Expression::empty(ArcEnv::default());
        e.add_assign(&b).unwrap();
        e.add_assign(b).unwrap();
    }

    #[test]
    fn add_vref_to_expr() {
        let mut env = ArcEnv::default();
        let v: VarRef = env.insert("b".into(), Vtype::Binary, None).unwrap();
        let base = Expression::empty(env.clone());
        let base_res = (base.clone() + v.clone()).unwrap();

        {
            let (v, e) = (v.clone(), base.clone());
            let res = e + &v;
            assert_eq!(base_res, res.unwrap());
        }
        {
            let (v, e) = (v.clone(), base.clone());
            let res = (&e) + v;
            assert_eq!(base_res, res.unwrap());
        }
        {
            let (v, e) = (v.clone(), base.clone());
            let res = (&e) + (&v);
            assert_eq!(base_res, res.unwrap());
        }
        {
            let (v, e) = (v.clone(), base.clone());
            let res = v + e;
            assert_eq!(base_res, res.unwrap());
        }
        {
            let (v, e) = (v.clone(), base.clone());
            let res = v + (&e);
            assert_eq!(base_res, res.unwrap());
        }
        {
            let (v, e) = (v.clone(), base.clone());
            let res = (&v) + e;
            assert_eq!(base_res, res.unwrap());
        }
        {
            let (v, e) = (v.clone(), base.clone());
            let res = (&v) + (&e);
            assert_eq!(base_res, res.unwrap());
        }
    }

    #[test]
    fn add_assign_vref_to_expr() {
        let mut env = ArcEnv::default();
        let v: VarRef = env.insert("b".into(), Vtype::Binary, None).unwrap();
        let mut e = Expression::empty(env);
        e.add_assign(&v).unwrap();
        e.add_assign(v).unwrap();
    }
}

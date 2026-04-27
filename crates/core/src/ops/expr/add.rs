use lunamodel_error::LunaModelResult;
use lunamodel_types::Bias;

use crate::ops::traits::internal::PrvAddAssign;
use crate::ops::utils::{VarMulRes, VarMulRes::*};
use crate::ops::{LmAddAssign, utils::check_envs};
use crate::prelude::{HigherOrder, Linear, Quadratic};
use crate::{adds, radds};
use crate::{expression::Expression, variable::VarRef};

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
        rhs.check_living()?;
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

impl PrvAddAssign<Bias> for Expression {
    fn aa(&mut self, rhs: Bias) {
        self.offset += rhs;
    }
}

impl PrvAddAssign<Linear> for Expression {
    fn aa(&mut self, rhs: Linear) {
        self.linear += rhs;
    }
}

impl PrvAddAssign<Vec<VarMulRes>> for Expression {
    fn aa(&mut self, rhs: Vec<VarMulRes>) {
        for item in rhs {
            match item {
                Const(c) => self.offset += c,
                Lin(l) => self.linear += l,
                Quad(q) => {
                    if let Some(expr_q) = self.quadratic.as_mut() {
                        *expr_q += q
                    } else {
                        let mut new_q = Quadratic::default();
                        new_q += q;
                        match new_q.is_zero() {
                            true => (),
                            false => self.quadratic = Some(new_q),
                        }
                    }
                }
                HiOr(h) => {
                    if let Some(expr_h) = self.higher_order.as_mut() {
                        *expr_h += h
                    } else {
                        let mut new_h = HigherOrder::default();
                        new_h += h;
                        match new_h.is_zero() {
                            true => (),
                            false => self.higher_order = Some(new_h),
                        }
                    }
                }
            }
        }
    }
}

impl PrvAddAssign<Option<Vec<VarMulRes>>> for Expression {
    fn aa(&mut self, rhs: Option<Vec<VarMulRes>>) {
        if let Some(vs) = rhs
            && !vs.is_empty()
        {
            self.aa(vs)
        }
    }
}

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
            let res = e + b;
            assert_eq!(base_res, res.unwrap());
        }
        {
            let e = base.clone();
            let res = (&e) + b;
            assert_eq!(base_res, res.unwrap());
        }
        {
            let e = base.clone();
            let res = (&e) + b;
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
            let res = b + e;
            assert_eq!(base_res, res.unwrap());
        }
        {
            let e = base.clone();
            let res = b + (&e);
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
        let v: VarRef = env.insert("b", Vtype::Binary, None).unwrap();
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
        let v: VarRef = env.insert("b", Vtype::Binary, None).unwrap();
        let mut e = Expression::empty(env);
        e.add_assign(&v).unwrap();
        e.add_assign(v).unwrap();
    }
}

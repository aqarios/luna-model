use lunamodel_error::{LunaModelError, LunaModelResult};
use lunamodel_types::Bias;

use crate::prelude::{HigherOrder, Quadratic};
use crate::traits::Editable;
use crate::{Expression, ops::LmMulAssign, prelude::VarRef};
use crate::{muls, rmuls};
use crate::{
    ops::{
        traits::internal::{PrvAddAssign, PrvMul},
        utils::{VarMulRes, check_envs},
    },
    prelude::Linear,
};

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
        rhs.check_living()?;
        *self = Expression::empty(self.env.clone()).edit(|expr| {
            expr.aa(rhs.m(self.offset));
            expr.aa(self.linear.m(rhs));
            expr.aa(self.quadratic.m(rhs));
            expr.aa(self.higher_order.m(rhs));
        });
        Ok(())
    }
}

impl LmMulAssign<&Expression> for Expression {
    fn mul_assign(&mut self, rhs: &Expression) -> LunaModelResult<()> {
        check_envs(self, rhs)?;
        *self = Expression::empty(self.env.clone()).maybe_edit(|expr| {
            expr.aa(self.m(rhs.offset));
            expr.aa(self.m(&rhs.linear));
            expr.aa(self.m(&rhs.quadratic));
            expr.aa(self.m(&rhs.higher_order));
            Ok::<(), LunaModelError>(())
        })?;
        Ok(())
    }
}

muls!(Expression => Bias, usize, VarRef, Expression);
rmuls!(Expression => Bias, usize, VarRef);

impl PrvMul<Bias> for &Expression {
    type Output = Vec<VarMulRes>;

    fn m(self, rhs: Bias) -> Self::Output {
        let mut r = Vec::new();
        r.push(VarMulRes::Const(self.offset * rhs));
        r.append(&mut self.linear.m(rhs));
        r.append(&mut self.quadratic.m(rhs));
        r.append(&mut self.higher_order.m(rhs));
        r
    }
}

impl PrvMul<&Linear> for &Expression {
    type Output = Vec<VarMulRes>;

    fn m(self, rhs: &Linear) -> Self::Output {
        let mut r = Vec::new();
        r.append(&mut rhs.m(self.offset));
        r.append(&mut self.linear.m((rhs, &self.env)));
        r.append(&mut self.quadratic.m((rhs, &self.env)));
        r.append(&mut self.higher_order.m((rhs, &self.env)));
        r
    }
}

impl PrvMul<&Option<Quadratic>> for &Expression {
    type Output = Vec<VarMulRes>;

    fn m(self, rhs: &Option<Quadratic>) -> Self::Output {
        let mut r = Vec::new();
        r.append(&mut rhs.m(self.offset));
        r.append(&mut self.linear.m((rhs, &self.env)));
        r.append(&mut self.quadratic.m((rhs, &self.env)));
        r.append(&mut self.higher_order.m((rhs, &self.env)));
        r
    }
}

impl PrvMul<&Option<HigherOrder>> for &Expression {
    type Output = Vec<VarMulRes>;

    fn m(self, rhs: &Option<HigherOrder>) -> Self::Output {
        let mut r = Vec::new();
        r.append(&mut rhs.m(self.offset));
        r.append(&mut self.linear.m((rhs, &self.env)));
        r.append(&mut self.quadratic.m((rhs, &self.env)));
        r.append(&mut self.higher_order.m((rhs, &self.env)));
        r
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        prelude::{ArcEnv, Expression, HigherOrder, Linear, LmMulAssign, Quadratic, VarRef},
        traits::DefaultEditable,
    };
    use lunamodel_types::{Bias, Vtype};

    #[test]
    fn mul_bias_to_expr() {
        let env = ArcEnv::default();

        let b: Bias = 12.34;
        let base = (Expression::empty(env.clone()) + 2).unwrap();
        let base_res = (base.clone() * b).unwrap();

        {
            let e = base.clone();
            let res = e * b;
            assert_eq!(base_res, res.unwrap());
        }
        {
            let e = base.clone();
            let res = (&e) * b;
            assert_eq!(base_res, res.unwrap());
        }
        {
            let e = base.clone();
            let res = (&e) * b;
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
            let res = b * e;
            assert_eq!(base_res, res.unwrap());
        }
        {
            let e = Expression::empty(env.clone());
            let res = b * (&e);
            assert_eq!(base_res, res.unwrap());
        }
    }

    #[test]
    fn mul_assign_bias_to_expr() {
        let b = 12.34;
        let e = Expression::empty(ArcEnv::default());
        e.clone().mul_assign(&b).unwrap();
        e.clone().mul_assign(b).unwrap();
    }

    #[test]
    fn mul_vref_to_expr() {
        let mut env = ArcEnv::default();
        let v: VarRef = env.insert("b", Vtype::Binary, None).unwrap();
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
        let v: VarRef = env.insert("b", Vtype::Binary, None).unwrap();
        let e = Expression::empty(env);
        dbg!(&v, &e);
        e.clone().mul_assign(&v).unwrap();
        e.clone().mul_assign(v).unwrap();
    }

    #[test]
    fn mul_expr_to_expr() {
        let mut env = ArcEnv::default();
        let u: VarRef = env.insert("u", Vtype::Binary, None).unwrap();
        let v: VarRef = env.insert("v", Vtype::Binary, None).unwrap();
        let z: VarRef = env.insert("z", Vtype::Binary, None).unwrap();

        let o = 5.0;
        let lb = 2.0;
        let l = Linear::with(|l| *l += (u.id, lb));
        let qb = 3.0;
        let q = Quadratic::with(|q| *q += (u.id, v.id, qb));
        let hb = 4.0;
        let h = HigherOrder::with(|h| *h += (vec![u.id, v.id, z.id], hb));

        let cns = Expression::with(|e| {
            e.env = env.clone();
            e.offset += o;
        });
        let lin = Expression::with(|e| {
            e.env = env.clone();
            e.linear = l.clone();
        });
        let qud = Expression::with(|e| {
            e.env = env.clone();
            e.quadratic = Some(q.clone());
        });
        let hio = Expression::with(|e| {
            e.env = env.clone();
            e.higher_order = Some(h.clone());
        });

        // cns * cns
        let cns_cns_exp = Expression::with(|e| {
            e.env = env.clone();
            e.offset = o * o;
        });
        // lin * lin
        let lin_lin_exp = Expression::with(|e| {
            e.env = env.clone();
            e.linear = l.clone() * lb;
        });
        // qud * qud
        let qud_qud_exp = Expression::with(|e| {
            e.env = env.clone();
            e.quadratic = Some(q.clone() * qb);
        });
        // hio * hio
        let hio_hio_exp = Expression::with(|e| {
            e.env = env.clone();
            e.higher_order = Some(h.clone() * hb);
        });
        // cns * lin & lin * cns
        let cns_lin_exp = Expression::with(|e| {
            e.env = env.clone();
            e.linear = l.clone() * o;
        });
        // cns * qud & qud * cns
        let cns_qud_exp = Expression::with(|e| {
            e.env = env.clone();
            e.quadratic = Some(q.clone() * o);
        });
        // cns * hio & hio * cns
        let cns_hio_exp = Expression::with(|e| {
            e.env = env.clone();
            e.higher_order = Some(h.clone() * o);
        });
        // lin * qud & qud * lin
        let lin_qud_exp = Expression::with(|e| {
            e.env = env.clone();
            e.quadratic = Some(q.clone() * lb);
        });
        // lin * hio & hio * lin
        let lin_hio_exp = Expression::with(|e| {
            e.env = env.clone();
            e.higher_order = Some(h.clone() * lb);
        });
        // qud * hio & hio * qud
        let qud_hio_exp = Expression::with(|e| {
            e.env = env.clone();
            e.higher_order = Some(h.clone() * qb);
        });

        // cns * cns
        let cns_cns = (cns.clone() * cns.clone()).unwrap();
        assert_eq!(cns_cns_exp, cns_cns);
        // cns * lin & lin * cns
        let cns_lin = (cns.clone() * lin.clone()).unwrap();
        let lin_cns = (lin.clone() * cns.clone()).unwrap();
        assert_eq!(cns_lin_exp, cns_lin);
        assert_eq!(cns_lin_exp, lin_cns);
        // cns * qud & qud * cns
        let cns_qud = (cns.clone() * qud.clone()).unwrap();
        let qud_cns = (qud.clone() * cns.clone()).unwrap();
        assert_eq!(cns_qud_exp, cns_qud);
        assert_eq!(cns_qud_exp, qud_cns);
        // cns * hio & hio * cns
        let cns_hio = (cns.clone() * hio.clone()).unwrap();
        let hio_cns = (hio.clone() * cns.clone()).unwrap();
        assert_eq!(cns_hio_exp, cns_hio);
        assert_eq!(cns_hio_exp, hio_cns);

        // lin * lin
        let lin_lin = (lin.clone() * lin.clone()).unwrap();
        assert_eq!(lin_lin_exp, lin_lin);
        // lin * qud & qud * lin
        let lin_qud = (lin.clone() * qud.clone()).unwrap();
        let qud_lin = (qud.clone() * lin.clone()).unwrap();
        assert_eq!(lin_qud_exp, lin_qud);
        assert_eq!(lin_qud_exp, qud_lin);
        // lin * hio && hio * lin
        let lin_hio = (lin.clone() * hio.clone()).unwrap();
        let hio_lin = (hio.clone() * lin.clone()).unwrap();
        assert_eq!(lin_hio_exp, lin_hio);
        assert_eq!(lin_hio_exp, hio_lin);

        // qud * qud
        let qud_qud = (qud.clone() * qud.clone()).unwrap();
        assert_eq!(qud_qud_exp, qud_qud);
        // qud * hio & hio * qud
        let qud_hio = (qud.clone() * hio.clone()).unwrap();
        let hio_qud = (hio.clone() * qud.clone()).unwrap();
        dbg!(&qud_hio_exp, &hio_qud);
        assert_eq!(qud_hio_exp, qud_hio);
        assert_eq!(qud_hio_exp, hio_qud);

        // hio * hio
        let hio_hio = (hio.clone() * hio.clone()).unwrap();
        assert_eq!(hio_hio_exp, hio_hio);
    }

    #[test]
    fn mul_assign_expr_to_expr() {
        let mut env = ArcEnv::default();
        let v: VarRef = env.insert("b", Vtype::Binary, None).unwrap();
        let e = Expression::empty(env);
        dbg!(&v, &e);
        e.clone().mul_assign(&v).unwrap();
        e.clone().mul_assign(v).unwrap();
    }
}

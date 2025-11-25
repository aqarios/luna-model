use crate::{
    Expression,
    ops::utils::check_envs,
    prelude::{Linear, Quadratic, VarRef},
    traits::DefaultEditable,
};
use lunamodel_error::LunaModelResult;
use lunamodel_types::{Bias, VarIdx, Vtype::*};
use std::ops::Mul;

impl Mul<Bias> for VarRef {
    type Output = Linear;
    fn mul(self, rhs: Bias) -> Self::Output {
        (&self).mul(rhs)
    }
}

impl Mul<Bias> for &VarRef {
    type Output = Linear;
    fn mul(self, rhs: Bias) -> Self::Output {
        Linear::with(|l| *l += (self.id, rhs))
    }
}

pub enum QuadResType {
    Const(Bias),
    Lin((VarIdx, Bias)),
    Quad((VarIdx, VarIdx, Bias)),
}

impl Into<Expression> for QuadResType {
    fn into(self) -> Expression {
        use QuadResType::*;
        match self {
            Const(cnst) => cnst.into(),
            Lin(lin) => Linear::with(|l| l[lin.0] = lin.1).into(),
            Quad(quad) => Quadratic::with(|q| q[(quad.0, quad.1)] = quad.2).into(),
        }
    }
}

impl Mul<&VarRef> for &VarRef {
    type Output = LunaModelResult<QuadResType>;

    fn mul(self, rhs: &VarRef) -> Self::Output {
        use QuadResType::*;
        check_envs(self, rhs)?;
        let env = self.env.read_arc();
        Ok(match (self.id == rhs.id, env[rhs.id].vtype) {
            // -1*-1 == +1*+1 == 1 so this is constant offset
            (true, Spin) => Const(1.0),
            // 1*1 == 1 and 0*0 == 0 so this is linear
            (true, Binary | InvertedBinary) => Lin((self.id, 1.0)),
            _ => Quad((self.id, rhs.id, 1.0)),
        })
    }
}

impl Mul<&VarRef> for &Linear {
    type Output = LunaModelResult<(QuadResType)>;

    fn mul(self, rhs: &VarRef) -> Self::Output {
        _ = rhs;
        unimplemented!()
    }
}

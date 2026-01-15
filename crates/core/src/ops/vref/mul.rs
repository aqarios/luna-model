use std::ops::Mul;

use crate::{
    Expression,
    ops::{
        LmAddAssign, LmMulAssign,
        traits::internal::PrvMul,
        utils::{
            VarMulRes::{self, *},
            check_envs, reduce_vars_mul,
        },
    },
    prelude::{Linear, VarRef},
    traits::{DefaultEditable, Editable},
};
use lunamodel_error::LunaModelResult;
use lunamodel_types::{Bias, VarIdx, Vtype::*};

impl Mul<Self> for &VarRef {
    type Output = LunaModelResult<Expression>;
    fn mul(self, rhs: Self) -> Self::Output {
        check_envs(self, rhs)?;
        self.check_living()?;
        rhs.check_living()?;
        let mut out = Expression::empty(self.env.clone()).maybe_edit(|e| e.add_assign(self))?;
        out.mul_assign(rhs)?;
        Ok(out)
    }
}

impl Mul<Bias> for &VarRef {
    type Output = LunaModelResult<Expression>;
    fn mul(self, rhs: Bias) -> Self::Output {
        self.check_living()?;
        let mut out = Expression::empty(self.env.clone()).maybe_edit(|e| e.add_assign(self))?;
        out.mul_assign(rhs)?;
        Ok(out)
    }
}

impl Mul<usize> for &VarRef {
    type Output = LunaModelResult<Expression>;
    fn mul(self, rhs: usize) -> Self::Output {
        self.mul(rhs as Bias)
    }
}

impl PrvMul<Bias> for VarRef {
    type Output = Linear;
    fn m(self, rhs: Bias) -> Self::Output {
        (&self).m(rhs)
    }
}

impl PrvMul<Bias> for &VarRef {
    type Output = Linear;
    fn m(self, rhs: Bias) -> Self::Output {
        Linear::with(|l| *l += (self.id, rhs))
    }
}

impl PrvMul<&VarRef> for Bias {
    type Output = Linear;
    fn m(self, rhs: &VarRef) -> Self::Output {
        rhs.m(self)
    }
}

impl PrvMul<(&VarRef, Bias)> for &VarRef {
    type Output = VarMulRes;

    fn m(self, rhs: (&VarRef, Bias)) -> Self::Output {
        self.m((rhs.0.id, rhs.1))
    }
}

impl PrvMul<&VarRef> for &VarRef {
    type Output = VarMulRes;

    fn m(self, rhs: &VarRef) -> Self::Output {
        self.m((rhs, 1.0))
    }
}

impl PrvMul<(VarIdx, Bias)> for VarRef {
    type Output = VarMulRes;

    fn m(self, rhs: (VarIdx, Bias)) -> Self::Output {
        (&self).m(rhs)
    }
}

impl PrvMul<(VarIdx, Bias)> for &VarRef {
    type Output = VarMulRes;

    fn m(self, rhs: (VarIdx, Bias)) -> Self::Output {
        let env = self.env.read_arc();
        match (self.id == rhs.0, env[rhs.0].vtype) {
            // -1*-1 == +1*+1 == 1 so this is constant offset
            (true, Spin) => Const(rhs.1),
            // 1*1 == 1 and 0*0 == 0 so this is linear
            (true, Binary | InvertedBinary) => Lin((self.id, rhs.1)),
            // binary * its inverted.
            (false, Binary | InvertedBinary) => {
                // does self have an inverted? and is the inverted the rhs?
                if let Some(inverted) = env[self.id].inverted
                    && inverted == rhs.0
                {
                    // it does! and it is!
                    // This multiplication is zero.
                    Const(0.0)
                } else {
                    // nope, just two non-equal, non-related variables.
                    Quad((self.id, rhs.0, rhs.1))
                }
            }
            _ => Quad((self.id, rhs.0, rhs.1)),
        }
    }
}

impl PrvMul<(VarIdx, VarIdx, Bias)> for &VarRef {
    type Output = VarMulRes;

    fn m(self, rhs: (VarIdx, VarIdx, Bias)) -> Self::Output {
        let (u, v, bias) = rhs;
        self.m((vec![u, v], bias))
    }
}

impl PrvMul<(Vec<u32>, Bias)> for &VarRef {
    type Output = VarMulRes;

    fn m(self, rhs: (Vec<u32>, Bias)) -> Self::Output {
        let VarRef { id, env } = self;
        let env = env.read_arc();
        let (mut contrib, bias) = rhs;
        contrib.push(*id);
        if let Some(vars) = reduce_vars_mul(&contrib, |v| env[v].vtype, |v| env[v].inverted) {
            (vars, bias).into()
        } else {
            (Vec::default(), Bias::default()).into()
        }
    }
}

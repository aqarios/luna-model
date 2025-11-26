use crate::{
    ops::utils::{
        VarMulRes::{self, *},
        check_envs, reduce_vars_mul,
    },
    prelude::{Linear, VarRef},
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

impl Mul<&VarRef> for Bias {
    type Output = Linear;
    fn mul(self, rhs: &VarRef) -> Self::Output {
        rhs.mul(self)
    }
}

impl Mul<(&VarRef, Bias)> for &VarRef {
    type Output = LunaModelResult<VarMulRes>;

    fn mul(self, rhs: (&VarRef, Bias)) -> Self::Output {
        check_envs(self, rhs.0)?;
        self.mul((rhs.0.id, rhs.1))
    }
}

impl Mul<&VarRef> for &VarRef {
    type Output = LunaModelResult<VarMulRes>;

    fn mul(self, rhs: &VarRef) -> Self::Output {
        self.mul((rhs, 1.0))
    }
}

impl Mul<(VarIdx, Bias)> for &VarRef {
    type Output = LunaModelResult<VarMulRes>;

    fn mul(self, rhs: (VarIdx, Bias)) -> Self::Output {
        let env = self.env.read_arc();
        Ok(match (self.id == rhs.0, env[rhs.0].vtype) {
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
        })
    }
}

impl Mul<(VarIdx, VarIdx, Bias)> for &VarRef {
    type Output = LunaModelResult<VarMulRes>;

    fn mul(self, rhs: (VarIdx, VarIdx, Bias)) -> Self::Output {
        let (u, v, bias) = rhs;
        self.mul((vec![u, v], bias))
    }
}

impl Mul<(Vec<u32>, Bias)> for &VarRef {
    type Output = LunaModelResult<VarMulRes>;

    fn mul(self, rhs: (Vec<u32>, Bias)) -> Self::Output {
        let VarRef { id, env } = self;
        let env = env.read_arc();
        let (mut contrib, bias) = rhs;
        contrib.push(*id);
        let vars = reduce_vars_mul(&contrib, |v| env[v].vtype, |v| env[v].inverted);
        Ok((vars, bias).into())
    }
}

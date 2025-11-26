use std::ops::Mul;

use lunamodel_error::LunaModelResult;

use crate::{
    ops::utils::VarMulRes,
    prelude::{Quadratic, VarRef},
};

impl Mul<&VarRef> for &Quadratic {
    type Output = LunaModelResult<Vec<VarMulRes>>;

    fn mul(self, rhs: &VarRef) -> Self::Output {
        self.iter_flat().map(|v| rhs * v).collect()
    }
}

impl Mul<&VarRef> for &Option<Quadratic> {
    type Output = LunaModelResult<Option<Vec<VarMulRes>>>;

    fn mul(self, rhs: &VarRef) -> Self::Output {
        self.as_ref().map(|q| q * rhs).transpose()
    }
}

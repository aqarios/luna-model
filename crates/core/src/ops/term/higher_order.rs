use std::ops::Mul;

use lunamodel_error::LunaModelResult;

use crate::{
    ops::utils::VarMulRes,
    prelude::{HigherOrder, VarRef},
};

impl Mul<&VarRef> for &HigherOrder {
    type Output = LunaModelResult<Vec<VarMulRes>>;

    fn mul(self, rhs: &VarRef) -> Self::Output {
        self.iter_contrib().map(|v| rhs * v).collect()
    }
}

impl Mul<&VarRef> for &Option<HigherOrder> {
    type Output = LunaModelResult<Option<Vec<VarMulRes>>>;

    fn mul(self, rhs: &VarRef) -> Self::Output {
        self.as_ref().map(|h| h * rhs).transpose()
    }
}

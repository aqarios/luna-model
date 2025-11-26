use std::ops::Mul;

use lunamodel_error::LunaModelResult;

use crate::{
    ops::utils::VarMulRes,
    prelude::{Linear, VarRef},
};

impl Mul<&VarRef> for &Linear {
    /// I'd like to change the return type to this at some point in the
    /// future but that's currently unstable. So we have to wait a bit.
    /// Or we allow it explicitly...
    ///
    ///   type Output = LunaModelResult<impl Iterator<Item = VarMulRes>>;
    ///   `impl Trait` in associated types is unstable
    ///   see issue #63063 <https://github.com/rust-lang/rust/issues/63063>
    ///   for more information [E0658]
    type Output = LunaModelResult<Vec<VarMulRes>>;

    fn mul(self, rhs: &VarRef) -> Self::Output {
        self.iter().map(|v| rhs * v).collect()
    }
}

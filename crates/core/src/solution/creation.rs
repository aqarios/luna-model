use lunamodel_error::{LunaModelError, LunaModelResult};
use lunamodel_types::Vtype;

use crate::ArcEnv;

use super::Solution;

impl Solution {}

impl TryFrom<ArcEnv> for Solution {
    type Error = LunaModelError;

    fn try_from(env: ArcEnv) -> LunaModelResult<Self> {
        let mut slf = Self::default();
        for v in env.vars() {
            match v.vtype()? {
                Vtype::Binary => slf.add_binary(v.name()?),
                Vtype::Spin => slf.add_spin(v.name()?),
                Vtype::Integer => slf.add_integer(v.name()?),
                Vtype::Real => slf.add_real(v.name()?),
                Vtype::InvertedBinary => (),
            }
        }
        Ok(slf)
    }
}

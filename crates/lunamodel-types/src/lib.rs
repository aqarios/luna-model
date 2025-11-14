mod bound;
mod dtypes;
mod res;
mod varid;
mod varname;
mod vtype;

use std::sync::LazyLock;

pub use bound::Bound;
pub use dtypes::{
    Bias, BinaryAssignment, EnvIdx, IntegerAssignment, RealAssignment, SpinAssignment, VarIdx,
};
pub use varid::VarId;
pub use varname::VarName;
pub use vtype::Vtype;

pub static DEFAULT_BIAS: LazyLock<Bias> = LazyLock::new(|| Bias::default());

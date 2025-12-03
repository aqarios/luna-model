mod bound;
mod cmp;
mod dtypes;
mod sense;
mod specs;
mod utils;
mod varid;
mod varname;
mod vtype;

use std::sync::LazyLock;

pub use bound::Bound;
pub use cmp::Comparator;
pub use dtypes::{
    Bias, BinaryAssignment, EnvIdx, IntegerAssignment, RealAssignment, SpinAssignment, VarIdx,
};
pub use sense::Sense;
pub use specs::{Ctype, Specs};
pub use varid::VarId;
pub use varname::VarName;
pub use vtype::Vtype;

pub static DEFAULT_BIAS: LazyLock<Bias> = LazyLock::new(|| Bias::default());

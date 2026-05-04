//! Shared primitive and enum types used across the LunaModel workspace.
//!
//! This crate stays intentionally lightweight. It contains the data vocabulary
//! that many higher-level crates need to agree on: variable ids and names,
//! optimization sense, variable types, constraint comparators, bounds, and
//! model capability specifications. Keeping those definitions here avoids
//! dependency cycles between the modeling, translation, hashing, and Python
//! layers.
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
pub use utils::EnumSetFromVec;
pub use varid::VarId;
pub use varname::VarName;
pub use vtype::Vtype;

/// Canonical zero bias used where a stable shared default is convenient.
pub static DEFAULT_BIAS: LazyLock<Bias> = LazyLock::new(Bias::default);

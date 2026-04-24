//! Strongly typed wrappers for internal variable identifiers.

use crate::dtypes::VarIdx;
use derive_more::{Deref, DerefMut};

/// Newtype wrapper around a variable index.
///
/// This exists mainly for places that want a distinct semantic type without
/// paying any runtime cost.
#[derive(Debug, Clone, Copy, Deref, DerefMut, Hash, PartialEq, Eq)]
pub struct VarId(pub VarIdx);

// impl AddAssign<VarId> for VarId {
//     fn add_assign(&mut self, rhs: VarId) {
//         self.0 += rhs.0
//     }
// }
//
// impl SubAssign<VarId> for VarId {
//     fn sub_assign(&mut self, rhs: VarId) {
//         self.0 -= rhs.0
//     }
// }
//
// impl ToString for VarId {
//     fn to_string(&self) -> String {
//         self.0.to_string()
//     }
// }

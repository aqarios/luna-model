use crate::dtypes::VarIdx;
use derive_more::{Deref, DerefMut};

#[derive(Debug, Clone, Copy, Deref, DerefMut)]
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

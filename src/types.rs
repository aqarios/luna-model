use uuid::Uuid;

use crate::core::VarId;

pub type Id = u32;
pub type EnvId = Uuid;
pub type VarIndex = VarId;
pub type Bias = f64;

pub type BinaryAssignmentType = u8;
pub type SpinAssignmentType = i8;
pub type IntegerAssignmentType = i64;
pub type RealAssignmentType = f64;

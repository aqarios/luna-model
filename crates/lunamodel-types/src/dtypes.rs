/// Variable index type.
pub type VarIdx = u32;
/// Environment index type.
pub type EnvIdx = u64; // has to be u64 since counter has to be u64 :(
/// Bias type.
pub type Bias = f64;
/// Type used for binary variable assignments.
pub type BinaryAssignment = u8;
/// Type used for spin variable assignments.
pub type SpinAssignment = i8;
/// Type used for integer variable assignments.
pub type IntegerAssignment = i64;
/// Type used for assignments for variables with [crate::Vtype::Real].
pub type RealAssignment = f64;

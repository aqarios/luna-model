mod addition;
pub mod base;
mod equality;
mod errors;
mod evaluation;
mod expr;
mod multiplication;
mod negative;
mod separation;
mod substitution;

pub use base::BiasConstraints;
pub use base::ExpressionBase;
pub use base::ExpressionBaseAdd;
pub use base::ExpressionBaseAdjustment;
pub use base::ExpressionBaseCreation;
pub use base::ExpressionEvaluation;
pub use base::IndexConstraints;
pub use base::One;

pub use errors::EnvMismatchError;
pub use errors::VariableOutOfRangeErr;

pub use expr::Expression;

pub use separation::Separation;
pub use substitution::Substitution;

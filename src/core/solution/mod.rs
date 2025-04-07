mod base;
mod res;
mod timing;

pub mod sol;

pub use base::AssignmentBaseTypes;
pub use res::OwnedResult;
pub use res::ResultIterator;
pub use res::ResultView;
pub use res::Sample;
pub use res::SampleIterator;
pub use res::Samples;
pub use res::SamplesIterator;
pub use sol::RcSolution;
pub use sol::Solution;
pub use sol::VarAssignment;
pub use timing::Timer;
pub use timing::Timing;

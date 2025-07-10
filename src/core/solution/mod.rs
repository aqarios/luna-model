mod res;
mod timing;

mod iterators;
mod sample;
pub mod sol;
mod sol_print;

pub use iterators::ResultIterator;
pub use iterators::SampleIterator;
pub use iterators::SamplesIterator;
pub use res::OwnedResult;
pub use res::ResultView;
pub use sample::OwnedSample;
pub use sample::Sample;
pub use sample::Samples;
pub use sol::PrintLayout;
pub use sol::SharedSolution;
pub use sol::Solution;
pub use sol::VarAssignment;
pub use timing::Timer;
pub use timing::Timing;

mod timing;

// mod sol_print;

pub mod sol;
pub mod result;
pub mod sample;

// mod sample_old;
// pub mod sol_old;
// mod res;
// mod iterators;

// pub use iterators::ResultIterator;
// pub use iterators::SampleIterator;
// pub use iterators::SamplesIterator;
// pub use res::OwnedResult;
// pub use res::ResultView;
// pub use sample::OwnedSample;
// pub use sample::Sample;
// pub use sample::Samples;
// pub use sol::PrintLayout;
// pub use sol::SharedSolution;
// pub use sol::Solution;
pub use sample::VarAssignment;
pub use timing::Timer;
pub use timing::Timing;
pub use sol::Solution;
pub use sol::Column;
pub use sol::ColElement;
pub use sol::ShowMetadata;
pub use sol::PrintLayout;
pub use sol::VarKey;
pub use result::ResultView;
pub use sample::Sample;
pub use sample::Samples;
pub use sample::SampleIterator;
pub use sample::SamplesIterator;

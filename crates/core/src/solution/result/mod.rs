//! Result-oriented views over solution rows.
//!
//! [`ResultView`] extends [`crate::solution::sample::SampleView`] with the
//! metadata that is commonly needed when treating a row as a solver result:
//! objective values, energies, counts, feasibility, and constraint summaries.

mod view;

/// Row view with result-oriented metadata accessors.
pub use view::ResultView;

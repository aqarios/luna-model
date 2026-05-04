//! Sample-oriented views over solution rows.
//!
//! The sample view types expose only the variable assignments for a row. They
//! are intentionally smaller in scope than [`crate::solution::result::ResultView`]
//! and are useful when callers want to work with assignments without assuming
//! evaluated objective or feasibility metadata is present.

mod view;

/// Borrowed view over one solution row.
pub use view::SampleView;
/// Supported lookup selectors for [`SampleView::try_get`].
pub use view::SampleViewIdx;

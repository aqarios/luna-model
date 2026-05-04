//! Python wrappers for result-level solution views.

mod iter;
mod view;

pub use iter::PyResultIterator;
pub use view::PyResultView;
